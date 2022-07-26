use std::{collections::HashMap, io::Error, sync::Arc, fmt};

use tokio::{net::TcpStream, io::{BufReader, AsyncWriteExt, AsyncBufReadExt, AsyncReadExt}};

type Connections = HashMap<String, BufReader<TcpStream>>;

#[derive(Debug)]
pub struct HttpClient {
    connections: Connections,
}

impl HttpClient {
    pub fn new() -> Self {
        Self { 
            connections: HashMap::new()
        }
    }

    pub async fn connect(&mut self, addr: &'static str) -> &mut Self {
        let connection = BufReader::new(TcpStream::connect(addr).await.unwrap());
        self.connections.insert(addr.to_owned(), connection);

        self
    }

    async fn request(&mut self, method: Method, addr: String, endpoint: String, headers: Arc<String>, body: Option<Arc<String>>) -> Result<Vec<u8>, Error> {
        let mut all_header = format!("{:?} {} HTTP/1.1\r\n", method, endpoint.as_str());
        all_header.push_str(&headers);
        all_header.push_str("\r\n\r\n");

        let stream = self.connections.get_mut(&addr).unwrap();

        stream.write_all(all_header.as_bytes()).await?;

        let len = stream.fill_buf().await.unwrap().len();
        let mut buffer = vec![0u8; len];
        stream.read_exact(&mut buffer).await?;

        Ok(buffer)
    }

    pub async fn get(&mut self, addr: String, endpoint: String, headers: Arc<String>) -> Result<Vec<u8>, Error> {
        self.request(Method::GET, addr, endpoint, headers, None).await
    }

    pub async fn post(&mut self, addr: String, endpoint: String, headers: Arc<String>, body: Arc<String>) -> Result<Vec<u8>, Error> {
        self.request(Method::POST, addr, endpoint, headers, Some(body)).await
    }
}

#[derive(Debug)]
enum Method {
    GET,
    POST
}