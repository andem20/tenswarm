use serde_yaml::Value;

use crate::clients::{client_trait::HttpClient, custom_http_client::CustomHttpClient};

type Client = Box<dyn HttpClient>;

pub struct TestHttpClient {
    client: Client,
    pre_loop: Value,
    test_loop: Value,
    total_response_count: u64,
    total_response_time: u128,
}

impl TestHttpClient {
    pub fn new(pre_loop: Value, test_loop: Value) -> Self {
        let client: Client = Box::new(CustomHttpClient::new());

        TestHttpClient {
            client,
            pre_loop,
            test_loop,
            total_response_count: 0,
            total_response_time: 0,
        }
    }

    pub fn test_loop(&self) {

    }
}
