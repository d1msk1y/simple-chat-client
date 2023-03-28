const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;

function createMessageGetter(methodName, params = {}) {
  return async function(){
    const json = await invoke(methodName, params);
    greetMsgEl.textContent = json.message;
    return JSON.parse(json);
  }
}

const getMessageById = createMessageGetter("get_message_by_id", {id: "0"});
const getLastMessage = createMessageGetter("get_last_message");
const getAllMessages = createMessageGetter("get_all_messages");

async function printLastMessage() {
  const message = await getLastMessage();
  greetMsgEl.textContent = message.message;
}

window.addEventListener("DOMContentLoaded", () => {
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => printLastMessage());
});
