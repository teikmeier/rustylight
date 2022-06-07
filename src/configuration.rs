extern crate confy;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    version: u8,
    pub dmx_serial_port_win: String,
    pub dmx_serial_port_osx: String,
    pub dmx_serial_port_other: String,
    pub show_path: String,
    pub midi_channel: u8,
    pub midi_port: String,
    pub fps: u64,
}

impl Default for BaseConfig {
    fn default() -> Self {
        BaseConfig {
            version: 0,
            dmx_serial_port_win: String::from("COM-1"),
            dmx_serial_port_osx: String::from("/dev/tty.usbserial-ENVVVC0F"),
            dmx_serial_port_other: String::from("/dev/ttyUSB0"),
            show_path: String::from("default_show"),
            midi_channel: 1,
            midi_port: String::from("M-Audio MIDISPORT Uno"),
            fps: 20,
        }
    }
}

pub fn load() -> Result<BaseConfig, confy::ConfyError> {
    let config: BaseConfig = confy::load("rusty-light")?;
    println!("Config loading:          Done");
    Ok(config)
}
