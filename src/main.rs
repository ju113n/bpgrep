mod cli;
mod file;
mod search;

fn main() {
    let args = cli::parse();

    println!("pattern: {}", args.pattern);
    println!("filename: {}", args.filename);

    file::read_path(&args.filename)
        .into_iter()
        .flatten()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .filter(|line| line.contains(&args.pattern))
        .for_each(|line| println!("--> {line}"));

    search::boyer_moore(&String::from("toto"), &args.filename);
}
