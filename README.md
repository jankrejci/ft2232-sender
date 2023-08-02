# Description
Simple tool for sending random data over serial port. Intention is
to provide UART data source for debugging other applications through
FT2232 UART<->USB interface.

# Usage

```
cargo run --release -- -d /dev/ttyUSB1 -c 100000000 -b 12000000
```

# Results

```
Sent 100.00 MB in 83.40 s, raw speed 11.990187 Mb/s
```

