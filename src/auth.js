const { invoke } = window.__TAURI__.tauri;

let loginInputEl;

async function auth(){
  await invoke("auth", {username: loginInputEl.value})
  window.location.href = "index.html";
}

window.addEventListener("DOMContentLoaded", () => {
  loginInputEl = document.querySelector("#login-input");
  document.querySelector("#auth-button")
      .addEventListener("click", () => auth());
});