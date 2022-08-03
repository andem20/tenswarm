use std::error::Error;

use serde_yaml::Value;

pub fn load_yaml(path: &str) -> Result<Value, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;

    let value: Value = serde_yaml::from_reader(file)?;

    Ok(value)
}