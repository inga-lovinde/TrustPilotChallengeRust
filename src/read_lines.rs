use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    io::BufReader::new(File::open(filename)?).lines().collect()
}
