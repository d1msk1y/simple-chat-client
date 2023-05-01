use std::env;
use std::fmt::format;
use reqwest::Error;
use crate::{get_request, SERVER_ADDRESS};

pub async fn auth(username: &str) -> Result<bool,  bool> {
    let token = add_user(username).await.unwrap();
    let key = "CHATTOKEN";
    env::set_var(key, &token);
    assert_eq!(env::var(key), Ok(token.to_string()));
    if token != "" {
        Ok(true)
    } else {
        Ok(false)
    }
}

async fn add_user(username: &str) -> Result<String, Error>{
    let url = SERVER_ADDRESS.to_owned() + "/auth";
    let response = reqwest::Client::new()
        .get(&url)
        .header("Username", username)
        .send()
        .await?
        .text()
        .await?;
    println!("{}", &url);
    println!("{}", response);
    Ok(format!("{}", response))
}