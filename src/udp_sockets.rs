use core::time::Duration;
use std::net::UdpSocket;
use std::error::Error;
use std::str;
use crate::shows::ShowUpdate;
use log::{trace, debug};

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
        off: None,
    };

    let mut tempo1 = None;
    let mut tempo2 = None;

    loop {
        match udp_socket.recv(&mut buf) {
            Ok(received) => {
                let input_string = str::from_utf8(&buf[..received]).unwrap();
                if input_string.contains("song_") {
                    update.song = Some(input_string.replace("song_", "").parse::<usize>().unwrap());
                }
                if input_string.contains("scene_") {
                    update.scene = Some(input_string.replace("scene_", "").parse::<usize>().unwrap());
                }
                if input_string.contains("tempo1_") {
                    tempo1 = Some(input_string.replace("tempo1_", "").parse::<u8>().unwrap());
                }
                if input_string.contains("tempo2_") {
                    tempo2 = Some(input_string.replace("tempo2_", "").parse::<u8>().unwrap());
                }
                if input_string.eq("off") {
                    debug!("Received an ALL_NOTES_OFF event.");
                    update.off = Some(true);
                }
            },
            Err(_) => {
                break;
            }
        }
    }
    if tempo1.is_some() && tempo2.is_some() {
        update.tempo = Some(tempo1.unwrap() + tempo2.unwrap());
    }
    if update.song.is_some() || update.scene.is_some() || tempo1.is_some() || tempo2.is_some() || update.tempo.is_some() || update.off.is_some() {
        trace!("Update: so {:?} - sc {:?} - t1 {:?} - t2 {:?} - t {:?} - off {:?}", update.song, update.scene, tempo1, tempo2, update.tempo, update.off);
    }
    update
}