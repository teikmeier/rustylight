use midir::{MidiInput, MidiInputConnection};
use crate::configuration::BaseConfig;
use crate::shows::ShowUpdate;
use std::error::Error;
use crossbeam_channel::{unbounded, Receiver};
use log::{info, trace, error};

pub struct MidiMessage {
    status: u8,
    data1: u8,
    data2: Option<u8>,
}

pub struct MidiPort {
    midi_channel: u8,
    midi_port: String,
    connection: Option<MidiInputConnection<()>>,
    receiver: Option<Receiver<MidiMessage>>
}

const PROGRAMM_CHANGE: u8 = 192; // programm changes have a status range from 192-207
const CONTROL_CHANGE: u8 = 176; // control changes have a status range from 176-191
const NOTE_ON: u8 = 144; // note on events have a status range from 144-159
const NOTE_OFF: u8 = 128; // note off events have a status range from 128-143
const SONG_SELECT: u8 = 0;
const ALL_NOTES_OFF: u8 = 123;
const TEMPO_CONTROL_1: u8 = 12;
const TEMPO_CONTROL_2: u8 = 13;

impl MidiPort {
    pub fn connect (&mut self) -> Result<(), Box<dyn Error>> {
        let midi_in = MidiInput::new("midir reading input")?;
        let ports = midi_in.ports();
        let port_result = ports.iter().find(|p| midi_in.port_name(p).unwrap().contains(&self.midi_port));
        let port;

        if port_result.is_none() {
            error!("");
            error!("!!  Couldn't find {} in available midi ports.  !!", self.midi_port);
            error!("    Available midi input ports are:");
            for p in ports.iter() {
                error!("    - {}", midi_in.port_name(&p)?);
            }
            error!("");
            return Err("".into());
        }

        port = port_result.unwrap();
        info!("Connected midi port:     {}", midi_in.port_name(port)?);
        let (sender, receiver) = unbounded();
        self.receiver = Some(receiver);

        let connection = midi_in.connect(&port, "midir-read-input", move |_stamp, message, _| {
            let parsed_message = parse_midi_message(message);
            if let Some(payload) = parsed_message {
                trace!("MIDI Message: s {} - d1 {} - d2 {:?}", payload.status, payload.data1, payload.data2);
                match sender.try_send(payload) {
                    Ok(()) => (),
                    Err(err) => error!("{}", err),
                };
            }
        }, ());
        self.connection = connection.ok();

        return Ok(());
    }

    pub fn read_all(&self) -> ShowUpdate {
        let mut update = ShowUpdate {
            song: None,
            scene: None,
            tempo: None,
            off: None,
            notes: [None; 128],
        };
        if let Some(receiver) = &self.receiver {
            let mut tempo1 = None;
            let mut tempo2 = None;
            loop {
                match receiver.try_recv() {
                    Ok(message) => {
                        if message.status == PROGRAMM_CHANGE + &self.midi_channel {
                            update.scene = Some(message.data1 as usize);
                        } else if message.status == CONTROL_CHANGE + &self.midi_channel && message.data1 == SONG_SELECT && message.data2.is_some() {
                            update.song = Some(message.data2.unwrap() as usize);
                        } else if message.status == CONTROL_CHANGE + &self.midi_channel && message.data1 == TEMPO_CONTROL_1 && message.data2.is_some() {
                            tempo1 = message.data2;
                        } else if message.status == CONTROL_CHANGE + &self.midi_channel && message.data1 == TEMPO_CONTROL_2 && message.data2.is_some() {
                            tempo2 = message.data2;
                        } else if message.status == CONTROL_CHANGE + &self.midi_channel && message.data1 == ALL_NOTES_OFF {
                            update.off = Some(true);
                        } else if message.status == NOTE_ON + &self.midi_channel && message.data2.is_some() {
                            update.notes[message.data1 as usize] = message.data2;
                        } else if message.status == NOTE_OFF + &self.midi_channel {
                            update.notes[message.data1 as usize] = Some(0);
                        }
                    },
                    Err(_) => break,
                }
            }
            if tempo1.is_some() && tempo2.is_some() {
                update.tempo = Some(tempo1.unwrap() + tempo2.unwrap());
            }
        }
        update
    }
}

pub fn new (config: &BaseConfig) -> Option<MidiPort> {
    let mut port = MidiPort {
        midi_channel: config.midi_channel - 1, // to ease the calculation of midi messages later on
        midi_port: config.midi_port.clone(),
        connection: None,
        receiver: None,
    };
    if port.connect().is_ok() {
        return Some(port);
    };
    None
}

fn parse_midi_message(midi_message: &[u8]) -> Option<MidiMessage> {
    let parsed_midi_message = midi_message.to_vec();
    if parsed_midi_message.len() >=2 {
        let mut result = MidiMessage {
            status: parsed_midi_message[0],
            data1: parsed_midi_message[1],
            data2: None,
        };
        if parsed_midi_message.len() >= 3 {
            result.data2 = Some(parsed_midi_message[2])
        }
        return Some(result);
    }
    None
}
