use clap::Parser;

use libhowl::Server;

/// Howl server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Host address
    #[clap(short = 'H', long, default_value = "127.0.0.1:8080")]
    host: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let server = Server::new();
    server.start(&args.host).await;
}