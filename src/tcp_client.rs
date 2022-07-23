use std::{collections::HashMap, io::Error};

use tokio::{net::TcpStream, io::{BufReader, AsyncWriteExt, AsyncBufReadExt, AsyncReadExt}};

type Connections = HashMap<String, BufReader<TcpStream>>;

pub struct TcpClient {
    connections: Connections,
}

impl TcpClient {
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

    pub async fn get(&mut self, addr: String, headers: &[u8]) -> Result<Vec<u8>, Error> {
        let stream = self.connections.get_mut(&addr).unwrap();

        stream.write_all(headers).await?;

        let len = stream.fill_buf().await.unwrap().len();
        let mut buffer = vec![0u8; len];
        stream.read_exact(&mut buffer).await?;

        Ok(buffer)
    }
}