use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub pattern: String,
    pub filename: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub fn parse() -> Cli {
    Cli::parse()
}
