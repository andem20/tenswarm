use std::sync::Arc;

pub struct Request {
    method: Method,
    addr: Arc<String>,
    endpoint: String,
    headers: Arc<String>,
    body: Option<Arc<String>>,
}

impl Request {
    pub fn new(
        method: Method,
        addr: Arc<String>,
        endpoint: String,
        headers: Arc<String>,
        body: Option<Arc<String>>,
    ) -> Self {
        Request {
            method,
            addr,
            endpoint,
            headers,
            body,
        }
    }
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}
