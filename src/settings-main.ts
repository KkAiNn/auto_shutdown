import { createApp } from "vue";
import SettingsWindow from "./pages/SettingsWindow.vue";
import "./styles.css";
import { getCurrentWindow } from "@tauri-apps/api/window";

const app = createApp(SettingsWindow);
app.mount("#app");

async function showWindow() {
  try {
    const settingsWindow = getCurrentWindow();
    await settingsWindow.show();
  } catch (e) {
    console.log("Running in browser, skip window show");
  }
}

showWindow();
