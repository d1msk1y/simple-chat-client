const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;

async function getMessageById() {
  greetMsgEl.textContent = await invoke("get_message_by_id", {id: "0"});
}

async function getLastMessage() {
  greetMsgEl.textContent = await invoke("get_last_message");
}

async function getAllMessages() {
  greetMsgEl.textContent = await invoke("get_all_messages");
}

window.addEventListener("DOMContentLoaded", () => {
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => getMessages());
});
