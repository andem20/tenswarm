use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use super::request::Method;

#[async_trait]
pub trait HttpClient: Send {
    async fn connect(self: Box<Self>, addr: Arc<String>) -> Box<dyn HttpClient>;
    async fn request(
        &mut self,
        method: Method,
        addr: Arc<String>,
        endpoint: String,
        headers: Arc<String>,
        body: Option<Arc<String>>,
    ) -> Result<Vec<u8>, Box<dyn Error>>;
}
