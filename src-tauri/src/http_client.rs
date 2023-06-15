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

pub fn security_headers() -> Option<HeaderMap> {
    let token = get_env_var("CHATTOKEN".to_string());
    let room_id = get_env_var("ROOMID".to_string());

    let mut headers = HeaderMap::new();
    headers.insert("Token", HeaderValue::from_str(token.as_str()).unwrap());
    headers.insert("RoomID", HeaderValue::from_str(room_id.as_str()).unwrap());
    Option::from(headers)
}

fn merge_headers(mut headers: Option<HeaderMap>, extra_headers: Option<HeaderMap>) -> Option<HeaderMap> {
    if let Some(extra_headers) = extra_headers {
        headers.as_mut().unwrap().extend(extra_headers.into_iter());
    }
    headers
}

pub async fn get_request(endpoint: &str, extra_headers: Option<HeaderMap>) -> Result<String, Error>{
    let headers = merge_headers(security_headers(), extra_headers);

    let url = SERVER_ADDRESS.to_owned() + &endpoint;
    let response = reqwest::Client::new()
        .get(&url)
        .headers(headers.unwrap())
        .send()
        .await?
        .text()
        .await?;
    println!("{}", response);
    Ok(format!("{}", response))
}

pub async fn post_json(endpoint: &str, json: &str) -> Result<(), Error>{
    let client = reqwest::Client::new();

    let _response = client
        .post(SERVER_ADDRESS.to_owned() + endpoint)
        .header("Content-Type", "application/json")
        .body(json.to_string())
        .send()
        .await?;

    Ok(())
}

pub async fn post_request(endpoint: &str, extra_headers: Option<HeaderMap>) -> Result<(), Error>{
    let client = reqwest::Client::new();
    let headers = merge_headers(security_headers(), extra_headers);

    let _response = client
        .post(SERVER_ADDRESS.to_owned() + endpoint)
        .headers(headers.unwrap())
        .send()
        .await?;

    Ok(())
}