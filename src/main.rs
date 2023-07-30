use clap::Parser;
mod serial_writer;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    count: usize,

    #[arg(short, long)]
    device: String,

    #[arg(short, long)]
    baudrate: u32,
}

fn main() {
    let opts = Cli::parse();
    serial_writer::write_to_serial(&opts.device, opts.count, opts.baudrate);
}
