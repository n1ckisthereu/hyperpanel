use std::fs::File;
use std::io::{Error, Read};

#[warn(dead_code)]
pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    Ok(contents)
}
