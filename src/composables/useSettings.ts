import { ref, onMounted } from 'vue';
import { Store } from '@tauri-apps/plugin-store';

const SETTINGS_KEY = 'autoHideToTray';

export function useSettings() {
  const autoHideToTray = ref(false);
  let store: Store | null = null;

  const initStore = async () => {
    if (!store) {
      store = await Store.load('settings.json');
    }
  };

  const loadSettings = async () => {
    await initStore();
    if (store) {
      const value = await store.get<boolean>(SETTINGS_KEY);
      if (value !== null) {
        autoHideToTray.value = value;
      }
    }
  };

  const saveSettings = async (value: boolean) => {
    await initStore();
    if (store) {
      await store.set(SETTINGS_KEY, value);
      await store.save();
      autoHideToTray.value = value;
    }
  };

  onMounted(() => {
    loadSettings();
  });

  return {
    autoHideToTray,
    loadSettings,
    saveSettings,
  };
}
