#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Error;
use tauri::CursorIcon::Text;

async fn get_request() -> Result<String, Error>{
    let response = reqwest::get("http://localhost:8080/messages")
        .await?
        .text()
        .await?;
    println!("{}", response);
    Ok(format!("{}", response))
}

#[tauri::command]
async fn get_messages() -> Result<String, String>{
    get_request().await.map_err(|e| e.to_string())
}

fn main() {
    get_messages();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_messages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
