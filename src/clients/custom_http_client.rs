use std::{collections::HashMap, error::Error, sync::Arc};

use async_trait::async_trait;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use super::{client_trait::HttpClient, request};

type Connections = HashMap<String, BufReader<TcpStream>>;

#[derive(Debug)]
pub struct CustomHttpClient {
    connections: Connections,
}

impl CustomHttpClient {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }
}

#[async_trait]
impl HttpClient for CustomHttpClient {
    async fn connect(mut self: Box<Self>, addr: Arc<String>) -> Box<dyn HttpClient> {
        let connection = BufReader::new(TcpStream::connect(addr.to_string()).await.unwrap());
        self.connections.insert(addr.to_string(), connection);

        self
    }

    async fn request(
        &mut self,
        method: request::Method,
        addr: Arc<String>,
        endpoint: String,
        headers: Arc<String>,
        body: Option<Arc<String>>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut all_header = format!("{:?} {} HTTP/1.1\r\n", method, endpoint.as_str());
        all_header.push_str(&headers);
        all_header.push_str("\r\n\r\n");

        let stream = self.connections.get_mut(&addr.to_string()).unwrap();

        stream.write_all(all_header.as_bytes()).await?;

        let len = stream.fill_buf().await.unwrap().len();
        let mut buffer = vec![0u8; len];
        stream.read_exact(&mut buffer).await?;

        Ok(buffer)
    }
}