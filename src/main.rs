mod args;
mod serial_writer;

use args::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    serial_writer::run(&args);
}
