#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use std::io::Read;
use std::net::ToSocketAddrs;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::CursorIcon::Text;

async fn get_request(command: &str) -> Result<String, Error>{
    let url = "http://localhost:8080".to_owned() + &command;
    let response = reqwest::get("")
        .await?
        .text()
        .await?;
    // println!("{}", response);
    Ok(format!("{}", response))
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: String,
    username: String,
    time: String,
    message: String,
}

#[tauri::command]
async fn get_messages() -> Result<Message, Message>
{
    get_message_by_id().await
}

async fn get_message_by_id() -> Result<Message, Message>
{
    let input = r#"{
    "id": "1",
    "username": "d1msk1y 2",
    "time": "00:01",
    "message": "Hellow d1msk1y!"
    }"#;

    let message: Message = serde_json::from_str(input).unwrap();

    println!("{}", message.message);
    Ok(message)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_messages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
