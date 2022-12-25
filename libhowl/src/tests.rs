#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::time::Duration;

    use assert_json_diff::assert_json_eq;
    use log::debug;
    use serde_json::json;
    use test_log::test;
    use tokio::sync::mpsc::channel;
    use url::Url;

    use crate::client::Client;
    use crate::data_store::{Chart, DataType};
    use crate::server::Server;
    use crate::structs::InitCommandType;

    #[test]
    fn test_chart() {
        let mut test_state = HashMap::new();
        let chart = Chart::new();
        chart.process_data(&json!({
            "title": "Test",
            "data": [{
                "timestamp": 2932438,
                "value": 1
            }]
        }), &mut test_state);
        assert_json_eq!(serde_json::to_string(&test_state).unwrap(), serde_json::to_string(&json!({
          "test": {
            "title": "Test",
            "values": [
              [
                1
              ]
            ],
            "x_points": [
              2932438
            ],
            "x_type": "DateTime"
          }
        })).unwrap());
        chart.process_data(&json!({
            "title": "Test",
            "data": [{
                "timestamp": 2932439,
                "value": 2
            }]
        }), &mut test_state);
        assert_json_eq!(test_state, json!({
          "test": {
            "title": "Test",
            "values": [
              [
                1,
                2
              ]
            ],
            "x_points": [
              2932438,
              2932439
            ],
            "x_type": "DateTime"
          }
        }));
        chart.process_data(&json!({
            "title": "Test2",
            "data": [{
                "timestamp": 28320932,
                "value": 3
            }]
        }), &mut test_state);
        debug!("test_state: {}", serde_json::to_string_pretty(&test_state).unwrap());
        assert_json_eq!(test_state, json!({
          "test": {
            "title": "Test",
            "values": [
              [
                1,
                2
              ]
            ],
            "x_points": [
              2932438,
              2932439
            ],
            "x_type": "DateTime"
          },
          "test2": {
            "title": "Test2",
            "values": [
              [
                3
              ]
            ],
            "x_points": [
              28320932
            ],
            "x_type": "DateTime"
          }
        }));
    }

    #[test(tokio::test)]
    async fn server_test() {
        let host = "127.0.0.1:8000";
        tokio::spawn(async {
            let server = Server::new();
            server.start(host).await;
        });
        let server_url = Url::from_str(format!("ws://{}", host).as_str()).unwrap();
        tokio::time::sleep(Duration::from_secs(2)).await;

        let (on_initial_data_tx, mut on_initial_data_rx) = channel(16);
        let (on_new_data_tx, mut on_new_data_rx) = channel(16);
        let mut client_sub = Client::new(InitCommandType::Subscriber);
        client_sub.set_on_initial_data(on_initial_data_tx).await;
        client_sub.set_on_new_data(on_new_data_tx).await;
        tokio::spawn(async move {
            loop {
                let event = on_initial_data_rx.recv().await.unwrap();
                debug!("on_initial_data_rx event: {}", serde_json::to_string(&event).unwrap());
            }
        });
        tokio::spawn(async move {
            loop {
                let event = on_new_data_rx.recv().await.unwrap();
                debug!("on_new_data_rx event: {}", serde_json::to_string(&event).unwrap());
            }
        });
        client_sub.connect(server_url.clone()).await;
        let mut client_pro = Client::new(InitCommandType::Provider);
        client_pro.connect(server_url.clone()).await;
        tokio::time::sleep(Duration::from_secs(2)).await;

        let data = serde_json::to_string(&json!({
            "title": "Test",
            "data": [{
                "timestamp": 2932438,
                "value": 1
            }]
        })).unwrap();
        debug!("Sharing data: {}", data);
        client_pro.share_data(serde_json::from_str(&data).unwrap()).await;
        debug!("testing disconnect and sending data (is client cleaned up from the server?)");
        tokio::time::sleep(Duration::from_secs(2)).await;
        client_sub.disconnect().await;
        debug!("client_sub disconnected");
        tokio::time::sleep(Duration::from_secs(2)).await;
        let data = serde_json::to_string(&json!({
            "title": "Test2",
            "data": [{
                "timestamp": 2932438,
                "value": 2
            }]
        })).unwrap();
        client_pro.share_data(serde_json::from_str(&data).unwrap()).await;
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}