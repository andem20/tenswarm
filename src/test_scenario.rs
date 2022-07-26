use std::collections::HashMap;

use serde_yaml::Value;

use crate::http_client::HttpClient;

#[derive(Debug)]
pub struct Scenario {
    host: String,
    port: u16,
    ramp_up: u128,  // Ramp up time in millis
    duration: u128, // Duration in millis
    clients: Vec<HttpClient>,
}

impl Scenario {
    pub fn new(scenario_name: &'static str) -> Self {

        let file_path = format!("./scenarios/{scenario_name}.yml");

        let file = std::fs::File::open(file_path).expect("File does not exist.");

        let map: Value = serde_yaml::from_reader(file).unwrap();
        let scenario = &map["scenario"];

        let clients = scenario["clients"].as_u64().unwrap() as usize;
        let host = scenario["host"].as_str().unwrap().to_owned();
        let port = scenario["port"].as_u64().unwrap() as u16;
        let duration = scenario["duration"].as_str().unwrap();
        let ramp_up = scenario["ramp-up"].as_str().unwrap();

        let clients = Vec::<HttpClient>::with_capacity(clients)
            .iter()
            .map(|_| HttpClient::new())
            .collect();

        let ramp_up = time_to_millis(ramp_up);
        let duration = time_to_millis(duration);

        Self {
            host,
            port,
            ramp_up,
            duration,
            clients,
        }
    }

    pub fn execute(&self) {
        println!("{:?}", self);
        // self.pretest();
        // self.testloop();
        // self.posttest();
        // self.teardown();
    }

    fn pretest(&self) {}

    fn testloop(&self) {
        let start_time = std::time::Instant::now();

        while start_time.elapsed().as_millis() < self.duration {
            // Include ramp up
            println!("{}", start_time.elapsed().as_millis());
        }
    }

    fn teardown(&self) {}

    fn posttest(&self) {}
}

fn time_to_millis(time: &str) -> u128 {
    let unit: String = time.chars().filter(|c| !c.is_digit(10)).collect();
    let unit = unit.as_str();
    let time: u128 = time
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let factor = match unit {
        "t" => 3600000,
        "m" => 60000,
        "s" => 1000,
        "ms" => 1,
        _ => 1,
    };

    time * factor
}
