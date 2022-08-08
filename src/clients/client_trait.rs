use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use super::request::Method;

#[async_trait]
pub trait HttpClient {
    async fn connect(&mut self, addr: Arc<String>) -> &mut Self;
    async fn request(
        &mut self,
        method: Method,
        addr: Arc<String>,
        endpoint: String,
        headers: Arc<String>,
        body: Option<Arc<String>>,
    ) -> Result<Vec<u8>, Box<dyn Error>>;
}
