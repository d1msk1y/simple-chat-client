#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use std::io::Read;
use std::net::ToSocketAddrs;
use std::time::Duration;
use reqwest::{Error, Url};
use serde::{Deserialize, ser, Serialize};
use serde_json::{json, Value};
use tauri::CursorIcon::Text;
use chrono;
use tungstenite::{connect, Message};
use web_sys;
use std::env;
use std::thread;
use tauri::Error::Runtime;
use tokio::runtime;
use tauri::Window;
use reqwest::header::USER_AGENT;

static SERVER_ADDRESS: &str = "http://localhost:8080";

#[derive(Serialize, Deserialize, Debug)]
struct MessageInfo {
    id: String,
    username: String,
    time: String,
    message: String
}

#[derive(Serialize, Deserialize, Debug)]
struct MessagePage {
    messages: String,
    pageSize: String,
    total: String
}

async fn get_request(endpoint: &str) -> Result<String, Error>{
    let token = env::var("CHATTOKEN")
        .unwrap_or_else(|err| {
            println!("Failed to retrieve token: {}", err);
            "".to_string() // Provide a default value or fallback action
        });

    let url = SERVER_ADDRESS.to_owned() + &endpoint;
    let response = reqwest::Client::new()
        .get(&url)
        .header("Token", token)
        .send()
        .await?
        .text()
        .await?;
     println!("{}", &url);
     println!("{}", response);
    Ok(format!("{}", response))
}

async fn post_json(endpoint: &str, json: &str) -> Result<(), Error>{
    let client = reqwest::Client::new();

    let json_sting = json.to_string();

    let response = client
        .post(SERVER_ADDRESS.to_owned() + endpoint)
        .header("Content-Type", "application/json")
        .body(json_sting)
        .send()
        .await?;

    Ok(())
}

async fn get_token() {
    let endpoint ="/token";
    let token = get_request(endpoint).await.unwrap();
    let key = "CHATTOKEN";
    env::set_var(key, &token);
    assert_eq!(env::var(key), Ok(token.to_string()));
}

#[tauri::command]
async fn send_message(message:&str) -> Result<(), String> {
    let last_message_json = get_last_message().await?;
    let last_message: MessageInfo = serde_json::from_str(last_message_json.as_str()).unwrap();

    println!("Last message index was: {}",last_message.id);

    let id: i32 = last_message.id.parse().unwrap();
    let m = MessageInfo {
        id: (id + 1).to_string(),
        username: "d1msk1y 1".to_string(),
        time: chrono::offset::Local::now().to_string(),
        message: message.to_string()
    };

    let endpoint = "/messages";
    let stringified_json = serde_json::to_string(&m).unwrap();
    post_json(endpoint, stringified_json.as_str()).await.map_err(|e|e.to_string())
}

#[tauri::command]
async fn ws_handshake() {
    get_token().await;
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
            get_message_by_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    handle.join().unwrap();
}
