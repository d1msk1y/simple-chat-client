use std::env;
use reqwest::Error;
use crate::http_client::{get_request, SERVER_ADDRESS};
use crate::models::Room;

pub async fn create_new_room() -> String {
    let endpoint = "/rooms/new";
    let response = get_request(endpoint).await.map_err(|e|e.to_string()).unwrap();
    let new_room: Room = serde_json::from_str(response.as_str()).expect("JSON was not well-formatted");

    let key = "ROOMCODE";
    env::set_var(key, &new_room.code);
    assert_eq!(env::var(key), Ok(new_room.code));
    response
}