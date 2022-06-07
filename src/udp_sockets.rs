use core::time::Duration;
use std::net::UdpSocket;
use std::error::Error;
use std::str;
use crate::shows::ShowUpdate;

const READ_FROM_ADDRESS: &str = "127.0.0.1:32567";
const WRITE_TO_ADDRESS: &str = "127.0.0.1:32568";

pub fn open_udp_socket() -> Result<UdpSocket, Box<dyn Error>> {
    let socket = UdpSocket::bind(WRITE_TO_ADDRESS).expect("couldn't bind to address");
    socket.connect(READ_FROM_ADDRESS).expect("connect function failed");
    socket.set_read_timeout(Option::from(Duration::from_millis(2))).expect("set_read_timeout call failed");
    Ok(socket)
}

pub fn read_all_from_udp_socket(udp_socket: &UdpSocket) -> ShowUpdate {
    let mut buf = [0; 2048];
    let mut update = ShowUpdate {
        song: None,
        scene: None,
        tempo: None,
    };

    loop {
        match udp_socket.recv(&mut buf) {
            Ok(received) => {
                let input_string = str::from_utf8(&buf[..received]).unwrap();
                if input_string.contains("song") {
                    update.song = Some(input_string.replace("song", "").parse::<usize>().unwrap());
                }
                if input_string.contains("scene") {
                    update.scene = Some(input_string.replace("scene", "").parse::<usize>().unwrap());
                }
                if input_string.contains("tempo") {
                    update.tempo = Some(input_string.replace("tempo", "").parse::<u8>().unwrap());
                }
                if input_string.eq("off") {
                    println!("Received an ALL_NOTES_OFF event, not sure what to do yet.");
                }
            },
            Err(_) => {
                break;
            }
        }
    }
    update
}