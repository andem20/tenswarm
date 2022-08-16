use serde_yaml::Value;
use tokio::sync::broadcast::Receiver;

pub struct ResponseMetaData {
    response_count: u32,
    response_time: u128,
}

impl ResponseMetaData {
    fn new() -> Self {
        Self {
            response_count: 0,
            response_time: 0,
        }
    }

    pub fn response_count(&self) -> u32 {
        self.response_count
    }

    pub fn add_response_count(&mut self, count: u32) {
        self.response_count += count;
    }

    pub fn response_time(&self) -> u128 {
        self.response_time
    }

    pub fn add_response_time(&mut self, time: u128) {
        self.response_time += time;
    }
}

pub struct TestClientData {
    steps: Vec<Value>,
    response_data: Vec<ResponseMetaData>,
    rx: Receiver<bool>,
    interval: u64,
}

impl TestClientData {
    pub fn new(steps: Vec<Value>, rx: Receiver<bool>, interval: u64) -> Self {
        let mut response_data = Vec::with_capacity(steps.len());

        steps.iter().for_each(|_| {
            let meta_data = ResponseMetaData::new();
            response_data.push(meta_data);
        });

        Self {
            steps,
            response_data,
            rx,
            interval,
        }
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn steps(&self) -> Vec<Value> {
        self.steps.clone()
    }

    pub fn add_response_count(&mut self, index: usize, count: u32) {
        self.response_data
            .get_mut(index)
            .unwrap()
            .add_response_count(count);
    }

    pub fn add_response_time(&mut self, index: usize, time: u128) {
        self.response_data
            .get_mut(index)
            .unwrap()
            .add_response_time(time);
    }

    pub fn response_data(&self) -> &[ResponseMetaData] {
        self.response_data.as_ref()
    }

    pub fn rx(&self) -> &Receiver<bool> {
        &self.rx
    }

}

// TODO temporary type for test result
pub type TestResult = (u32, u128);

pub trait TestClient {
    fn test_loop(self: Box<Self>) -> tokio::task::JoinHandle<TestResult>;
}