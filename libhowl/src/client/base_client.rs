use serde_json::Value;
use tokio::sync::mpsc::Sender;

use crate::structs::{Command, InitCommand, InitCommandType};

#[derive(Default)]
pub struct BaseClient {
    r#type: InitCommandType,
    new_data_tx: Option<Sender<Value>>,
    initial_data_tx: Option<Sender<Value>>,
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
            .send(Command::Data(data)).await.unwrap();
    }

    pub fn set_command_out_tx(&mut self, tx: Sender<Command>) {
        self.command_out_tx = Some(tx);
    }

    pub async fn after_connection(&self) {
        let command = Command::Init(InitCommand {
            r#type: self.r#type
        });
        self.command_out_tx.as_ref().unwrap().send(command).await.unwrap();
    }

    pub async fn disconnect(&self) {
        self.command_out_tx.as_ref().unwrap().send(Command::CloseConnection).await.unwrap();
    }

    pub fn set_on_new_data(&mut self, tx: Sender<Value>) {
        self.new_data_tx = Some(tx);
    }

    pub fn set_on_initial_data(&mut self, tx: Sender<Value>) {
        self.initial_data_tx = Some(tx);
    }

    pub async fn execute_command(&self, command: Command) {
        if let Command::InitialState(data) = command {
            if self.initial_data_tx.is_some() {
                self.initial_data_tx.as_ref().unwrap().send(data).await.unwrap();
            }
        }
        else if let Command::Data(data) = command {
            if self.new_data_tx.is_some() {
                self.new_data_tx.as_ref().unwrap().send(data).await.unwrap();
            }
        }
    }
}