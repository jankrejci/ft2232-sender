mod args;
mod serial_writer;

use args::Cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    serial_writer::run(args).await;
}
