use std::error::Error;
use std::fs;
use std::io::Read;

pub fn read_from_stdin() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    return Ok(buffer);
}

pub fn read_from_file(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    return Ok(content);
}
