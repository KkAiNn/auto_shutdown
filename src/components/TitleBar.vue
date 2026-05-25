<template>
  <div class="draggable h-12 flex items-center justify-between px-4 bg-dark-900/80 backdrop-blur-md border-b border-white/5">
    <!-- Logo and Title -->
    <div class="flex items-center gap-2 no-drag">
      <div class="w-6 h-6 rounded-lg bg-gradient-to-br from-primary-500 to-primary-600 flex items-center justify-center">
        <PowerIcon class="w-3.5 h-3.5 text-white" />
      </div>
      <span class="text-sm font-medium text-white/90">Auto Shutdown</span>
    </div>

    <!-- Window Controls -->
    <div class="flex items-center gap-1 no-drag">
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
import { PowerIcon, MinusIcon, XIcon } from 'lucide-vue-next';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { invoke } from '@tauri-apps/api/core';

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
