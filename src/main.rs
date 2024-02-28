use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

mod cli;

fn main() {
    let args = cli::parse();

    println!("pattern: {}", args.pattern);
    println!("filename: {}", args.filename);

    if let Ok(lines) = read_file(&args.filename) {
        lines
            .flatten()
            .filter(|line| line.contains(&args.pattern))
            .for_each(|line| println!("{line}"));
    }
}

fn read_file(filepath: &String) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(BufReader::new(file).lines())
}
