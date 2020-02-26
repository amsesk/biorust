use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod sequence;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    Ok(io::BufReader::new(file).lines())
}
