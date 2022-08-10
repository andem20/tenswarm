use serde_yaml::Value;
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use crate::{
    clients::{client_trait::HttpClient, custom_http_client::CustomHttpClient, request::Method},
    utils,
};

type TestResult = (u32, u128);

pub struct Scenario {
    host: String,
    port: u16,
    ramp_up_millis: u128,
    duration_millis: u128,
    clients: Vec<Box<dyn HttpClient>>,
    scenario_map: Value,
}

impl Scenario {
    pub fn new(scenario_name: &'static str) -> Self {
        let file_path = format!("./scenarios/{scenario_name}.yml");

        let scenario_map = utils::file::load_yaml(&file_path).unwrap();
        let scenario = &scenario_map["scenario"];

        let clients_size = scenario["clients"].as_u64().unwrap() as usize;
        let host = scenario["host"].as_str().unwrap().to_owned();
        let port = scenario["port"].as_u64().unwrap() as u16;
        let duration = scenario["duration"].as_str().unwrap();
        let ramp_up = scenario["ramp-up"].as_str().unwrap();

        let clients = create_clients(clients_size);

        let ramp_up_millis = utils::time::string_to_millis_u128(ramp_up);
        let duration_millis = utils::time::string_to_millis_u128(duration);

        Self {
            host,
            port,
            ramp_up_millis,
            duration_millis,
            clients,
            scenario_map,
        }
    }

    pub async fn execute(self) {
        // self.pretest();
        self.testloop().await;
        // self.posttest();
        // self.teardown();
    }

    fn pretest(&self) {}

    async fn testloop(self) {
        let mut tasks = Vec::with_capacity(self.clients.len());

        let steps = self.scenario_map["scenario"]["testloop"]["steps"]
            .as_sequence()
            .unwrap()
            .clone();

        let interval = self.get_interval();
        let time_offset = interval / self.clients.len() as u64;
        let addr = Arc::new(format!("{}:{}", &self.host, &self.port));
        let headers = Arc::new("Host: localhost".to_owned());

        let total_start_time = Instant::now();

        for (_i, client) in self.clients.into_iter().enumerate() {
            thread::sleep(Duration::from_millis(time_offset));
            let task = create_test_task(
                client,
                &steps,
                &headers,
                &addr,
                total_start_time,
                interval,
                self.duration_millis,
            );

            tasks.push(task);
        }

        let timer = utils::time::create_timer(self.duration_millis);

        let test = futures::future::join_all(tasks).await;
        timer.await.unwrap();

        let mut total_response_count = 0;
        let mut total_response_time = 0;

        test.into_iter().for_each(|result| {
            let (response_count, response_time) = result.unwrap();
            total_response_count += response_count;
            total_response_time += response_time;
        });

        utils::print::print_conclusion(total_start_time, total_response_count, total_response_time);
    }

    fn teardown(&self) {}

    fn posttest(&self) {}

    fn get_interval(&self) -> u64 {
        let interval = self.scenario_map["scenario"]["testloop"]["interval"]
            .as_str()
            .unwrap();

        utils::time::string_to_millis_u128(interval) as u64
    }
}

fn create_clients(clients_size: usize) -> Vec<Box<dyn HttpClient>> {
    let mut clients: Vec<Box<dyn HttpClient>> = Vec::with_capacity(clients_size);

    for _ in 0..clients_size {
        clients.push(Box::new(CustomHttpClient::new()));
    }

    clients
}

fn create_test_task(
    client: Box<dyn HttpClient>,
    steps: &Vec<Value>,
    headers: &Arc<String>,
    addr: &Arc<String>,
    total_start_time: Instant,
    interval: u64,
    duration_millis: u128,
) -> tokio::task::JoinHandle<TestResult> {
    let steps = steps.clone();
    let headers = headers.clone();
    let addr = addr.clone();

    tokio::spawn(async move {
        let mut client = client.connect(addr.clone()).await;

        let mut total_response_count: u32 = 0;
        let mut total_response_time = 0;

        while total_start_time.elapsed().as_millis() < duration_millis {
            // TODO Include ramp up
            if interval != 0 {
                tokio::time::sleep(Duration::from_millis(interval)).await;
            }

            for step in steps.iter() {
                let endpoint = step["step"]["endpoint"].as_str().unwrap();

                let start_time = std::time::Instant::now();
                let _resp = client
                    .request(
                        Method::GET,
                        addr.clone(),
                        endpoint.to_owned(),
                        headers.clone(),
                        None,
                    )
                    .await
                    .unwrap();

                total_response_time += start_time.elapsed().as_millis();

                total_response_count += 1;
            }
        }

        (total_response_count, total_response_time)
    })
}
