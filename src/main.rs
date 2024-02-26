use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String,
    file: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let cli = Cli::parse();

    println!("pattern: {}", cli.pattern);
    println!("file: {}", cli.file);
}
