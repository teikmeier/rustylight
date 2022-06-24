use derive_more::Display;
use serialport::Error as SerialError;
use std::error::Error as StdError;
use serialport::{ new, SerialPort };
use core::time::Duration;
use std::cmp::{ min };
use crate::configuration::BaseConfig;

const SET_PARAMETERS_COMMAND: u8 = 4;
const SEND_PACKET_COMMAND: u8 = 6;

const START_VAL: u8 = 0x7E;
const END_VAL: u8 = 0xE7;

const MIN_FRAME_SIZE: usize = 24;
const MAX_FRAME_SIZE: usize = 512;


#[derive(Debug, Display)]
pub enum Error {
    Serial(SerialError),
    IO(std::io::Error),
    PortClosed,
}

impl From<SerialError> for Error {
    fn from(e: SerialError) -> Self {
        Error::Serial(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;
        match *self {
            Serial(ref e) => Some(e),
            IO(ref e) => Some(e),
            PortClosed => None,
        }
    }
}

pub struct Dmxis {
    port: Option<Box<dyn SerialPort>>,
    port_name: String,
    break_time: u8,
    mark_after_break_time: u8,
    output_rate: u8
}

impl Dmxis {
    pub fn new(port_name_input: &str) -> Dmxis {
        return Dmxis{
            port: None,
            port_name: String::from(port_name_input),
            break_time: 9, //DMX protocol defines a break to indicate the beginnging of a packet
            mark_after_break_time: 1, //DMX protocol defines a mark after break to indicate the beginnging of a packet
            output_rate: 40 //fps
        };
    }

    pub fn open(&mut self) -> Result<(), Error> {
        if self.port.is_some() {
            return Ok(());
        }

        let port = new(&self.port_name, 57600)
            .timeout(Duration::from_millis(1))
            .open()?;

        self.port = Some(port);

        // send the default parameters to the port
        if let Err(e) = self.set_dmx_params() {
            self.port = None;
            return Err(e);
        }
        Ok(())
    }

    fn set_dmx_params(&mut self) -> Result<(), Error> {
        let packet = [
            0,
            0,
            self.break_time,
            self.mark_after_break_time,
            self.output_rate
        ];
        self.write_packet(SET_PARAMETERS_COMMAND, &packet, false)
    }

    fn write_packet(
        &mut self,
        command: u8,
        paket: &[u8],
        add_pad_byte: bool
    ) -> Result<(), Error> {
        let port = self.port.as_mut().ok_or(Error::PortClosed)?;
        // Enttec messages are the size of the payload plus 5 bytes for type, length, and framing.
        let paket_size = paket.len() + add_pad_byte as usize;
        let (len_lsb, len_msb) = (paket_size as u8, (paket_size >> 8) as u8);
        let header = [START_VAL, command, len_lsb, len_msb];
        port.write_all(&header)?;
        if add_pad_byte {
            port.write_all(&[0][..])?;
        }
        port.write_all(paket)?;
        port.write_all(&[END_VAL][..])?;
        Ok(())
    }

    pub fn write(&mut self, frame: &[u8]) {
        let input_size = frame.len();
        let capacity = match input_size {
            0..=MIN_FRAME_SIZE => MIN_FRAME_SIZE,
            MIN_FRAME_SIZE..=MAX_FRAME_SIZE => input_size,
            _ => {
                println!("Frame data too large, cutting off excess data");
                MAX_FRAME_SIZE
            }
        };
        let mut padded_frame = Vec::with_capacity(capacity);
        padded_frame.extend_from_slice(&frame[0..min(input_size, capacity)]);
        padded_frame.resize(capacity, 0);
        let written = self.write_packet(SEND_PACKET_COMMAND, &padded_frame, true);
        match written {
            Ok(()) => (),
            Err(error) => println!("Frame was not successfully written to DMXIS: {:?}", error)
        }
    }

    // pub fn lights_off(&mut self) {
    //     let empty_frame = vec!(0);
    //     self.write(&empty_frame)
    // }
    
    // pub fn close(&mut self) -> Result<(), Error> {
    //     self.lights_off();
    //     self.port = None;
    //     Ok(())
    // }
}

pub fn open_dmxis_port(config: &BaseConfig) -> Result<Dmxis, Box<dyn StdError>> {
    let serial_port = if cfg!(windows) {
        &config.dmx_serial_port_win
    } else if cfg!(macos) {
        &config.dmx_serial_port_osx
    } else {
        &config.dmx_serial_port_other
    };
    let mut dmxis = Dmxis::new(serial_port);
    let opened = dmxis.open();
    if opened.is_ok() {
        println!("Opened DMX serial port:  {}", serial_port);
        return Ok(dmxis);
    }
    println!("");
    println!("!!  No dmx port to open, check your config and that the dmx interface is properly connected.  !!");
    println!("");
    return Err("".into());
}