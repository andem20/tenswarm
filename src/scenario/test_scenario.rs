use serde_yaml::Value;
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use tokio::sync::broadcast::Sender;

use crate::{
    test_clients::{test_client::TestClient, test_http_client::TestHttpClient},
    utils,
};

pub struct Scenario {
    host: String,
    port: u16,
    ramp_up_millis: u128,
    duration_millis: u128,
    clients: Vec<Box<dyn TestClient>>,
    scenario_map: Value,
    tx: Sender<bool>,
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

        let ramp_up_millis = utils::time::string_to_millis_u128(ramp_up);
        let duration_millis = utils::time::string_to_millis_u128(duration);

        let (tx, _) = tokio::sync::broadcast::channel(1);

        let clients = create_clients(clients_size, &host, port, &scenario_map, &tx);

        Self {
            host,
            port,
            ramp_up_millis,
            duration_millis,
            clients,
            scenario_map,
            tx,
        }
    }

    pub async fn execute(self) {
        // self.pretest();
        self.testloop().await;
        // self.posttest();
        // self.teardown();
    }

    // fn pretest(&self) {}

    async fn testloop(self) {
        let mut tasks = Vec::with_capacity(self.clients.len());

        let interval = utils::file::get_interval(self.scenario_map);
        let time_offset = interval / self.clients.len() as u64;

        let total_start_time = Instant::now();

        self.clients.into_iter().for_each(|client| {
            thread::sleep(Duration::from_millis(time_offset));

            let task = client.test_loop();
            tasks.push(task);
        });

        let timer = utils::time::create_timer(self.duration_millis, self.tx);

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

    // fn teardown(&self) {}

    // fn posttest(&self) {}
}

fn create_clients(
    clients_size: usize,
    host: &String,
    port: u16,
    scenario_map: &Value,
    tx: &Sender<bool>,
) -> Vec<Box<dyn TestClient>> {
    let mut clients = Vec::with_capacity(clients_size);

    let addr = Arc::new(format!("{}:{}", &host, port));

    for _ in 0..clients_size {
        let client: Box<dyn TestClient> = Box::new(TestHttpClient::new(
            addr.clone(),
            scenario_map.clone(),
            tx.subscribe(),
        ));
        
        clients.push(client);
    }

    clients
}
