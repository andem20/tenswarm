use std::sync::Arc;

#[allow(dead_code)]
pub struct Request {
    method: Method,
    addr: Arc<String>,
    endpoint: String,
    headers: Arc<String>,
    body: Option<Arc<String>>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}
