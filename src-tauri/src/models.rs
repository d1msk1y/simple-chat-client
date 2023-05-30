use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageInfo {
    pub id: String,
    pub username: String,
    pub time: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePage {
    pub messages: String,
    pub pageSize: String,
    pub total: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub jwt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub id: String,
    pub code: String,
}