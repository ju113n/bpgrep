use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use walkdir::WalkDir;

pub fn read_path(path: &String) -> Vec<Lines<BufReader<File>>> {
    WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|e| e.path().is_file())
        .flat_map(|e| File::open(e.path()))
        .map(|f| BufReader::new(f).lines())
        .collect()
}
