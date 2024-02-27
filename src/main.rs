use std::fs::read_to_string;

mod cli;

fn main() {
    let args = cli::parse();

    println!("pattern: {}", args.pattern);
    println!("filename: {}", args.filename);

    for line in read_to_string(args.filename).unwrap().lines() {
        if line.contains(&args.pattern) {
            println!("{line}");
        }
    }
}
