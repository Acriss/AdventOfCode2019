use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};
use std::path::Path;

pub fn read_file_lines<P> (filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path> {
    let file: File = File::open(filename)?;
    let buffer: BufReader<File>= BufReader::new(file);
    Ok(buffer.lines())
}
