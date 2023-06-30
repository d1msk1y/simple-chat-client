# Chat-Client
![](https://img.shields.io/github/repo-size/d1msk1y/simple-chat-client)
![](https://img.shields.io/github/last-commit/d1msk1y/simple-chat-client/main)

Simple-Chat-Client is a server side chat application built on Tauri framework, powered with Rust backend and with JS/HTML/CSS frontend. All data entries are stored in the Database and if needed can be accessed with exposed API endpoints.

## API

Client uses RESTful HTTP endpoints exposed on the [dedicated HTTP server](https://github.com/d1msk1y/sipmle-go-chat-server) written in go.

Each HTTP request has to possess so-called `security_headers`, they are nothing but some generic identifiers actually:
```rust
pub async fn get_request(endpoint: &str, extra_headers: Option<HeaderMap>) -> Result<String, Error>{
    let headers = merge_headers(security_headers(), extra_headers).unwrap_or_default();

    let url = SERVER_ADDRESS.to_owned() + &endpoint;
    let response = reqwest::Client::new()
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    Ok(format!("{}", response))
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
```
However, it's worth noting, that the client also has the capability of assigning `extra_headers`, which sure does increase the modularity of the code

### Endpoints
The most crucial endpoints in use:
``` 
GET    /ws
GET    /auth

GET    /messages/:id           
GET    /messages/pages/:page

GET    /rooms/new 
POST   /rooms/join
GET    /rooms/token/:token
GET    /rooms/users/:token
```

## WebSocket

To ensure real-time dual-way communication between client and the server, it was decided to bind both sides over WebSocket networking protocol using the lightweight stream-based WebSocket implementation [tungstenite](https://github.com/snapview/tungstenite-rs). 
```rust
use tungstenite::{connect, Message, WebSocket};

// Establish WebSocket connection
async fn ws_handshake() {
    let (mut socket, _) = connect("ws://localhost:8080/ws").expect("Failed to connect");
    loop {
        // Expect incoming requests
        expect_message(&mut socket);
    }
}


fn expect_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let message = socket.read_message().expect("Failed to receive message");
    if let Message::Text(json_message) = &message {
        let message: MessageInfo = serde_json::from_str(&json_message).unwrap();
        let message_formatted = serde_json::to_string_pretty(&message).unwrap();
    }
}
```

## Multi-Room

This chat app does support a multi-room feature, which is obviously a server-side implementation, but still has some interesting things going on, here, on the client side.

### Data-Caching

First things first, created/joined room credentials got to be stored.  
This is how it looks like:
```rust
fn cache_room(room: Room) {
    let room_token = room.token;
    let key = "ROOMTOKEN";
    
    env::set_var(key, &room_token);
    assert_eq!(env::var(key), Ok(room_token));
}
```
Basically it's just being stored in an environmental variable, which is not the safest way of course, but hey, ain't really chasing safety here:)

### Creating
Well, talking about room-creating logic here, you won't really see anything worth seeing (server-side, ladies and gentlemen), but here we go:
```rust
pub async fn create_new_room() -> String {
    let endpoint = "/rooms/new";
    
    // This is where the client receives a newly created room token. 
    // Can't tell if it would've been better off as a POST request though.
    let response = get_request(endpoint, empty_headers()).await.map_err(|e|e.to_string()).unwrap();
    
    let new_room: Room = serde_json::from_str(response.as_str()).expect("JSON was not well-formatted");
    cache_room(new_room);
    response
}
```

### Joining

When joining a room, the client does nothing, but just retrieves the relevant room with a known user-provided room token and cache it into an environmental variable: 

```rust
pub async fn join_room(token: &str) {
    let endpoint = "/rooms/token/".to_owned() + token;
    let response_room = get_request(endpoint.as_str(), empty_headers()).await.map_err(|e|e.to_string()).unwrap();
    let joined_room: Room = serde_json::from_str(response_room.as_str()).expect("JSON was not well-formatted");
    cache_room(joined_room);
}
```
That's not it though, now the client has to 'tag' the user with the earlier retrieved token:
```rust
//Some extra headers here
let mut headers = HeaderMap::new();
let username = get_env_var("CHATNICKNAME");
headers.insert("Username", username.to_string().parse().unwrap());

post_request("/rooms/join", Option::from(headers)).await.unwrap();
```