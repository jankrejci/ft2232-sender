use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub count: usize,

    #[arg(short, long)]
    pub device: String,

    #[arg(short, long)]
    pub baudrate: u32,
}
