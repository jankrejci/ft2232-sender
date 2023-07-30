mod args;
mod serial_writer;

use args::Cli;

fn main() {
    let opts = Cli::parse();
    serial_writer::write_to_serial(&opts.device, opts.count, opts.baudrate);
}
