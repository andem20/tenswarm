use crate::http_client::HttpClient;

pub struct Scenario {
    host: String,
    port: u16,
    ramp_up: u128,
    duration: u128,
    clients: Vec<HttpClient>,
}

impl Scenario {
    pub fn new(host: String, port: u16, ramp_up: u128, duration: u128, clients: usize) -> Self {
        let clients = Vec::<HttpClient>::with_capacity(clients)
            .iter()
            .map(|_| HttpClient::new())
            .collect();

        Self {
            host,
            port,
            ramp_up,
            duration,
            clients,
        }
    }

    pub fn execute(&self) {
        self.pretest();
        self.testloop();
        self.posttest();
        self.teardown();
    }

    fn pretest(&self) {
        todo!()
    }

    fn testloop(&self) {
        todo!()
    }

    fn teardown(&self) {
        todo!()
    }

    fn posttest(&self) {
        todo!()
    }
}
