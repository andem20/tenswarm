use std::{sync::Arc, time::Duration};

use serde_yaml::Value;
use tokio::sync::broadcast::Receiver;

use crate::{clients::{client_trait::HttpClient, custom_http_client::CustomHttpClient, request::Method}, utils};

type Client = Box<dyn HttpClient + Send>;
type TestResult = (u32, u128);

pub struct TestHttpClient {
    client: Client,
    addr: Arc<String>, 
    steps: Vec<Value>,
    interval: u64,
    rx: Receiver<bool>,
    total_response_count: u32,
    total_response_time: u128,
}

impl TestHttpClient {
    pub fn new(addr: Arc<String>, scenario_map: Value, rx: Receiver<bool>) -> Self {
        let client: Client = Box::new(CustomHttpClient::new());

        let steps = scenario_map["scenario"]["testloop"]["steps"]
            .as_sequence()
            .unwrap()
            .clone();

        let interval = utils::file::get_interval(scenario_map);

        TestHttpClient {
            client,
            addr,
            steps,
            interval,
            rx,
            total_response_count: 0,
            total_response_time: 0,
        }
    }

    pub fn test_loop(mut self) -> tokio::task::JoinHandle<TestResult> {
        let headers = Arc::new("Host: localhost".to_owned());

        tokio::spawn(async move {
            let mut client = self.client.connect(self.addr.clone()).await;
    
            while self.rx.is_empty() {
                // TODO Include ramp up
                if self.interval != 0 {
                    tokio::time::sleep(Duration::from_millis(self.interval)).await;
                }
    
                for step in self.steps.iter() {
                    let endpoint = step["step"]["endpoint"].as_str().unwrap();
    
                    let start_time = std::time::Instant::now();
                    let _resp = client
                        .request(
                            Method::GET,
                            self.addr.clone(),
                            endpoint.to_owned(),
                            headers.clone(),
                            None,
                        )
                        .await
                        .unwrap();
    
                    self.total_response_count += 1;
                    self.total_response_time += start_time.elapsed().as_millis();
                }
            }

            (self.total_response_count, self.total_response_time)
        })
    }
}
