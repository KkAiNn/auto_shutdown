import { ref, computed, onUnmounted } from 'vue';
import { Store } from '@tauri-apps/plugin-store';

export interface Preset {
  id: string;
  minutes: number;
  label: string;
}

const PRESETS_KEY = 'shutdown_presets';

const defaultPresets: Preset[] = [
  { id: '1', minutes: 15, label: '15分钟' },
  { id: '2', minutes: 30, label: '30分钟' },
  { id: '3', minutes: 45, label: '45分钟' },
  { id: '4', minutes: 60, label: '1小时' },
  { id: '5', minutes: 90, label: '1小时30分钟' },
  { id: '6', minutes: 120, label: '2小时' },
  { id: '7', minutes: 180, label: '3小时' },
  { id: '8', minutes: 240, label: '4小时' },
];

// Singleton state shared across all instances
const globalPresets = ref<Preset[]>([]);
const globalInitialized = ref(false);
let globalStore: Store | null = null;
let subscriberCount = 0;
let storeChangeHandler: (() => void) | null = null;

const initGlobalStore = async () => {
  if (!globalStore) {
    globalStore = await Store.load('presets.json');
  }
};

const loadGlobalPresets = async () => {
  await initGlobalStore();
  if (globalStore) {
    const saved = await globalStore.get<Preset[]>(PRESETS_KEY);
    if (saved && saved.length > 0) {
      globalPresets.value = saved;
    } else {
      globalPresets.value = [...defaultPresets];
    }
  } else {
    globalPresets.value = [...defaultPresets];
  }
  globalInitialized.value = true;
};

const saveGlobalPresets = async (newPresets: Preset[]) => {
  await initGlobalStore();
  globalPresets.value = newPresets;
  if (globalStore) {
    await globalStore.set(PRESETS_KEY, newPresets);
    await globalStore.save();
  }
  // Notify any listeners that store changed
  if (storeChangeHandler) {
    storeChangeHandler();
  }
};

export function usePresets() {
  // Subscribe to singleton
  subscriberCount++;

  // Also expose load function to allow explicit refresh
  const presets = globalPresets;
  const initialized = globalInitialized;

  // Watch for external changes
  if (subscriberCount === 1) {
    storeChangeHandler = () => {
      // Already synced via globalPresets
    };
  }

  const loadPresets = async () => {
    await loadGlobalPresets();
  };

  const savePresets = async (newPresets: Preset[]) => {
    await saveGlobalPresets(newPresets);
  };

  const addPreset = async (minutes: number, label: string) => {
    const newPreset: Preset = {
      id: Date.now().toString(),
      minutes,
      label,
    };
    await savePresets([newPreset, ...globalPresets.value]);
  };

  const updatePreset = async (id: string, minutes: number, label: string) => {
    const updated = globalPresets.value.map((p) =>
      p.id === id ? { ...p, minutes, label } : p
    );
    await savePresets(updated);
  };

  const deletePreset = async (id: string) => {
    const filtered = globalPresets.value.filter((p) => p.id !== id);
    await savePresets(filtered);
  };

  const resetToDefaults = async () => {
    await savePresets([...defaultPresets]);
  };

  const presetsForDisplay = computed(() => {
    return globalPresets.value.slice(0, 8).map((p) => ({
      minutes: p.minutes,
      label: p.label,
    }));
  });

  onUnmounted(() => {
    subscriberCount--;
    if (subscriberCount <= 0) {
      storeChangeHandler = null;
    }
  });

  // First time init
  if (!initialized.value) {
    loadPresets();
  }

  return {
    presets,
    presetsForDisplay,
    loadPresets,
    savePresets,
    addPreset,
    updatePreset,
    deletePreset,
    resetToDefaults,
  };
}
