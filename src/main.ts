import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";
import { getCurrentWindow } from "@tauri-apps/api/window";

const app = createApp(App);
app.mount("#app");

async function showWindow() {
  try {
    const mainWindow = getCurrentWindow();
    await mainWindow.show();
  } catch (e) {
    console.log("Running in browser, skip window show");
  }
  
  setTimeout(() => {
    const splash = document.getElementById("splash");
    if (splash) {
      splash.classList.add("hidden");
      setTimeout(() => splash.remove(), 300);
    }
  }, 200);
}

showWindow();
