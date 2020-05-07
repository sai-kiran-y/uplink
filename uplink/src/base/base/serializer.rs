use crate::base::{Config, Package};

use futures_util::stream::StreamExt;
use rumq_client::{self, eventloop, MqttOptions, QoS, Request};
use tokio::sync::mpsc::{self, channel, Sender, Receiver};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub struct Serializer {
    config:       Config,
    collector_rx: Receiver<Box<dyn Package>>,
    mqtt_tx:      Sender<Request>,
}

impl Serializer {
    pub fn new(config: Config, collector_rx: Receiver<Box<dyn Package>>) -> Serializer {
        let (requests_tx, requests_rx) = channel::<Request>(1);

        let options = mqttoptions(config.clone());

        thread::spawn(move || mqtt(options, requests_rx));

        Serializer { config, collector_rx, mqtt_tx: requests_tx }
    }

    #[tokio::main(core_threads = 1)]
    pub async fn start(&mut self) {
        loop {
            let data = match self.collector_rx.recv().await {
                Some(data) => data,
                None => {
                    error!("Senders closed!!");
                    return
                }
            };
            let channel = &data.channel();

            let topic = self.config.channels.get(channel).unwrap().topic.clone();
            let payload = data.serialize();
            let qos = QoS::AtLeastOnce;

            let publish = rumq_client::publish(topic, qos, payload);
            let publish = Request::Publish(publish);
            self.mqtt_tx.send(publish).await.unwrap();
        }
    }
}

#[tokio::main(basic_scheduler)]
async fn mqtt(options: MqttOptions, requests_rx: mpsc::Receiver<Request>) {
    // create a new eventloop and reuse it during every reconnection
    let mut eventloop = eventloop(options, requests_rx);

    loop {
        let mut stream = eventloop.stream();
        while let Some(notification) = stream.next().await {
            debug!("Notification = {:?}", notification);
        }

        tokio::time::delay_for(Duration::from_secs(5)).await;
    }
}

fn mqttoptions(config: Config) -> MqttOptions {
    // let (rsa_private, ca) = get_certs(&config.key.unwrap(), &config.ca.unwrap());
    let mut mqttoptions = MqttOptions::new(config.device_id, config.broker, config.port);
    mqttoptions.set_keep_alive(30);
    mqttoptions
}

fn _get_certs(key_path: &Path, ca_path: &Path) -> (Vec<u8>, Vec<u8>) {
    println!("{:?}", key_path);
    let mut key = Vec::new();
    let mut key_file = File::open(key_path).unwrap();
    key_file.read_to_end(&mut key).unwrap();

    let mut ca = Vec::new();
    let mut ca_file = File::open(ca_path).unwrap();
    ca_file.read_to_end(&mut ca).unwrap();

    (key, ca)
}