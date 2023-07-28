use std::time::{Duration, Instant};

pub fn write_to_serial(device: &str, file_path: &str, baudrate: u32) {
    let mut s = serialport::new(device, baudrate)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open serial port");

    let contents =
        std::fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines = contents.lines();

    for line in lines {
        let bytes = line
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).expect("Failed to parse byte"))
            .collect::<Vec<_>>();

        let start = Instant::now();
        s.write(&bytes).expect("Failed to write to serial port");
        let duration = start.elapsed();

        // Sleep only if the operation took less than 1ms.
        if duration < Duration::from_millis(1) {
            std::thread::sleep(Duration::from_millis(1) - duration);
        }
    }
}
