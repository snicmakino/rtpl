use std::error::Error;

use serde_json::Value;

use crate::input;

pub fn get_from_file(file: &str) -> Result<Value, Box<dyn Error>> {
    let json = input::read_from_file(file)?;
    return match serde_json::from_str(&json) {
        Ok(value) => Ok(value),
        Err(err) => Err(Box::new(err))
    };
}
