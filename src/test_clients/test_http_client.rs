use std::{sync::Arc, time::Duration, borrow::Borrow};

use serde_yaml::Value;
use tokio::sync::{broadcast::Receiver, Mutex};

use crate::{
    clients::{client_trait::HttpClient, custom_http_client::CustomHttpClient, request::Method},
    utils,
};

use super::test_client::{TestClient, TestClientData, TestResult};

type Client = Arc<Mutex<dyn HttpClient + Send + Sync>>;

pub struct TestHttpClient {
    client: Client,
    addr: Arc<String>,
    client_data: Arc<Mutex<TestClientData>>,
}

impl TestHttpClient {
    pub fn new(
        id: usize,
        host: &str,
        port: u16,
        mut scenario_map: Value,
        rx: Receiver<bool>,
    ) -> Self {
        let client: Client = Arc::new(Mutex::new(CustomHttpClient::new()));

        let addr = Arc::new(format!("{}:{}", &host, port));
        let steps = utils::file::get_steps(&mut scenario_map);
        let interval = utils::file::get_interval(&scenario_map);

        let client_data = Arc::new(Mutex::new(TestClientData::new(
            scenario_map,
            steps,
            rx,
            interval,
            id,
        )));

        TestHttpClient {
            client,
            addr,
            client_data,
        }
    }
}

impl TestClient for TestHttpClient {
    fn test_loop(&self) -> tokio::task::JoinHandle<()> {
        let headers = Arc::new("Host: localhost".to_owned());
        let client_data = self.client_data.clone();
        let client = self.client.clone();
        let addr = self.addr.clone();

        tokio::spawn(async move {
            let mut client = client.lock().await;
            client.connect(addr.clone()).await;

            let mut client_data = client_data.lock().await;

            while client_data.rx().is_empty() {
                // TODO Include ramp up
                if client_data.interval() != 0 {
                    tokio::time::sleep(Duration::from_millis(client_data.interval())).await;
                }

                for step in client_data.steps.iter_mut() {
                    let endpoint = step.step()["endpoint"].as_str().unwrap();

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

                    let time = start_time.elapsed().as_millis();

                    step.add_time(time);
                    step.add_count();
                }
            }
        })
    }

    fn pretest(&self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {})
    }

    fn client_data(&self) -> Arc<Mutex<TestClientData>> {
        self.client_data.clone()
    }
    
}
