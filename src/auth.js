const { invoke } = window.__TAURI__.tauri;

let loginInputEl;
let roomTokenInputEl;
let roomToken;

async function auth() {
  return await invoke("auth", {username: loginInputEl.value})
}

async function joinRoom() {
  let result = await auth();
  await invoke("join_room_by_token", { token: roomTokenInputEl.value });
  if (result === true){
    window.location.href = "index.html";

  }
}

async function newRoom() {
  let roomJSON = await invoke ("post_new_room");
  let parsedRoom = JSON.parse(roomJSON);
  roomToken = parsedRoom.code;
  roomId = parsedRoom.id;
  await auth();
}

window.addEventListener("DOMContentLoaded", () => {
  loginInputEl = document.querySelector("#login-input");
  roomTokenInputEl = document.querySelector('#room-code-input')
  document.querySelector("#create-room-button")
      .addEventListener("click", () => newRoom());

  document.querySelector("#join-room-button")
      .addEventListener("click", () => joinRoom());
});