use std::env;
use reqwest::Error;
use crate::http_client::{get_request, SERVER_ADDRESS};
use crate::models::Room;

fn cache_room(room: Room) {
    let room_code = room.code;
    let room_id = room.id;

    println!("{}", &room_id);
    println!("{}", &room_code);

    let key = "ROOMCODE";
    env::set_var(key, &room_code);
    assert_eq!(env::var(key), Ok(room_code));

    let key = "ROOMID";
    env::set_var(key, &room_id);
    assert_eq!(env::var(key), Ok(room_id));
}

pub async fn create_new_room() -> String {
    let endpoint = "/rooms/new";
    let response = get_request(endpoint).await.map_err(|e|e.to_string()).unwrap();
    let new_room: Room = serde_json::from_str(response.as_str()).expect("JSON was not well-formatted");
    cache_room(new_room);
    response
}

pub async fn join_room(join_code: String) -> String {
    let endpoint = "/rooms/code/".to_owned() + join_code.as_str();
    let response = get_request(endpoint.as_str()).await.map_err(|e|e.to_string()).unwrap();
    let joined_room: Room = serde_json::from_str(response.as_str()).expect("JSON was not well-formatted");
    cache_room(joined_room);


    response
}