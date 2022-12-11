use log::debug;
use serde_json::Value;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::data_store::DataStoreEvent;
use crate::structs::{Command, CommandType, InitCommand, InitCommandType};

#[derive(Default)]
pub struct BaseClient {
    r#type: InitCommandType,
    new_data_tx: Option<Sender<DataStoreEvent>>,
    initial_data_tx: Option<Sender<DataStoreEvent>>,
    command_out_tx: Option<Sender<Command>>
}

impl BaseClient {
    pub fn new(r#type: InitCommandType) -> Self {
        Self {
            r#type,
            ..Default::default()
        }
    }

    pub async fn share_data(&self, data: Value) {
        self.command_out_tx.as_ref()
            .expect("set_command_out_tx needs to be called before share_data!")
            .send(Command::new_data(data)).await.unwrap();
    }

    pub fn set_command_out_tx(&mut self, tx: Sender<Command>) {
        self.command_out_tx = Some(tx);
    }

    pub async fn after_connection(&self) {
        self.command_out_tx.as_ref().unwrap().send(Command::new_init(InitCommand {
            r#type: self.r#type
        })).await.unwrap();
    }

    pub async fn disconnect(&self) {
        self.command_out_tx.as_ref().unwrap().send(Command::new_close_connection()).await.unwrap();
    }

    pub fn set_on_new_data(&mut self, tx: Sender<DataStoreEvent>) {
        self.new_data_tx = Some(tx);
    }

    pub fn set_on_initial_data(&mut self, tx: Sender<DataStoreEvent>) {
        self.initial_data_tx = Some(tx);
    }

    pub async fn execute_command(&self, command: Command) {
        if command.r#type == CommandType::InitialData {
            if self.initial_data_tx.is_some() {
                self.initial_data_tx.as_ref().unwrap().send(command.initial_data.unwrap()).await.unwrap();
            }
        }
        else if command.r#type == CommandType::DataStoreEvent {
            if self.new_data_tx.is_some() {
                self.new_data_tx.as_ref().unwrap().send(command.event.unwrap()).await.unwrap();
            }
        }
    }
}