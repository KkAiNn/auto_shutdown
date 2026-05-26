import { ref, computed, onMounted } from 'vue';
import { Store } from '@tauri-apps/plugin-store';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';

export interface AppSettings {
  autoHideToTray: boolean;
  autoExecutePreset: boolean;
  presetMinutes: number | null;
}

const SETTINGS_KEY = 'app_settings';

// Singleton state
const globalAutoStartEnabled = ref(false);
const globalSettings = ref<AppSettings>({
  autoHideToTray: false,
  autoExecutePreset: false,
  presetMinutes: null,
});
const globalSettingsInitialized = ref(false);
let globalStore: Store | null = null;

const initGlobalStore = async () => {
  if (!globalStore) {
    globalStore = await Store.load('settings.json');
  }
};

const loadGlobalSettings = async () => {
  await initGlobalStore();
  if (globalStore) {
    const saved = await globalStore.get<AppSettings>(SETTINGS_KEY);
    if (saved) {
      globalSettings.value = saved;
    }
  }
  try {
    globalAutoStartEnabled.value = await isEnabled();
  } catch {
    globalAutoStartEnabled.value = false;
  }
  globalSettingsInitialized.value = true;
};

const saveGlobalSettings = async (newSettings: Partial<AppSettings>) => {
  await initGlobalStore();
  globalSettings.value = { ...globalSettings.value, ...newSettings };
  if (globalStore) {
    await globalStore.set(SETTINGS_KEY, globalSettings.value);
    await globalStore.save();
  }
};

export function useSettings() {
  const autoStartEnabled = globalAutoStartEnabled;
  const settings = globalSettings;
  const initialized = globalSettingsInitialized;

  const loadSettings = async () => {
    await loadGlobalSettings();
  };

  const setAutoStart = async (enabled: boolean) => {
    try {
      if (enabled) {
        await enable();
        console.log('Autostart registered successfully');
      } else {
        await disable();
        console.log('Autostart unregistered successfully');
      }
      globalAutoStartEnabled.value = enabled;
    } catch (e) {
      console.error('Failed to set autostart:', e);
      alert('设置开机自启失败: ' + (e as Error)?.message);
    }
  };

  const setAutoExecutePreset = async (enabled: boolean) => {
    await saveGlobalSettings({ autoExecutePreset: enabled });
  };

  const setPresetMinutes = async (minutes: number | null) => {
    await saveGlobalSettings({ presetMinutes: minutes });
  };

  const setAutoHideToTray = async (enabled: boolean) => {
    await saveGlobalSettings({ autoHideToTray: enabled });
  };

  const autoHideToTray = computed(() => globalSettings.value.autoHideToTray);

  const saveSettings = async (autoHide?: boolean) => {
    // Backward compatible: if autoHide passed, persist it
    if (typeof autoHide === "boolean") {
      await saveGlobalSettings({ autoHideToTray: autoHide });
    }
  };

  onMounted(() => {
    if (!initialized.value) {
      loadSettings();
    }
  });

  return {
    autoStartEnabled,
    autoHideToTray,
    settings,
    initialized,
    loadSettings,
    setAutoStart,
    setAutoExecutePreset,
    setPresetMinutes,
    setAutoHideToTray,
    saveSettings,
  };
}
