use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use serde_yaml::Value;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

use crate::{http_client::HttpClient, print_utils};

// #[derive(Debug)]
pub struct Scenario {
    host: String,
    port: u16,
    ramp_up: u128,  // Ramp up time in millis
    duration: u128, // Duration in millis
    clients: Vec<HttpClient>,
    scenario_map: Value,
}

impl Scenario {
    pub fn new(scenario_name: &'static str) -> Self {
        let file_path = format!("./scenarios/{scenario_name}.yml");

        let file = std::fs::File::open(file_path).expect("File does not exist.");

        let scenario_map: Value = serde_yaml::from_reader(file).unwrap();
        let scenario = &scenario_map["scenario"];

        let clients_size = scenario["clients"].as_u64().unwrap() as usize;
        let host = scenario["host"].as_str().unwrap().to_owned();
        let port = scenario["port"].as_u64().unwrap() as u16;
        let duration = scenario["duration"].as_str().unwrap();
        let ramp_up = scenario["ramp-up"].as_str().unwrap();

        let mut clients = Vec::with_capacity(clients_size);

        for _ in 0..clients_size {
            clients.push(HttpClient::new());
        }

        let ramp_up = time_to_millis(ramp_up);
        let duration = time_to_millis(duration);

        Self {
            host,
            port,
            ramp_up,
            duration,
            clients,
            scenario_map,
        }
    }

    pub async fn execute(self) {
        // println!("{:?}", self);
        // self.pretest();
        self.testloop().await;
        // self.posttest();
        // self.teardown();
    }

    fn pretest(&self) {}

    async fn testloop(mut self) {
        let mut tasks = Vec::with_capacity(self.clients.len());

        let duration = self.duration;
        let steps = self.scenario_map["scenario"]["testloop"]["steps"]
            .as_sequence()
            .unwrap()
            .clone();

        let interval = self.get_interval();

        let total_start_time = Instant::now();

        let addr = Arc::new(format!("{}:{}", &self.host, &self.port));

        let headers = Arc::new("Host: localhost".to_owned());
        let time_offset = interval / self.clients.len() as u64;

        for (_i, mut client) in self.clients.into_iter().enumerate() {
            thread::sleep(Duration::from_millis(time_offset));
            let steps = steps.clone();
            let headers = headers.clone();
            let addr = addr.clone();

            let task = tokio::spawn(async move {
                let client = client.connect(addr.clone()).await;

                let mut total_response_count = 0;

                while total_start_time.elapsed().as_millis() < duration {
                    // TODO Include ramp up
                    if interval != 0 {
                        tokio::time::sleep(Duration::from_millis(interval)).await;
                    }

                    for step in steps.iter() {
                        let endpoint = step["step"]["endpoint"].as_str().unwrap();

                        let _resp = client
                            .get(addr.clone(), endpoint.to_owned(), headers.clone())
                            .await
                            .unwrap();

                        total_response_count += 1;
                    }
                }

                total_response_count
            });

            tasks.push(task);
        }

        let test = futures::future::join_all(tasks).await;

        let mut total_response_count = 0;

        test.into_iter().for_each(|result| {
            let response_count = result.unwrap();
            total_response_count += response_count;
        });

        println!("Reponse count: {total_response_count}");
        println!("Requests pr. second: {}", total_response_count as f32 / total_start_time.elapsed().as_secs_f32());
    }

    fn teardown(&self) {}

    fn posttest(&self) {}

    fn get_interval(&self) -> u64 {
        let interval = self.scenario_map["scenario"]["testloop"]["interval"]
            .as_str()
            .unwrap();

        time_to_millis(interval) as u64
    }
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
