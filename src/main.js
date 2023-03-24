const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;

async function getMessages() {
  greetMsgEl.textContent = await invoke("get_messages");
}

window.addEventListener("DOMContentLoaded", () => {
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => getMessages());
});
