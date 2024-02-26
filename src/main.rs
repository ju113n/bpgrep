use std::fs::read_to_string;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String,
    filename: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let cli = Cli::parse();

    println!("pattern: {}", cli.pattern);
    println!("filename: {}", cli.filename);

    for line in read_to_string(cli.filename).unwrap().lines() {
        if line.contains(&cli.pattern) {
            println!("{line}");
        }
    }
}
