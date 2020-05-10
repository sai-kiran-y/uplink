use super::{Control, Package};
use derive_more::From;
use serde::{Deserialize, Serialize};
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{Sender, Receiver};

use std::time::{UNIX_EPOCH, SystemTime, SystemTimeError, Duration};
use std::collections::HashMap;

mod process;
pub mod controller;

pub use controller::Controller;

#[derive(Debug, From)]
pub enum Error {
    Serde(serde_json::Error),
    Stream(rumq_client::EventLoopError),
    Process(process::Error),
    Controller(controller::Error)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    // action id
    pub id:      String,
    // control or process
    kind:    String,
    // action name
    name:    String,
    // action payload. json. can be args/payload. depends on the invoked command
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionResponse {
    id:       String,
    // timestamp
    timestamp: u128,
    // running, failed
    state:    String,
    // progress percentage for processes
    progress: u8,
    // list of error
    errors:   Vec<String>,
}

impl ActionResponse {
    pub fn new(id: &str, state: &str) -> Result<Self, SystemTimeError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let status = ActionResponse { id: id.to_owned(), timestamp, state: state.to_owned(), progress: 0, errors: Vec::new() };
        Ok(status)
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}

pub struct Actions {
    process: process::Process,
    controller: controller::Controller,
    collector_tx: Sender<Box<dyn Package>>,
    actions_rx: Option<Receiver<Action>>
}


pub async fn new(
    collector_tx: Sender<Box<dyn Package>>,
    controllers: HashMap<String, Sender<Control>>,
    actions_rx: Receiver<Action>) -> Actions {

    let controller = Controller::new(controllers, collector_tx.clone());
    let process = process::Process::new(collector_tx.clone());


    Actions {
        process,
        controller,
        collector_tx,
        actions_rx: Some(actions_rx)
    }
}

impl Actions {
    pub async fn start(&mut self) {
        let mut notification_stream = self.actions_rx.take().unwrap();

        // start the eventloop
        loop {
            while let Some(action) = notification_stream.next().await {
                debug!("Action = {:?}", action);
                let action_id = action.id.clone();
                let action_name = action.name.clone();
                let error = match self.handle(action).await {
                    Ok(_) => continue,
                    Err(e) => e
                };

                self.forward_action_error(&action_id, &action_name, error).await;
            }

            tokio::time::delay_for(Duration::from_secs(1)).await;
        }
    }


    async fn handle(&mut self, action: Action) -> Result<(), Error> {
        debug!("Action = {:?}", action);

        match action.kind.as_ref() {
            "control" => {
                let command = action.name.clone();
                let payload = action.payload.clone();
                let id = action.id;
                self.controller.execute(&id, command, payload)?;
            }
            "process" => {
                let command = action.name.clone();
                let payload = action.payload.clone();
                let id = action.id;

                self.process.execute(id.clone(), command.clone(), payload).await?;
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    async fn forward_action_error(&mut self, id: &str, action: &str, error:Error) {
        error!("Failed to execute. Command = {:?}, Error = {:?}", action, error);
        let mut status = match ActionResponse::new(id, "Failed") {
            Ok(status) => status,
            Err(e) => {
                error!("Failed to create status. Error = {:?}", e);
                return
            }
        };

        status.add_error(format!("{:?}", error));
        if let Err(e) = self.collector_tx.send(Box::new(status)).await {
            error!("Failed to send status. Error = {:?}", e);
        }
    }
}


impl Package for ActionResponse {
    fn channel(&self) -> String {
        return "action_status".to_owned();
    }

    fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}
