const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;
let messageInputEl;

const getMessageById = messageGetter("get_message_by_id", {id: "0"});
const getLastMessage = messageGetter("get_last_message");
const getAllMessages = messageGetter("get_all_messages");

window.onload = async function (){
  await PrintMessagePage("0");
}

let messagePageIndex = 0;
let lastMessageId = 0;

function messageGetter(methodName, params = {}) {
  return async function(){
    const json = await invoke(methodName, params);
    return JSON.parse(json);
  }
}

function createMessageBox(message){
  const messageBoxHTML = `
      <div style="background-color: #e1e1e1; border-radius: 10px; padding: 10px; max-width: 300px;">
        <p style="font-size: 12px; margin: 0; color: #4b4b4b;">${message.username}</p>
        <p style="font-size: 14px; margin: 0;">${message.message}</p>
        <p style="font-size: 12px; margin: 0; color: #7a7a7a;">Sent at ${message.time}</p>
      </div>
    `;
  return document.createRange().createContextualFragment(messageBoxHTML).firstElementChild;
}

async function printMessage(message){
  let messageBox = createMessageBox(message);
  let messagePanel = document.getElementById("message-panel");

  let messageId = parseInt(message.id);
  if (messageId >= lastMessageId){
    lastMessageId = messageId;
    messagePanel.append(messageBox);
    window.scrollTo(0, document.body.scrollHeight);
  } else {
    messagePanel.insertBefore(messageBox,messagePanel.firstChild);
  }
  document.querySelector("#debug-text").textContent = lastMessageId.toString();
}

async function printLastMessage() {
  await printMessage(await getLastMessage());
}

async function sendMessage(){
  await invoke("send_message", {message: messageInputEl.value});
  messageInputEl.value = "";
}

async function PrintMessagePage(id){
  const json = await invoke("get_message_by_page", {id: id});
  let parse = JSON.parse(json);
  let items = parse.messages;

  for (let i = 0; i <= Object.keys(items).length;){
    await printMessage(items[i]);
    i++;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-button")
      .addEventListener("click", () => printLastMessage());

  messageInputEl = document.querySelector("#login-input");
  document.querySelector("#send-message-button")
      .addEventListener("click", () => sendMessage());
});

window.onwheel = async e => {
  if (e.deltaY < 0 && window.scrollY == 0) {
    let indexString = (messagePageIndex + 1).toString()
    await PrintMessagePage(indexString);
  }
}