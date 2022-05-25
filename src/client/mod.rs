#[cfg(feature = "default")]
mod native_client;
#[cfg(feature = "default")]
pub use native_client::Client;

#[cfg(feature = "wasm")]
mod wasm_client;
#[cfg(feature = "wasm")]
pub use wasm_client::Client;

