use serde_yaml::Value;
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant}, collections::HashMap,
};
use tokio::sync::broadcast::Sender;

use crate::{
    test_clients::{
        test_client::TestClient, test_http_client::TestHttpClient, test_mqtt_client::TestMqttClient,
    },
    utils::{self, print},
};

pub struct Scenario {
    ramp_up_millis: u128,
    duration_millis: u128,
    clients: Vec<Arc<dyn TestClient>>,
    scenario_map: Value,
    tx: Sender<bool>,
}

impl Scenario {
    pub fn new(scenario_name: &'static str) -> Self {
        let file_path = format!("./scenarios/{scenario_name}.yml");

        let scenario_map = utils::file::load_yaml(&file_path).unwrap();
        let scenario = &scenario_map["scenario"];
        let protocol = scenario["protocol"].as_str().unwrap();

        let clients_size = scenario["clients"].as_u64().unwrap() as usize;
        let host = scenario["host"].as_str().unwrap().to_owned();
        let port = scenario["port"].as_u64().unwrap() as u16;
        let duration = scenario["duration"].as_str().unwrap();
        let ramp_up = scenario["ramp-up"].as_str().unwrap();

        let ramp_up_millis = utils::time::string_to_millis_u128(ramp_up);
        let duration_millis = utils::time::string_to_millis_u128(duration);

        let (tx, _) = tokio::sync::broadcast::channel(1);

        let clients = match protocol {
            "http" => create_http_clients(clients_size, &host, port, &scenario_map, &tx),
            "mqtt" => create_mqtt_clients(clients_size, &host, port, &scenario_map, &tx),
            _ => panic!("No protocol specified"),
        };

        Self {
            ramp_up_millis,
            duration_millis,
            clients,
            scenario_map,
            tx,
        }
    }

    pub async fn execute(&self) {
        self.pretest().await;
        self.testloop().await;
        // self.posttest();
        // self.teardown();
    }

    async fn pretest(&self) {
        let mut tasks = Vec::with_capacity(self.clients.len());

        self.clients.iter().for_each(|client| {
            let task = client.clone().pretest();
            tasks.push(task);
        });

        let _result = futures::future::join_all(tasks).await;
    }

    async fn testloop(&self) {
        let mut tasks = Vec::with_capacity(self.clients.len());

        let interval = utils::file::get_interval(&self.scenario_map);
        let time_offset = interval / self.clients.len() as u64;

        let total_start_time = Instant::now();

        self.clients.iter().for_each(|client| {
            thread::sleep(Duration::from_millis(time_offset));

            let task = client.clone().test_loop();
            tasks.push(task);
        });

        let timer = utils::time::create_timer(self.duration_millis, self.tx.clone());

        futures::future::join_all(tasks).await;
        timer.await.unwrap();

        let mut steps_vec: Vec<(u128, usize)> = Vec::new(); // (time, count)

        for client in self.clients.iter() {
            let client_data = client.client_data();
            for (i, step) in client_data.lock().await.steps().iter().enumerate() {
                if let Some(step_values) = steps_vec.get_mut(i) {
                    step_values.0 += step.time();
                    step_values.1 += step.count();
                } else {
                    steps_vec.insert(i, (0, 0));
                }
            }
        }

        for (i, step) in steps_vec.iter().enumerate() {
            let avg_response_time = step.0 as f64 / step.1 as f64;
            let requests_per_second = (step.1 as f64 / total_start_time.elapsed().as_secs_f64()) as u32;
            println!("Step #{}: {:.2} ms, {} req/sec", i, avg_response_time, requests_per_second);
        }

        // utils::print::print_conclusion(total_start_time, total_response_count, total_response_time);
    }

    // fn teardown(&self) {}

    // fn posttest(&self) {}
}

// TODO fix recurrent code
fn create_http_clients(
    clients_size: usize,
    host: &String,
    port: u16,
    scenario_map: &Value,
    tx: &Sender<bool>,
) -> Vec<Arc<dyn TestClient>> {
    let mut clients = Vec::with_capacity(clients_size);

    for i in 0..clients_size {
        let client: Arc<dyn TestClient> = Arc::new(TestHttpClient::new(
            i,
            host,
            port,
            scenario_map.clone(),
            tx.subscribe(),
        ));

        clients.push(client);
    }

    clients
}

fn create_mqtt_clients(
    clients_size: usize,
    host: &String,
    port: u16,
    scenario_map: &Value,
    tx: &Sender<bool>,
) -> Vec<Arc<dyn TestClient>> {
    let mut clients = Vec::with_capacity(clients_size);

    for i in 0..clients_size {
        let client: Arc<dyn TestClient> = Arc::new(TestMqttClient::new(
            i,
            host,
            port,
            scenario_map.clone(),
            tx.subscribe(),
        ));

        clients.push(client);
    }

    clients
}
