#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::time::Duration;

    use async_std::task;
    use log::debug;
    use rand::{Rng, SeedableRng};
    use serde::{Deserialize, Serialize};
    use test_log::test;
    use tokio::sync::mpsc::channel;
    use url::Url;

    use crate::client::Client;
    use crate::data_store::{DataStore, DataStoreEvent};
    use crate::server::Server;
    use crate::structs::{InitCommandType, UniversalNumber};

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

        let data = r#"
        {
            "title": "Portfolio (across bots)",
            "data": {
                "FRONT": [{
                    "value": "172.3033304100000000",
                    "suffix": "FRONT"
                }, {
                    "value": "44.204419416685500000000000",
                    "suffix": "$"
                }]
            }
        }
        "#;
        debug!("Sharing data: {}", data);
        client_pro.share_data(serde_json::from_str(data).unwrap()).await;
        debug!("testing disconnect and sending data (is client cleaned up from the server?)");
        tokio::time::sleep(Duration::from_secs(2)).await;
        client_sub.disconnect().await;
        debug!("client_sub disconnected");
        tokio::time::sleep(Duration::from_secs(2)).await;
        client_pro.share_data(serde_json::from_str(data).unwrap()).await;
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
            let float_unum = UniversalNumber::from_str(format!("{}", test_float).as_str()).unwrap();
            assert_eq!(float_unum.f.unwrap() as f32, test_float);
            let added_unum = test_unum + float_unum;
            debug!("Testing added unum: {} + {} = {}", test_num, float_unum, added_unum);
            let epsilon = 1e-6;
            assert!((added_unum.f.unwrap() - ((test_num as f64) + (test_float as f64)) as f64) < epsilon);
        }
    }

    #[test(tokio::test)]
    async fn datastore_test() {
        let (event_tx, mut event_rx) = channel(16);
        let mut data_store = DataStore::new(event_tx);
        tokio::spawn(async move {
            fn test_number_from_event(event: &DataStoreEvent, title: &str, number_str: &str, name: &str) {
                assert_eq!(&event.categorical_number_data.as_ref().unwrap().get(title).unwrap().get(name).unwrap().number, &UniversalNumber::from_str(number_str).unwrap());
            }

            let event = event_rx.recv().await.unwrap();
            println!("Event: {:#?}", event);
            test_number_from_event(&event, "Portfolio (across bots)", "120", "Bitcoin");
            let event = event_rx.recv().await.unwrap();
            println!("Event: {:#?}", event);
            test_number_from_event(&event, "Portfolio (across bots)", "120", "Bitcoin");
            test_number_from_event(&event, "Portfolio (across bots)", "29", "Ethereum");
            let event = event_rx.recv().await.unwrap();
            println!("Event: {:#?}", event);
            test_number_from_event(&event, "Portfolio (across bots)", "41", "Ethereum");
            let event = event_rx.recv().await.unwrap();
            println!("Event: {:#?}", event);
            test_number_from_event(&event, "Portfolio (across bots)", "25", "Ethereum");
        });
        let data = r#"
        {
            "title": "Portfolio (across bots)",
            "data": {
                "Bitcoin": [
                    { "value": "120", "suffix": "btc" },
                    { "value": "12000", "suffix": "$" }
                ]
            }
        }
        "#;
        data_store.add_entry("1".to_string(), serde_json::from_str(data).as_ref().unwrap()).await;
        task::sleep(Duration::from_secs(2)).await;
        let data = r#"
        {
            "title": "Portfolio (across bots)",
            "data": {
                "Ethereum": [
                    { "value": "29", "suffix": "eth" },
                    { "value": "2934", "suffix": "$" }
                ]
            }
        }
        "#;
        data_store.add_entry("1".to_string(), serde_json::from_str(data).as_ref().unwrap()).await;
        task::sleep(Duration::from_secs(2)).await;
        debug!("Testing adding");
        let data = r#"
        {
            "title": "Portfolio (across bots)",
            "data": {
                "Ethereum": [
                    { "value": "12", "suffix": "eth" },
                    { "value": "20", "suffix": "$" }
                ]
            }
        }
        "#;
        data_store.add_entry("2".to_string(), serde_json::from_str(data).as_ref().unwrap()).await;
        task::sleep(Duration::from_secs(2)).await;
        debug!("Replace 12 from former node");
        let data = r#"
        {
            "title": "Portfolio (across bots)",
            "data": {
                "Ethereum": [
                    { "value": "13", "suffix": "eth" },
                    { "value": "21", "suffix": "$" }
                ]
            }
        }
        "#;
        data_store.add_entry("1".to_string(), serde_json::from_str(data).as_ref().unwrap()).await;
        task::sleep(Duration::from_secs(2)).await;
    }
}