use std::sync::Arc;

use rumqttc::{AsyncClient, EventLoop, MqttOptions};
use serde_yaml::Value;
use tokio::sync::{broadcast::Receiver, Mutex};

use crate::utils;

use super::test_client::{Step, TestClient, TestClientData};

pub struct TestMqttClient {
    client: Arc<Mutex<AsyncClient>>,
    eventloop: Arc<Mutex<EventLoop>>,
    client_data: Arc<Mutex<TestClientData>>,
}

impl TestMqttClient {
    pub fn new(id: usize, host: &str, port: u16, scenario_map: Value, rx: Receiver<bool>) -> Self {
        let id_str = format!("mqtt_client_{}", &id);
        let mut mqtt_options = MqttOptions::new(id_str, host, port);

        let credentials = scenario_map
            .get("scenario")
            .unwrap()
            .get("credentials")
            .unwrap();
        let username = credentials.get("username").unwrap().as_str().unwrap();
        let password = credentials.get("password").unwrap().as_str().unwrap();
        mqtt_options.set_credentials(username, password);

        let (client, eventloop) = AsyncClient::new(mqtt_options, 10);

        let steps = scenario_map["scenario"]["testloop"]["steps"]
            .as_sequence()
            .unwrap()
            .clone();

        let mut steps_vec = Vec::with_capacity(steps.len());

        for step in steps {
            let step = Step::new(step.get("step").unwrap().to_owned());
            steps_vec.push(step);
        }

        let interval = utils::file::get_interval(&scenario_map);

        let client_data = Arc::new(Mutex::new(TestClientData::new(
            scenario_map,
            steps_vec,
            rx,
            interval,
            id,
        )));

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

            let pretest = client_data
                .scenario_map()
                .get("scenario")
                .unwrap()
                .get("pretest");
            if pretest.is_none() {
                return ();
            }

            let subs = pretest
                .unwrap()
                .get("subscribe")
                .unwrap()
                .as_sequence()
                .unwrap();

            for sub in subs {
                let topic = sub.as_str().unwrap();
                client
                    .subscribe(topic, rumqttc::QoS::AtLeastOnce)
                    .await
                    .unwrap();
            }
        })
    }

    fn test_loop(&self) -> tokio::task::JoinHandle<()> {
        let client = self.client.clone();
        let client_data = self.client_data.clone();

        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);

        tokio::spawn(async move {
            let publish = Value::String("publish".to_owned());
            let client_data = client_data.lock().await;
            while client_data.rx().is_empty() {
                for step in client_data.steps() {
                    let step = step.step().as_mapping().unwrap();
                    if step.contains_key(&publish) {
                        let client = client.lock().await;
                        let _publish = client
                            .publish(
                                "some/topic",
                                rumqttc::QoS::ExactlyOnce,
                                false,
                                "hej".as_bytes().to_vec(),
                            )
                            .await;
                        // tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }

            let _stop = client
                .lock()
                .await
                .publish("stop", rumqttc::QoS::ExactlyOnce, false, "stop")
                .await;
        });

        let client_data = self.client_data.clone();
        let eventloop = self.eventloop.clone();
        tokio::spawn(async move {
            let mut response_count = 0;

            while client_data.lock().await.rx().is_empty() {
                let mut eventloop = eventloop.lock().await;

                if let Ok(message) = eventloop.poll().await {
                    println!("{:?}", message);
                    response_count += 1;
                    // if await: wait for publish on topic -> send message on tx
                }
            }
        })
    }

    fn client_data(&self) -> Arc<Mutex<TestClientData>> {
        todo!()
    }
}