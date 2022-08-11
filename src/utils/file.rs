use std::error::Error;

use serde_yaml::Value;

use super::time;

pub fn load_yaml(path: &str) -> Result<Value, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;

    let value: Value = serde_yaml::from_reader(file)?;

    Ok(value)
}

pub fn get_interval(scenario_map: Value) -> u64 {
    let interval = scenario_map["scenario"]["testloop"]["interval"]
        .as_str()
        .unwrap();

    time::string_to_millis_u128(interval) as u64
}