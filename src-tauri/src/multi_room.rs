use std::env;
use reqwest::Error;
use tauri::http::header::{HeaderMap, HeaderValue};
use crate::get_env_var;
use crate::http_client::{get_request, SERVER_ADDRESS, empty_headers, post_json, post_request};
use crate::models::Room;

fn cache_room(room: Room) {
    let room_token = room.token;
    println!("room token: {}", &room_token);

    let key = "ROOMTOKEN";
    env::set_var(key, &room_token);
    assert_eq!(env::var(key), Ok(room_token));
}

pub async fn create_new_room() -> String {
    let endpoint = "/rooms/new";
    let response = get_request(endpoint, empty_headers()).await.map_err(|e|e.to_string()).unwrap();
    let new_room: Room = serde_json::from_str(response.as_str()).expect("JSON was not well-formatted");
    cache_room(new_room);
    response
}

pub async fn join_room(token: &str) -> String {
    let endpoint = "/rooms/token/".to_owned() + token;
    let response_room = get_request(endpoint.as_str(), empty_headers()).await.map_err(|e|e.to_string()).unwrap();
    let joined_room: Room = serde_json::from_str(response_room.as_str()).expect("JSON was not well-formatted");
    cache_room(joined_room);

    let mut headers = HeaderMap::new();
    let username = get_env_var("CHATNICKNAME".to_string());
    headers.insert("Username", username.to_string().parse().unwrap());
    let response = post_request("/rooms/join", Option::from(headers));

    response_room
}