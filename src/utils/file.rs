use std::error::Error;

use serde_yaml::Value;

use crate::test_clients::test_client::Step;

use super::time;

pub fn load_yaml(path: &str) -> Result<Value, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;

    let value: Value = serde_yaml::from_reader(file)?;

    Ok(value)
}

pub fn get_interval(scenario_map: &Value) -> u64 {
    let interval = scenario_map["scenario"]["testloop"]["interval"]
        .as_str()
        .or(Some("0ms"))
        .unwrap();

    time::string_to_millis_u128(interval) as u64
}

pub fn get_steps(scenario_map: &mut Value) -> Vec<Step> {
    scenario_map["scenario"]["testloop"]["steps"]
        .as_sequence_mut()
        .unwrap()
        .iter_mut()
        .map(|step| Step::new(step.get("step").unwrap().to_owned()))
        .collect()
}
