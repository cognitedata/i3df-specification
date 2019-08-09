use std::io;
use std::io::prelude::*;
use std::fs;

pub fn as_string() -> io::Result<String> {
    let mut f = fs::File::open("i3df-spec.yaml")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

