mod configuration;
mod enttec_devices;
mod faders;
mod midi_ports;
mod shows;

use configuration::BaseConfig;
use enttec_devices::Dmxis;
use shows::Show;
use midi_ports::MidiPort;

use core::time::Duration;
use std::thread::sleep;
use std::time::Instant;
use log::{info, warn, error};

fn main() -> Result<(), ::std::io::Error> {

    println!("Rustylight");
    println!("");
    println!("Starting, hang on...");

    let config_result = configuration::load();
    if config_result.is_err() {
        return Ok(());
    }

    let config = config_result.unwrap();
    let show = shows::load_show(&config);
    let dmx_port = enttec_devices::open_dmxis_port(&config);
    let midi_port = midi_ports::new(&config);
    info!("");

    if show.is_err() || dmx_port.is_err() || !midi_port.is_some() {
        error!("Destroying the application. See logs for further details.");
        error!("Bye!");
        error!("");
        return Ok(());
    }

    start_game_loop(&config, show.unwrap(), dmx_port.unwrap(), midi_port.unwrap());

    return Ok(());
}

fn start_game_loop(config: &BaseConfig, mut show: Show, mut dmx_port: Dmxis, midi_port: MidiPort) {
    let frame_duration = 1000/config.fps;
    let mut sleep_duration;
    show.print_content();
    info!("");
    info!("Here we go!");
    info!("");
    loop {
        let loop_start_time = Instant::now();

        // Read all inputs
        let update = midi_port.get_update();

        // Update internal state
        show.update(update);
        show.update_state();

        // Render internal state to DMX
        let dmx_data = show.get_dmx_data();
        dmx_port.write(&dmx_data);

        // Fill remaining frame with idle time
        let elapsed = loop_start_time.elapsed().as_millis() as u64;
        if elapsed < frame_duration {
            sleep_duration = frame_duration - elapsed;
        } else {
            warn!("Dropped {} frame(s).", (elapsed - (elapsed % frame_duration)) / frame_duration);
            sleep_duration = elapsed % frame_duration;
        }
        sleep(Duration::from_millis(sleep_duration));
    }
}
