const { invoke } = window.__TAURI__.tauri;

let loginInputEl;
let roomCodeInputEl;
let roomCode;
let roomId;

async function auth(){
  await invoke("auth", {username: loginInputEl.value})
  window.location.href = "index.html";
}

async function joinRoom() {
  await auth();
  await invoke ("join_room_by_code", {join_code: roomCodeInputEl.value});
}

async function newRoom() {
  let roomJSON = await invoke ("post_new_room");
  let parsedRoom = JSON.parse(roomJSON);
  roomCode = parsedRoom.code;
  roomId = parsedRoom.id;
  await auth();
}

window.addEventListener("DOMContentLoaded", () => {
  loginInputEl = document.querySelector("#login-input");
  roomCodeInputEl = document.querySelector('#room-code-input')
  document.querySelector("#create-room-button")
      .addEventListener("click", () => newRoom());

  document.querySelector("#join-room-button")
      .addEventListener("click", () => joinRoom());
});