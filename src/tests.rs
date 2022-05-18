#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::time::Duration;
    use crate::server::Server;
    use test_log::test;
    use url::Url;
    use crate::client::Client;
    use crate::structs::InitCommandType;

    #[test(tokio::test)]
    async fn spawn_server() {
        let host = "127.0.0.1:8000";
        tokio::spawn(async {
            let server = Server::new();
            server.start(host).await;
        });
        tokio::time::sleep(Duration::from_secs(2)).await;
        let mut client = Client::new(InitCommandType::Provider);
        client.connect(Url::from_str(format!("ws://{}", host).as_str()).unwrap()).await;
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}