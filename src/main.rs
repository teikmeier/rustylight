mod dmx_buffer;
mod shape_utility;
mod led_bar;
mod led_bar_facade;

use core::time::Duration;
use std::thread;
use std::thread::sleep;
use std::error::Error;
use rust_dmx::{EnttecDmxPort, DmxPort};
use dmx_buffer::{DmxBuffer};
use std::time::Instant;
use std::str;
use std::net::{TcpListener, TcpStream};
use std::net::UdpSocket;
use shape_utility::ShapeUtility;
use led_bar::{LedBar, Led, Color};
use crate::led_bar_facade::LedBarFacade;
use crate::shape_utility::Shape;
use std::sync::{Arc, Mutex};

fn main() {
    start_udp();
}

fn create_led_bars() -> Vec<LedBar> {
    let mut led_bars: Vec<LedBar> = Vec::new();

    // @TODO: make some config stuff to create multiple bars
    let mut led_bar = LedBar::new(0);
    led_bar.set_global_brightness(255);
    led_bar.set_global_strobe(0);
    let mut offset = 2;
    for _i in 0..12 {
        let led = Led::new(offset);
        led_bar.add_led(led);
        offset = offset + 6;
    }

    led_bars.push(led_bar);
    return led_bars;
}

fn game_loop(led_bar_facade: &mut LedBarFacade, midi_signal: String, socket: &UdpSocket, time: Instant) -> Result<(), rust_dmx::Error> {
    // led_bar.set_color(Color::Red, 7, true);
    // led_bar.set_color(Color::Green, 245, false);
    // led_bar.set_color(Color::Blue, 138, false);
    // led_bar.set_color(Color::Black, 100, false);
    // led_bar.set_color(Color::Amber, 200, false);
    // led_bar.set_color(Color::White, 100, false);

    let mut buffer = DmxBuffer::new();
    let mut port = EnttecDmxPort::available_ports()?.pop().unwrap();
    port.open()?;

    let bpm = 145 as f64;
    let bps = bpm / 60.0;
    // led_bar_facade.sparkle();

    sleep(Duration::from_millis(40));

    loop {
        let loop_start_time = Instant::now();
        println!("current midi signal: {}", midi_signal);

        // <--> Example values start
        if midi_signal == "[144,60,78]" {
            led_bar_facade.set_color(&Color::Red, 7, true);
            led_bar_facade.set_color(&Color::Green, 245, false);
            led_bar_facade.set_color(&Color::Blue, 138, false);
        }

        if midi_signal.contains("[144,60") || midi_signal.contains("[128,60") {
            led_bar_facade.set_color(&Color::Red, 7, true);
            led_bar_facade.set_color(&Color::Green, 245, false);
            led_bar_facade.set_color(&Color::Blue, 138, false);
        }

        if midi_signal.contains("[144,62") || midi_signal.contains("[128,62") {
            led_bar_facade.set_color(&Color::Red, 128, true);
            led_bar_facade.set_color(&Color::Green, 7, false);
            led_bar_facade.set_color(&Color::Blue, 245, false);
        }

        if midi_signal.contains("[144,59") || midi_signal.contains("[128,59") {
            led_bar_facade.reset();
            break;
        } else {
            let timenow = (time.elapsed().as_millis() as f64) / 1000.0;
            led_bar_facade.animate(Shape::Sine, timenow, bps);
        }
        // <--> Example values end

        println!("{:?}", led_bar_facade.get_frame());
        buffer.set_values(led_bar_facade.get_frame());

        port.write(&buffer.get_values())?;

        let elapsed = loop_start_time.elapsed().as_millis() as u64;
        let mut sleep_duration = 40;

        if elapsed < 40 {
            sleep_duration = 40 - elapsed;
        }

        sleep(Duration::from_millis(sleep_duration));

        socket.set_read_timeout(Option::from(Duration::from_millis(1)));
        let mut buf = [0; 2048];
        let new_data_recevied: bool = match socket.peek(&mut buf) {
            Ok((amt)) => {
                let signal = str::from_utf8(&buf[..amt]).unwrap();
                println!("peek signal: {}", signal);
                !signal.trim().is_empty()
            }
            Err(err) => false,
        };

        if (new_data_recevied) {
            break;
        }
    }

    buffer.reset();
    port.write(&buffer.get_values())?;
    port.close();
    Ok(())
}

fn start_udp() -> std::io::Result<()> {
    println!("Start UDP Socket");
    let socket = UdpSocket::bind("127.0.0.1:9001")?;
    let mut buf = [0; 2048];

    let mut facade = LedBarFacade::new();
    let mut led_bars = create_led_bars();
    for mut led_bar in led_bars {
        facade.add_led_bar(led_bar);
    }

    let time = Instant::now();

    loop {
        // Receives a single datagram message on the socket.
        // If `buf` is too small to hold
        // the message, it will be cut off.
        socket.set_read_timeout(None);
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        let signal = String::from(str::from_utf8(&buf[..amt]).unwrap());

        if !signal.contains("[128") {
            game_loop(&mut facade, signal, &socket, time).unwrap();
            println!("Restart loop");
        }
    }
}
