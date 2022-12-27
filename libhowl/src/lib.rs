pub use client::Client;
pub use structs::InitCommandType;

#[cfg(feature = "default")]
pub use server::Server;
#[cfg(feature = "default")]
mod server;

mod utils;
mod tests;
mod structs;
mod client;
mod data_store;
mod types;

