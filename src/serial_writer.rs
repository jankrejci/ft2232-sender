use crate::args::Cli;
use packed_struct::prelude::*;
use rand::Rng;
use std::io::prelude::*;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

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

pub async fn run(cli: Cli) {
    let (tx, mut rx) = mpsc::channel(100);
    let start = Instant::now();

    let message_generator = tokio::spawn(async move {
        let mut bytes_sent = 0;
        while bytes_sent < cli.count {
            let message = generate_message();
            bytes_sent += message.len();
            tx.send(message)
                .await
                .expect("BUG: Failed to send message to the channel");
        }
        bytes_sent
    });

    let mut port = tokio_serial::new(cli.device, cli.baudrate)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open serial port");

    let message_writer = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            port.write_all(&message)
                .expect("Failed to write to serial port");
        }
    });

    let bytes_sent = message_generator
        .await
        .expect("BUG: Message generator failed");

    message_writer.await.expect("Message writer failed");

    let size_mb = bytes_sent as f64 / (1_000_000.0);
    let duration_s = start.elapsed().as_micros() as f64 / 1_000_000.0;
    let raw_speed = size_mb * 10.0 / duration_s;

    println!(
        "Sent {:.2} MB in {:.2} s, raw speed {:.6} Mb/s",
        size_mb, duration_s, raw_speed
    );
}

pub fn generate_message() -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let address: u8 = rng.gen();
    let length: u8 = rng.gen_range(1..(u8::MAX / 2));
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
