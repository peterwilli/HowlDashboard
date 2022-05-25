#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::time::Duration;
    use async_std::task;
    use log::debug;
    use rand::{random, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    use crate::server::Server;
    use test_log::test;
    use tokio::sync::mpsc::channel;
    use url::Url;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use crate::client::Client;
    use crate::structs::{DataStore, InitCommandType, UniversalNumber};

    #[test]
    fn test_un_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            number: UniversalNumber
        }
        fn test_shot(num: &str) {
            let un = UniversalNumber::from_str(num).unwrap();
            let test_struct = TestStruct {
                number: un
            };
            let json = serde_json::to_string(&test_struct).unwrap();
            debug!("json: {}", json);
            let test_struct_de: TestStruct = serde_json::from_str(&json).unwrap();
            assert_eq!(test_struct_de.number, un);
        }
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(80085129);
        for _ in 0..100 {
            let test_num: i32 = rng.gen();
            test_shot(&format!("{}", test_num));
            let test_num: f32 = rng.gen();
            test_shot(&format!("{}", test_num));
        }
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

        let (on_data_tx, mut on_data_rx) = channel(16);
        let mut client_sub = Client::as_subscriber(on_data_tx);
        tokio::spawn(async move {
            loop {
                let event = on_data_rx.recv().await.unwrap();
                debug!("data store event: {}", serde_json::to_string(&event).unwrap());
            }
        });
        client_sub.connect(server_url.clone()).await;

        let mut client_pro = Client::as_provider();
        client_pro.connect(server_url.clone()).await;
        tokio::time::sleep(Duration::from_secs(2)).await;

        let data = r#"
        {
            "category": "test_%",
            "number": 203
        }
        "#;
        debug!("Sharing data: {}", data);
        client_pro.share_data(serde_json::from_str(data).unwrap()).await;
        tokio::time::sleep(Duration::from_secs(10)).await;
    }

    #[test]
    fn universal_number_test() {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(80085128);
        for _ in 0..100 {
            let test_num: i32 = rng.gen();
            debug!("Testing with {}", test_num);
            let test_unum = UniversalNumber::from_str(format!("{}", test_num).as_str()).unwrap();
            assert_eq!(test_unum.n.unwrap(), test_num as i64);

            // Adding another int
            let test_num2: i32 = rng.gen();
            let added_unum = test_unum + UniversalNumber::from_str(format!("{}", test_num2).as_str()).unwrap();
            debug!("Testing added unum: {} + {} = {}", test_num, test_num2, added_unum);
            assert_eq!(added_unum.n.unwrap(), ((test_num as i64) + (test_num2 as i64)) as i64);

            // Adding a float...
            let test_float: f32 = rng.gen();
            let added_unum = test_unum + UniversalNumber::from_str(format!("{}", test_float).as_str()).unwrap();
            debug!("Testing added unum: {} + {} = {}", test_num, test_num2, added_unum);
            let epsilon = 1e-6;
            assert!((added_unum.f.unwrap() - ((test_num as f64) + (test_float as f64)) as f64) < epsilon);
        }
    }

    #[test(tokio::test)]
    async fn datastore_test() {
        let (event_tx, mut event_rx) = channel(16);
        let mut data_store = DataStore::new(event_tx);
        tokio::spawn(async move {
            let event = event_rx.recv().await.unwrap();
            println!("Event: {:#?}", event);
            assert_eq!(event.categorical_number_data.unwrap().get("test_%").unwrap(), &UniversalNumber::from_str("203").unwrap());
            let event = event_rx.recv().await.unwrap();
            println!("Event: {:#?}", event);
            assert_eq!(event.categorical_number_data.unwrap().get("test2_%").unwrap(), &UniversalNumber::from_str("12").unwrap());
            let event = event_rx.recv().await.unwrap();
            println!("Event: {:#?}", event);
            assert_eq!(event.categorical_number_data.unwrap().get("test2_%").unwrap(), &UniversalNumber::from_str("995.3").unwrap());
            let event = event_rx.recv().await.unwrap();
            println!("Event: {:#?}", event);
            assert_eq!(event.categorical_number_data.unwrap().get("test2_%").unwrap(), &UniversalNumber::from_str("996.3").unwrap());
        });
        let data = r#"
        {
            "category": "test_%",
            "number": 203
        }
        "#;
        data_store.add_entry("1".to_string(), serde_json::from_str(data).as_ref().unwrap()).await;
        task::sleep(Duration::from_secs(2)).await;
        let data = r#"
        {
            "category": "test2_%",
            "number": "12"
        }
        "#;
        data_store.add_entry("1".to_string(), serde_json::from_str(data).as_ref().unwrap()).await;
        task::sleep(Duration::from_secs(2)).await;
        debug!("Testing adding");
        let data = r#"
        {
            "category": "test2_%",
            "number": 983.3
        }
        "#;
        data_store.add_entry("2".to_string(), serde_json::from_str(data).as_ref().unwrap()).await;
        task::sleep(Duration::from_secs(2)).await;
        debug!("Replace 12 from former node");
        let data = r#"
        {
            "category": "test2_%",
            "number": 13
        }
        "#;
        data_store.add_entry("1".to_string(), serde_json::from_str(data).as_ref().unwrap()).await;
        task::sleep(Duration::from_secs(2)).await;
    }
}