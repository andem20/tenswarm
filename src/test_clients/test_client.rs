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
#[derive(Debug, Clone)]
pub struct Step {
    step: Value,
    time: u128,
    count: usize,
}

impl Step {
    pub fn new(step: Value) -> Self {
        Self {
            step,
            time: 0,
            count: 0
        }
    }

    pub fn step(&self) -> &Value {
        &self.step
    }

    pub fn add_time(&mut self, time: u128) {
        self.time += time;
    }

    pub fn add_count(&mut self) {
        self.count += 1;
    }
}

pub struct TestClientData {
    steps: Vec<Step>,
    response_data: Vec<ResponseMetaData>,
    rx: Receiver<bool>,
    interval: u64,
    scenario_map: Value,
    id: usize
}

impl TestClientData {
    pub fn new(scenario_map: Value, steps: Vec<Step>, rx: Receiver<bool>, interval: u64, id: usize) -> Self {
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
            scenario_map,
            id
        }
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn steps(&self) -> Vec<Step> {
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


    pub fn scenario_map(&self) -> &Value {
        &self.scenario_map
    }
}

// TODO temporary type for test result
pub type TestResult = (u32, u128);


pub trait TestClient: Send {
    fn pretest(&self) -> tokio::task::JoinHandle<()>;
    fn test_loop(&self) -> tokio::task::JoinHandle<TestResult>;
}