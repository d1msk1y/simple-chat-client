use reqwest::{Error};
use serde::{Deserialize, Serialize};
use chrono;
use tungstenite::{connect, Message};
use std::env;
use std::thread;
use crate::models::{MessageInfo, MessagePage};

pub static SERVER_ADDRESS: &str = "http://localhost:8080";

pub async fn get_request(endpoint: &str) -> Result<String, Error>{
    let token = env::var("CHATTOKEN")
        .unwrap_or_else(|err| {
            println!("Failed to retrieve token: {}", err);
            "".to_string() // Provide a default value or fallback action
        });

    println!("{}", &token);
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

pub async fn post_json(endpoint: &str, json: &str) -> Result<(), Error>{
    let client = reqwest::Client::new();
    let json_sting = json.to_string();

    let _response = client
        .post(SERVER_ADDRESS.to_owned() + endpoint)
        .header("Content-Type", "application/json")
        .body(json_sting)
        .send()
        .await?;

    Ok(())
}