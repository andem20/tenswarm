use crate::clients::client_trait::HttpClient;

type Client = Box<dyn HttpClient>;

pub struct TestHttpClient {
    client: Client
}
