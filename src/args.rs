use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    count: usize,

    #[arg(short, long)]
    device: String,

    #[arg(short, long)]
    baudrate: u32,
}
