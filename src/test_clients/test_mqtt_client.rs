use std::{sync::Arc, time::Duration};

use rumqttc::{AsyncClient, MqttOptions, EventLoop};
use serde_yaml::Value;
use tokio::sync::{broadcast::Receiver, Mutex};

use crate::utils;

use super::test_client::{TestClientData, TestClient, TestResult};

pub struct TestMqttClient {
    client: Arc<Mutex<AsyncClient>>,
    eventloop: Arc<Mutex<EventLoop>>,
    client_data: Arc<Mutex<TestClientData>>,
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

        let client_data = Arc::new(Mutex::new(TestClientData::new(scenario_map, steps, rx, interval)));

        Self {
            client: Arc::new(Mutex::new(client)),
            eventloop: Arc::new(Mutex::new(eventloop)),
            client_data,
        }
    }
}

impl TestClient for TestMqttClient {
    fn pretest(&self) -> tokio::task::JoinHandle<()> {
        let client_data = self.client_data.clone();
        let client = self.client.clone();
        
        tokio::spawn(async move {
            let client_data = client_data.lock().await;
            let client = client.lock().await;
            
            let pretest = client_data.scenario_map().get("scenario").unwrap().get("pretest");
            if pretest.is_none() {
                return ();
            }

            let subs = pretest.unwrap().get("subscribe").unwrap().as_sequence().unwrap();

            for sub in subs {
                let topic = sub.as_str().unwrap();
                client.subscribe(topic, rumqttc::QoS::AtLeastOnce).await.unwrap();
            }
        })
    }

    fn test_loop(&self) -> tokio::task::JoinHandle<TestResult> {
        tokio::spawn(async move {
            (10, 10)
        })
    }
}