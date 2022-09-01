use std::sync::Arc;

use serde_yaml::Value;
use tokio::sync::{broadcast::Receiver, Mutex};

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

    pub fn time(&self) -> u128 {
        self.time
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub struct TestClientData {
    pub steps: Vec<Step>,
    rx: Receiver<bool>,
    interval: u64,
    scenario_map: Value,
    id: usize
}

impl TestClientData {
    pub fn new(scenario_map: Value, steps: Vec<Step>, rx: Receiver<bool>, interval: u64, id: usize) -> Self {
        Self {
            steps,
            rx,
            interval,
            scenario_map,
            id
        }
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn steps(&self) -> &Vec<Step> {
        &self.steps
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
    fn test_loop(&self) -> tokio::task::JoinHandle<()>;
    fn client_data(&self) -> Arc<Mutex<TestClientData>>;
}