use std::{sync::Arc, error::Error, str::FromStr};

use async_trait::async_trait;
use hyper::{client::HttpConnector, Uri};

use super::{client_trait::HttpClient, request::Method};

pub struct HyperHttpClient {
    client: hyper::Client<HttpConnector>
}

impl HyperHttpClient {
    pub fn new() -> Self {
        Self {
            client: hyper::Client::new()
        }
    }
}

#[async_trait]
impl HttpClient for HyperHttpClient {
    async fn connect(&mut self, addr: Arc<String>) {
        
    }

    async fn request(
        &mut self,
        method: Method,
        addr: Arc<String>,
        endpoint: String,
        headers: Arc<String>,
        body: Option<Arc<String>>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let addr = format!("http://{}{}", &addr.to_string(), &endpoint);
        let uri = Uri::from_str(&addr)?;
        let response = self.client.get(uri).await?;

        let body = format!("{:?}", response).as_bytes().to_vec();

        Ok(body)
    }
}
