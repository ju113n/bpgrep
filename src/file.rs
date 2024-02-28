use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
};

pub fn read_file(filepath: &String) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(BufReader::new(file).lines())
}
