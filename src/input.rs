use std::env;
use std::fs;
use std::io;
use std::io::{Error, Read};

pub fn read_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    return Ok(buffer);
}

pub fn read_from_file() -> io::Result<String> {
    let content = fs::read_to_string("Cargo.toml")?;
    return Ok(content);
}
