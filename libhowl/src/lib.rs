pub use client::Client;

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

