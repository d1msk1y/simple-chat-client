#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use std::io::Read;
use std::net::ToSocketAddrs;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::CursorIcon::Text;

async fn get_request(endpoint: &str) -> Result<String, Error>{
    let url = "http://localhost:8080".to_owned() + &endpoint;
    let response = reqwest::get(&url)
        .await?
        .text()
        .await?;
    println!("{}", &url);
    Ok(format!("{}", response))
}

#[tauri::command]
async fn get_message_by_id(id: &str) -> Result<String, String>
{
    let endpoint = "/messages/".to_owned() + id;
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_message_by_id,
            get_all_messages,
            get_last_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
