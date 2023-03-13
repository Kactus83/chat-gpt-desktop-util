use tokio::sync::mpsc::{self, Sender};
use tokio::time::{self, Duration};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
}

pub async fn start_sending_messages(tx: Sender<String>) {
    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;
        let message = Message { text: "Hello from Rust!".to_owned() };
        let json = serde_json::to_string(&message).unwrap();
        tx.send(json).await.unwrap();
    }
}

pub async fn start_receiving_messages() {
    let client = Client::new();

    loop {
        let res = client.get("https://api.example.com/messages").send().await;

        match res {
            Ok(res) => {
                let text = res.text().await.unwrap();
                let messages: Vec<Value> = serde_json::from_str(&text).unwrap();

                for message in messages {
                    println!("Received message: {}", message);
                }
            }
            Err(err) => {
                println!("Failed to receive messages: {}", err);
            }
        }

        time::delay_for(Duration::from_secs(5)).await;
    }
}
