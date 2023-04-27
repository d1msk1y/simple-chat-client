use std::env;
use crate::get_request;

pub async fn auth() {
    let endpoint ="/token";
    let token = get_request(endpoint).await.unwrap();
    let key = "CHATTOKEN";
    env::set_var(key, &token);
    assert_eq!(env::var(key), Ok(token.to_string()));
}