const { invoke } = window.__TAURI__.tauri;

let loginInputEl;
let roomCodeInputEl;
let roomCode;

async function auth(){
  await invoke("auth", {username: loginInputEl.value})
  window.location.href = "index.html";
}

async function joinRoom() {
  await auth();
}

async function newRoom() {
  roomCode = await invoke ("post_new_room");
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