use std::{sync::Arc, time::Duration};

use serde_yaml::Value;
use tokio::sync::broadcast::Receiver;

use crate::{clients::{client_trait::HttpClient, custom_http_client::CustomHttpClient, request::Method}, utils};

use super::test_client::{TestClientData, TestResult, TestClient};

type Client = Box<dyn HttpClient + Send>;

pub struct TestHttpClient {
    client: Client,
    addr: Arc<String>, 
    test_client: TestClientData
}

impl TestHttpClient {
    pub fn new(addr: Arc<String>, scenario_map: Value, rx: Receiver<bool>) -> Self {
        let client: Client = Box::new(CustomHttpClient::new());

        let steps = scenario_map["scenario"]["testloop"]["steps"]
            .as_sequence()
            .unwrap()
            .clone();

        let interval = utils::file::get_interval(scenario_map);

        let test_client = TestClientData::new(steps, rx, interval);

        TestHttpClient {
            client,
            addr,
            test_client
        }
    }
}

impl TestClient for TestHttpClient {
    fn test_loop(mut self: Box<Self>) -> tokio::task::JoinHandle<TestResult> {
        let headers = Arc::new("Host: localhost".to_owned());

        tokio::spawn(async move {
            let mut client = self.client.connect(self.addr.clone()).await;

            let mut total_response_count = 0;
            let mut total_response_time = 0;
            let steps = self.test_client.steps();
    
            while self.test_client.rx().is_empty() {
                // TODO Include ramp up
                if self.test_client.interval() != 0 {
                    tokio::time::sleep(Duration::from_millis(self.test_client.interval())).await;
                }

                for (i, step) in steps.iter().enumerate() {
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
    
                    self.test_client.add_response_time(i, start_time.elapsed().as_millis());
                    self.test_client.add_response_count(i, 1);
                }
            }

            self.test_client.response_data().iter().for_each(|res_data| {
                total_response_count += res_data.response_count();
                total_response_time += res_data.response_time();
            });

            (total_response_count, total_response_time)
        })
    }
}
