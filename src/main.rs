use std::sync::Arc;
use tokio::sync::mpsc;
use gtk::prelude::*;

mod gui;
mod chat;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    // Load the configuration
    let config = Arc::new(config::Config::new());

    // Initialize the GUI
    gui::run();

    // Start sending messages
    let (tx, mut rx) = mpsc::channel(10);
    tokio::spawn(async move {
        chat::start_sending_messages(tx).await;
    });

    // Start receiving messages
    tokio::spawn(async move {
        chat::start_receiving_messages().await;
    });

    // Process incoming messages
    let messages_box = Arc::new(gui::get_messages_box());
    let input_entry = Arc::new(gui::get_input_entry());
    let send_button = Arc::new(gui::get_send_button());

    while let Some(json) = rx.recv().await {
        let message: chat::Message = serde_json::from_str(&json).unwrap();
        let text = message.text.clone();

        // Add the message to the messages box
        let messages_box_clone = messages_box.clone();
        gtk::idle_add(move || {
            utils::add_message_to_box(&messages_box_clone, &text);
            Continue(false)
        });

        // Clear the input entry
        let input_entry_clone = input_entry.clone();
        gtk::idle_add(move || {
            utils::clear_input_entry(&input_entry_clone);
            Continue(false)
        });
    }

    // Send messages when the send button is clicked
    let config_clone = config.clone();
    let tx_clone = tx.clone();
    send_button.connect_clicked(move |_| {
        let input_entry_text = utils::get_text_from_entry(&*input_entry);
        let message = chat::Message { text: input_entry_text };
        let json = serde_json::to_string(&message).unwrap();
        let client = reqwest::Client::new();

        let config_clone = config_clone.clone();
        let tx_clone = tx_clone.clone();

        tokio::spawn(async move {
            match client.post(&config_clone.api_url).body(json).send().await {
                Ok(_) => tx_clone.send(json).await.unwrap(),
                Err(err) => println!("Failed to send message: {}", err),
            }
        });
    });

    gtk::main();
}
