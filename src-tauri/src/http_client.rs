use std::collections::HashMap;
use reqwest::{Error};
use serde::{Deserialize, Serialize};
use chrono;
use tungstenite::{connect, Message};
use std::env;
use std::thread;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::get_env_var;
use crate::models::{MessageInfo, MessagePage};

pub static SERVER_ADDRESS: &str = "http://localhost:8080";

pub fn empty_headers() -> Option<HeaderMap> {
    None
}

pub async fn get_request(endpoint: &str, extra_headers: Option<HeaderMap>) -> Result<String, Error>{
    let token = get_env_var("CHATTOKEN".to_string());
    let room_id = get_env_var("ROOMID".to_string());

    let mut headers = HeaderMap::new();
    headers.insert("Token", HeaderValue::from_str(token.as_str()).unwrap());
    headers.insert("RoomID", HeaderValue::from_str(room_id.as_str()).unwrap());

    if let Some(extra_headers) = extra_headers{
        headers.extend(extra_headers.into_iter());
    }

    println!("{}", &token);
    let url = SERVER_ADDRESS.to_owned() + &endpoint;
    let response = reqwest::Client::new()
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    println!("Address: {}; Headers: {}", &url, room_id);
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