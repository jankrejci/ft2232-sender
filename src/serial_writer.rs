use packed_struct::prelude::*;
use rand::Rng;
use std::io::prelude::*;
use std::time::{Duration, Instant};

#[derive(PackedStruct)]
#[packed_struct(bit_numbering = "msb0")]
pub struct MessageHeader {
    #[packed_field(bits = "0..=7")]
    address: u8,
    #[packed_field(bits = "8..=14")]
    length: u8,
    #[packed_field(bits = "15")]
    write: bool,
}

pub fn write_to_serial(device: &str, count: usize, baudrate: u32) {
    let mut port = serialport::new(device, baudrate)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open serial port");

    let mut bytes_sent = 0;

    let start = Instant::now();
    while bytes_sent < count {
        let message = generate_message();
        bytes_sent += message.len();
        port.write_all(&message)
            .expect("Failed to write to serial port");
    }

    let duration = start.elapsed().as_micros() as f64 / 1_000_000.0;
    let size = bytes_sent as f64 / (1024.0 * 1024.0);
    let raw_speed = size * 10.0 / duration;

    println!(
        "Sent {:.2} MB in {:.2} s, raw speed {:.6} Mb/s",
        size, duration, raw_speed
    );
}

pub fn generate_message() -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let address: u8 = rng.gen();
    // let length: u8 = rng.gen_range(0..(u8::MAX / 2));
    let length: u8 = u8::MAX / 2;
    let message_header = MessageHeader {
        address,
        length,
        write: true,
    };
    let mut payload: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    let crc = crc(&payload);

    let mut message: Vec<u8> = message_header
        .pack()
        .expect("BUG: Failed to pack the struct")
        .into();
    message.append(&mut payload);
    message.push(crc);
    // println!("{:02X?}", message);
    message
}

fn crc(payload: &Vec<u8>) -> u8 {
    let mut payload = payload.clone();

    let poly: u8 = 0x31;
    let mut crc: u8 = 0xFF;

    for byte in payload.iter_mut() {
        for _ in 0..8 {
            if (*byte ^ crc) & 0x80 != 0 {
                crc = (crc << 1) ^ poly;
            } else {
                crc <<= 1;
            }
            *byte <<= 1;
        }
    }
    crc
}
