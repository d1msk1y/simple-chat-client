use super::models::{MessageInfo, MessagePage};

use std::env;
use std::fmt::format;
use std::io::ErrorKind;
use reqwest::{Error, StatusCode};
use crate::{get_request, SERVER_ADDRESS};
use crate::models::User;

pub async fn auth(username: &str) -> Result<bool,  bool> {
    let user_string = add_user(username).await.unwrap();
    let user: User = serde_json::from_str(user_string.as_str()).unwrap();

    let username = user.username;
    let token = user.jwt;

    let key = "CHATTOKEN";
    env::set_var(key, &token);
    assert_eq!(env::var(key), Ok(token));

    let key = "CHATNICKNAME";
    env::set_var(key, &username);
    assert_eq!(env::var(key), Ok(username));

    if user_string != ""{
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
        Ok(format!("{}", "Error occurred!"))
    }

}