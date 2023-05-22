#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod login;
mod models;
mod http_client;

use reqwest::{Error};
use serde::{Deserialize, Serialize};
use chrono;
use tungstenite::{connect, Message};
use std::env;
use std::thread;
use models::{MessageInfo, MessagePage};
use http_client::{get_request, post_json};

#[tauri::command]
async fn send_message(message:&str) -> Result<(), String> {
    let last_message_json = get_last_message().await?;
    let last_message: MessageInfo = serde_json::from_str(last_message_json.as_str()).unwrap();
    let id: i32;

    if last_message.id == ""{
        id = 0;
    } else {
        id = last_message.id.parse().unwrap();
    }

    let nickname = env::var("CHATNICKNAME")
        .unwrap_or_else(|err| {
            println!("Failed to retrieve nickname: {}", err);
            "".to_string() // Provide a default value or fallback action
        });

    let m = MessageInfo {
        id: (id + 1).to_string(),
        username: nickname,
        time: chrono::offset::Local::now().to_string(),
        message: message.to_string()
    };

    let stringified_json = serde_json::to_string(&m).unwrap();
    post_json("/messages", stringified_json.as_str()).await.map_err(|e|e.to_string())
}

#[tauri::command]
async fn ws_handshake() {
    let (mut socket, _) = connect("ws://localhost:8080/ws").expect("Failed to connect");
    loop {
        let message = socket.read_message().expect("Failed to receive message");
        if let Message::Text(json_message) = &message {

            let message: MessageInfo = serde_json::from_str(&json_message).unwrap();
            let message_formatted = serde_json::to_string_pretty(&message).unwrap();
            println!("Received message: {}", message_formatted);
        }
    }
}

#[tauri::command]
async fn get_message_by_id(id: &str) -> Result<String, String>
{
    let endpoint = "/messages/".to_owned() + id;
    get_request(endpoint.as_str()).await.map_err(|e| e.to_string())
}


#[tauri::command]
async fn get_message_by_page(id: &str) -> Result<String, String>
{
    let endpoint = "/messages/pages/".to_owned() + id;
    get_request(endpoint.as_str()).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_all_messages() -> Result<String, String> {
    let endpoint = "/messages";
    get_request(endpoint).await.map_err(|e|e.to_string())
}

#[tauri::command]
async fn get_last_message() -> Result<String, String> {
    let endpoint = "/messages/last";
    get_request(endpoint).await.map_err(|e|e.to_string())
}

#[tauri::command]
async fn auth(username: &str) -> Result<bool, bool> {
    login::auth(username).await
}

#[tauri::command]
fn get_env_var(name: String) -> Option<String> {
    env::var(name).ok()
}

#[tokio::main]
async fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let handle = thread::spawn(move || {
        rt.block_on(async {
            ws_handshake().await;
        });
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_message_by_id,
            get_all_messages,
            get_last_message,
            ws_handshake,
            send_message,
            get_message_by_page,
            auth,
            get_env_var
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    handle.join().unwrap();
}
