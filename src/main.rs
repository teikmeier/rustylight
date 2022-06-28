mod configuration;
mod enttec_devices;
mod faders;
mod midi_ports;
mod shows;
mod udp_sockets;

use configuration::BaseConfig;
use enttec_devices::Dmxis;
use shows::Show;

use core::time::Duration;
use std::thread::sleep;
use std::time::Instant;
use std::net::UdpSocket;
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
    let mut midi_port = midi_ports::new(&config);
    let _ = midi_port.connect(&config);
    let udp_socket = udp_sockets::open_udp_socket();
    info!("");

    if show.is_err() || dmx_port.is_err() || !midi_port.is_open() || udp_socket.is_err() {
        error!("Destroying the application. See logs for further details.");
        error!("Bye!");
        error!("");
        return Ok(());
    }

    start_game_loop(&config, show.unwrap(), dmx_port.unwrap(), udp_socket.unwrap());

    return Ok(());
}

fn start_game_loop(config: &BaseConfig, mut show: Show, mut dmx_port: Dmxis, udp_socket: UdpSocket) {
    let frame_duration = 1000/config.fps;
    let mut sleep_duration;
    show.print_content();
    info!("");
    info!("Here we go!");
    info!("");
    loop {
        let loop_start_time = Instant::now();

        // Read all inputs
        let udp_message = udp_sockets::read_all_from_udp_socket(&udp_socket);

        // Update internal state
        show.update(udp_message);
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
