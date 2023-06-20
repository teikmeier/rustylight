extern crate confy;
use serde::{Serialize, Deserialize};
use log::{info};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use log4rs::encode::pattern::PatternEncoder;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    version: u8,
    pub dmx_serial_port_win: String,
    pub dmx_serial_port_osx: String,
    pub dmx_serial_port_other: String,
    pub show_path: String,
    pub midi_channel: u8,
    pub midi_port: String,
    pub midi_faders: bool,
    pub fps: u64,
    pub log_level: String,
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
            midi_faders: false,
            fps: 20,
            log_level: String::from("info"),
        }
    }
}

pub fn load() -> Result<BaseConfig, confy::ConfyError> {
    let config = confy::load("rusty-light", None);
    let config_path = confy::get_configuration_file_path("rusty-light", None);
    println!("Config location:         {}", config_path?.into_os_string().into_string().unwrap());
    if config.is_err() {
        println!("Couldn't load config. Make sure your config file contains all required fields.");
        config
    } else {
        let config_result = config.unwrap();
        set_up_logging(&config_result);
        info!("Config loaded:           Done");
        Ok(config_result)
    }
}

fn set_up_logging(config: &BaseConfig) {
    let log_level = match config.log_level.as_str() {
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        _ => {
            println!("Log level '{}' didn't match anything so using 'warn'", config.log_level);
            LevelFilter::Warn
        }
    };
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log_level))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
}
