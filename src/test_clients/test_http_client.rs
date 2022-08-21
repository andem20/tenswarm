use std::{sync::Arc, time::Duration};

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
    pub fn new(host: &str, port: u16, scenario_map: Value, rx: Receiver<bool>) -> Self {
        let client: Client = Arc::new(Mutex::new(CustomHttpClient::new()));

        let addr = Arc::new(format!("{}:{}", &host, port));

        let steps = scenario_map["scenario"]["testloop"]["steps"]
            .as_sequence()
            .unwrap()
            .clone();

        let interval = utils::file::get_interval(&scenario_map);

        let client_data = Arc::new(Mutex::new(TestClientData::new(scenario_map, steps, rx, interval)));

        TestHttpClient {
            client,
            addr,
            client_data,
        }
    }
}

impl TestClient for TestHttpClient {
    fn test_loop(&self) -> tokio::task::JoinHandle<(TestResult)> {
        let headers = Arc::new("Host: localhost".to_owned());
        let client_data = self.client_data.clone();
        let client = self.client.clone();
        let addr = self.addr.clone();
        
        tokio::spawn(async move {
            client.lock().await.connect(addr.clone()).await;

            let mut total_response_count = 0;
            let mut total_response_time = 0;
            let steps = client_data.lock().await.steps();

            while client_data.lock().await.rx().is_empty() {
                // TODO Include ramp up
                if client_data.lock().await.interval() != 0 {
                    tokio::time::sleep(Duration::from_millis(client_data.lock().await.interval())).await;
                }

                for (i, step) in steps.iter().enumerate() {
                    let endpoint = step["step"]["endpoint"].as_str().unwrap();

                    let start_time = std::time::Instant::now();
                    let _resp = client.lock().await
                        .request(
                            Method::GET,
                            addr.clone(),
                            endpoint.to_owned(),
                            headers.clone(),
                            None,
                        )
                        .await
                        .unwrap();

                    client_data.lock().await
                        .add_response_time(i, start_time.elapsed().as_millis());
                    client_data.lock().await
                        .add_response_count(i, 1);
                }
            }

            client_data.lock().await
                .response_data()
                .iter()
                .for_each(|res_data| {
                    total_response_count += res_data.response_count();
                    total_response_time += res_data.response_time();
                });

            (total_response_count, total_response_time)
        })
    }

    fn pretest(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        
        let this = self.clone();
        tokio::spawn(async move {
            let hej = this;
            // let pretest = self.clone().client_data.clone().scenario_map().clone().get("pretest");
            // if pretest.is_none() {
            //     return ();
            // }

            println!("Sleeeeep");
            tokio::time::sleep(Duration::from_millis(1000)).await;
        })
    }
}
