use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::{Duration, Instant};

pub fn write_to_serial(device: &str, file_path: &str, baudrate: u32) {
    let mut port = serialport::new(device, baudrate)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open serial port");

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut bytes_sent = 0;

    let start = Instant::now();
    for line in reader.lines() {
        let bytes = line
            .expect("Failed to read line")
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).expect("Failed to parse byte"))
            .collect::<Vec<_>>();

        bytes_sent += bytes.len();

        port.write_all(&bytes)
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
