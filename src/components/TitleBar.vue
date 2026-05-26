<template>
  <div class="draggable h-12 flex items-center justify-between px-4 bg-dark-900/80 backdrop-blur-md border-b border-white/5 relative z-40">
    <!-- Logo and Title -->
    <div class="flex items-center gap-2 no-drag">
      <div class="w-6 h-6 rounded-lg bg-gradient-to-br from-primary-500 to-primary-600 flex items-center justify-center">
        <PowerIcon class="w-3.5 h-3.5 text-white" />
      </div>
      <span class="text-sm font-medium text-white/90">Auto Shutdown</span>
    </div>

    <!-- Window Controls -->
    <div class="flex items-center gap-1 no-drag">
      <!-- Settings Dropdown Button -->
      <div class="settings-dropdown-wrapper relative">
        <button
          @click="showDropdown = !showDropdown"
          class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-white hover:bg-white/10 transition-all duration-200"
          title="菜单"
        >
          <SettingsIcon class="w-4 h-4" />
        </button>
        
        <!-- Dropdown Menu -->
        <div
          v-if="showDropdown"
          class="absolute right-0 top-full mt-1 w-36 bg-dark-800 border border-white/10 rounded-xl shadow-2xl py-1 z-[100] animate-slide-up"
        >
          <button
            @click="handleMenuClick('logs')"
            class="w-full px-4 py-2.5 text-left text-sm text-gray-300 hover:bg-white/10 hover:text-white flex items-center gap-3 transition-colors"
          >
            <FileTextIcon class="w-4 h-4" />
            日志
          </button>
          <button
            @click="handleMenuClick('presets')"
            class="w-full px-4 py-2.5 text-left text-sm text-gray-300 hover:bg-white/10 hover:text-white flex items-center gap-3 transition-colors"
          >
            <ClockIcon class="w-4 h-4" />
            快速预设
          </button>
          <button
            @click="handleMenuClick('settings')"
            class="w-full px-4 py-2.5 text-left text-sm text-gray-300 hover:bg-white/10 hover:text-white flex items-center gap-3 transition-colors"
          >
            <SettingsIcon class="w-4 h-4" />
            设置
          </button>
        </div>
      </div>
      
      <button
        @click="minimizeWindow"
        class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-white hover:bg-white/10 transition-all duration-200"
        title="最小化"
      >
        <MinusIcon class="w-4 h-4" />
      </button>
      <button
        @click="hideToTray"
        class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-white hover:bg-red-500/80 transition-all duration-200"
        title="隐藏到托盘"
      >
        <XIcon class="w-4 h-4" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { PowerIcon, MinusIcon, XIcon, FileTextIcon, SettingsIcon, ClockIcon } from 'lucide-vue-next';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { invoke } from '@tauri-apps/api/core';

const showDropdown = ref(false);

const emit = defineEmits<{
  'show-logs': [];
  'show-presets': [];
  'show-settings': [];
}>();

const handleMenuClick = (action: string) => {
  showDropdown.value = false;
  if (action === 'logs') {
    emit('show-logs');
  } else if (action === 'presets') {
    emit('show-presets');
  } else if (action === 'settings') {
    emit('show-settings');
  }
};

const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement;
  if (!target.closest('.settings-dropdown-wrapper')) {
    showDropdown.value = false;
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});

const minimizeWindow = async () => {
  try {
    const window = getCurrentWebviewWindow();
    await window.minimize();
  } catch (error) {
    console.error('Failed to minimize window:', error);
  }
};

const hideToTray = async () => {
  try {
    const window = getCurrentWebviewWindow();
    await invoke('hide_to_tray', { window });
  } catch (error) {
    console.error('Failed to hide to tray:', error);
    // Fallback: just hide the window
    try {
      const window = getCurrentWebviewWindow();
      await window.hide();
    } catch (e) {
      console.error('Fallback hide failed:', e);
    }
  }
};
</script>
