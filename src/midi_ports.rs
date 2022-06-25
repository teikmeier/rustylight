use midir::{MidiInput, MidiInputConnection};
use crate::configuration::BaseConfig;
use std::error::Error;
use std::net::UdpSocket;

pub struct MidiPort {
    midi_channel: u8,
    connection: Option<MidiInputConnection<()>>,
}

impl MidiPort {
    pub fn is_open (&mut self) -> bool {
        self.connection.is_some()
    }

    pub fn connect (&mut self, config: &BaseConfig) -> Result<(), Box<dyn Error>> {
        if self.connection.is_some() {
            return Ok(());
        }
        let midi_in = MidiInput::new("midir reading input")?;
        let ports = midi_in.ports();
        let port_result = ports.iter().find(|p| midi_in.port_name(p).unwrap().contains(&config.midi_port));
        let port;
        if port_result.is_none() {
            println!("");
            println!("!!  Couldn't find {} in available midi ports.  !!", config.midi_port);
            println!("    Available midi input ports are:");
            for p in ports.iter() {
                println!("    - {}", midi_in.port_name(&p)?);
            }
            println!("");
            return Err("couldn't find wanted midi port in available ports".into());
        }
        port = port_result.unwrap();
        println!("Connected midi port:     {}", midi_in.port_name(port)?);
        let socket = UdpSocket::bind("127.0.0.1:32567").expect("couldn't bind to address");
        socket.connect("127.0.0.1:32568").expect("connect function failed");
        let midi_channel = self.midi_channel;
        let connection = midi_in.connect(&port, "midir-read-input", move |_stamp, message, _| {
            let _ = send_udp_message(&socket, message, midi_channel);
        }, ());
        self.connection = connection.ok();
        return Ok(());
    }
}

fn send_udp_message(socket: &UdpSocket, midi_message: &[u8], midi_channel: u8) -> std::io::Result<()> {
    let programm_change: u8 = 191 + midi_channel; // midi programm changes have a status range from 192-207
    let control_change: u8 = 175 + midi_channel; // midi control changes have a status range from 176-191
    const BANK_SELECT: u8 = 0; // bank select sounded most apropriate to map song or scene selection to
    const ALL_NOTES_OFF: u8 = 123;

    let parsed_vec_message = midi_message.to_vec();
    let status = parsed_vec_message[0];
    let data1 = parsed_vec_message[1];
    let data2 = parsed_vec_message.get(2);
    let mut string_message = String::from("");

    if status == programm_change {
        string_message = format!("scene{}", data1);
    } else if status == control_change && data1 == BANK_SELECT && data2.is_some() {
        string_message = format!("song{}", data2.unwrap());
    } else if status == control_change && data1 == ALL_NOTES_OFF {
        string_message = format!("off");
    }

    if !string_message.is_empty() {
        socket
        .send(string_message.as_bytes())
        .expect("Error on send");
    }
    Ok(())
}

pub fn new (config: &BaseConfig) -> MidiPort {
    MidiPort {
        midi_channel: config.midi_channel,
        connection: None,
    }
}
