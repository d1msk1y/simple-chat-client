use std::env;
use std::fmt::format;
use std::io::ErrorKind;
use reqwest::{Error, StatusCode};
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
        .await?;

    println!("{}", &url);
    if response.status().is_success() {
        let response_body = response.text().await?;
        println!("{}", response_body);
        Ok(format!("{}", response_body))
    } else {
        return Err(e)
    }

}