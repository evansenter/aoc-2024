use std::fs;

pub fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read file: {}", filename))
}
