mod cli;
mod file;

fn main() {
    let args = cli::parse();

    println!("pattern: {}", args.pattern);
    println!("filename: {}", args.filename);

    if let Ok(lines) = file::read_file(&args.filename) {
        lines
            .flatten()
            .filter(|line| line.contains(&args.pattern))
            .for_each(|line| println!("{line}"));
    }
}
