use std::sync::Arc;

use rumqttc::{AsyncClient, MqttOptions, EventLoop};
use serde_yaml::Value;
use tokio::sync::broadcast::Receiver;

use crate::utils;

use super::test_client::{TestClient, TestClientData};

pub struct TestMqttClient {
    client: AsyncClient,
    eventloop: EventLoop,
    client_data: TestClientData,
}

impl TestMqttClient {
    pub fn new(id: usize, host: &str, port: u16, scenario_map: Value, rx: Receiver<bool>) -> Self {
        let id = format!("mqtt_client_{}", id);
        let mqtt_options = MqttOptions::new(id, host, port);

        let (client, eventloop) = AsyncClient::new(mqtt_options, 10);

        let steps = scenario_map["scenario"]["testloop"]["steps"]
            .as_sequence()
            .unwrap()
            .clone();

        let interval = utils::file::get_interval(&scenario_map);

        let client_data = TestClientData::new(scenario_map, steps, rx, interval);

        Self {
            client,
            eventloop,
            client_data,
        }
    }
}
