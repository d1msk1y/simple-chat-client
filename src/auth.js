const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;
let greetInputEl;

const getMessageById = messageGetter("get_message_by_id", {id: "0"});
const getLastMessage = messageGetter("get_last_message");
const getAllMessages = messageGetter("get_all_messages");

window.onload = async function (){
  await PrintMessagePage("0");
}

window.addEventListener("DOMContentLoaded", () => {
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-button")
      .addEventListener("click", () => printLastMessage());

  greetInputEl = document.querySelector("#login-input");
  document.querySelector("#send-message-button")
      .addEventListener("click", () => sendMessage());
});