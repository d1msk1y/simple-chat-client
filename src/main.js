const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;

function createMessageGetter(methodName, params = {}) {
  return async function(){
    const json = await invoke(methodName, params);
    greetMsgEl.textContent = json.message;
    return JSON.parse(json);
  }
}

function createMessageBox(message, nickname){
  const timestamp = new Date().toLocaleTimeString();
  const messageBoxHTML = `
      <div style="background-color: #f2f2f2; border-radius: 10px; padding: 10px; max-width: 300px;">
        <p style="font-size: 14px; margin: 0;">${message}</p>
        <p style="font-size: 12px; margin: 0; color: #8c8c8c;">Sent at ${timestamp}</p>
      </div>
    `;
  return document.createRange().createContextualFragment(messageBoxHTML).firstElementChild;
}


const getMessageById = createMessageGetter("get_message_by_id", {id: "0"});
const getLastMessage = createMessageGetter("get_last_message");
const getAllMessages = createMessageGetter("get_all_messages");

async function printLastMessage() {
  const message = await getLastMessage();
  const messageBox = createMessageBox(message.message, message.nickname)
  document.body.appendChild(messageBox);
}

window.addEventListener("DOMContentLoaded", () => {
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => printLastMessage());
});
