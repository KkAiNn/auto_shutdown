<template>
  <div class="h-screen w-screen bg-dark-800/95 backdrop-blur-xl flex flex-col overflow-hidden text-white">
    <!-- Custom Title Bar -->
    <div 
      class="h-10 flex items-center justify-between px-4 border-b border-white/10 bg-dark-900/50 select-none" 
      data-tauri-drag-region
    >
      <div class="flex items-center gap-2 pointer-events-none">
        <SettingsIcon class="w-4 h-4 text-cyan-400" />
        <span class="text-sm font-medium text-white">设置</span>
      </div>
      <div class="flex items-center gap-1">
        <button 
          @click.stop="minimizeWindow" 
          class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-white hover:bg-white/10 transition-colors pointer-events-auto"
        >
          <MinusIcon class="w-4 h-4" />
        </button>
        <button 
          @click.stop="closeWindow" 
          class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-red-400 hover:bg-red-500/10 transition-colors pointer-events-auto"
        >
          <XIcon class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="!isReady" class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center gap-3">
        <div class="w-8 h-8 border-2 border-cyan-500/30 border-t-cyan-500 rounded-full animate-spin"></div>
        <span class="text-sm text-gray-400">加载中...</span>
      </div>
    </div>

    <!-- Settings Content -->
    <div v-else class="flex-1 overflow-y-auto p-6 space-y-6">
      <!-- Auto Start Toggle -->
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-medium text-white">开机自启动</p>
          <p class="text-xs text-gray-400 mt-0.5">登录 Windows 后自动启动应用</p>
        </div>
        <button
          @click="toggleAutoStart"
          class="relative w-12 h-6 rounded-full transition-colors duration-200"
          :class="autoStartEnabled ? 'bg-cyan-500' : 'bg-white/10'"
        >
          <div
            class="absolute top-1 w-4 h-4 rounded-full bg-white shadow transition-transform duration-200"
            :class="autoStartEnabled ? 'translate-x-7' : 'translate-x-1'"
          />
        </button>
      </div>

      <div class="border-t border-white/5"></div>

      <!-- Notification Toggle -->
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-medium text-white">倒计时提醒通知</p>
          <p class="text-xs text-gray-400 mt-0.5">还剩5、3、1分钟时发送系统通知</p>
        </div>
        <button
          @click="toggleNotification"
          class="relative w-12 h-6 rounded-full transition-colors duration-200"
          :class="settings.enableNotification ? 'bg-cyan-500' : 'bg-white/10'"
        >
          <div
            class="absolute top-1 w-4 h-4 rounded-full bg-white shadow transition-transform duration-200"
            :class="settings.enableNotification ? 'translate-x-7' : 'translate-x-1'"
          />
        </button>
      </div>

      <div class="border-t border-white/5"></div>

      <!-- Auto Execute Preset Toggle -->
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-medium text-white">自动执行预设</p>
          <p class="text-xs text-gray-400 mt-0.5">应用启动后自动开始倒计时</p>
        </div>
        <button
          @click="toggleAutoExecute"
          class="relative w-12 h-6 rounded-full transition-colors duration-200"
          :class="settings.autoExecutePreset ? 'bg-cyan-500' : 'bg-white/10'"
        >
          <div
            class="absolute top-1 w-4 h-4 rounded-full bg-white shadow transition-transform duration-200"
            :class="settings.autoExecutePreset ? 'translate-x-7' : 'translate-x-1'"
          />
        </button>
      </div>

      <!-- Preset Selector (shown when enabled) -->
      <div v-if="settings.autoExecutePreset" class="pl-4 border-l-2 border-cyan-500/30">
        <label class="block text-xs text-gray-400 mb-2">选择预设值</label>
        <div class="relative">
          <select
            :value="settings.presetMinutes ?? presetsForDisplay[0]?.minutes ?? 60"
            @change="onPresetChange"
            class="w-full appearance-none px-4 py-2.5 rounded-xl bg-dark-700/50 border border-white/10 text-white text-sm focus:outline-none focus:border-cyan-500/50 transition-colors pr-10"
          >
            <option
              v-for="preset in presetsForDisplay"
              :key="preset.id"
              :value="preset.minutes"
            >
              {{ preset.label }} ({{ preset.minutes }} 分钟)
            </option>
          </select>
          <div class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none">
            <ChevronDownIcon class="w-4 h-4 text-gray-500" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { SettingsIcon, XIcon, MinusIcon, ChevronDownIcon } from 'lucide-vue-next';
import { useSettings } from '../composables/useSettings';
import { usePresets } from '../composables/usePresets';
import { getCurrentWindow } from '@tauri-apps/api/window';

const { autoStartEnabled, settings, initialized, loadSettings, setAutoStart, setAutoExecutePreset, setPresetMinutes, setEnableNotification } = useSettings();
const { presets, loadPresets } = usePresets();

const isReady = ref(false);

onMounted(async () => {
  if (!initialized.value) {
    await loadSettings();
  }
  await loadPresets();
  isReady.value = true;
});

const presetsForDisplay = computed(() => {
  return presets.value.slice(0, 8);
});

const minimizeWindow = async () => {
  const window = getCurrentWindow();
  await window.minimize();
};

const closeWindow = async () => {
  const window = getCurrentWindow();
  await window.close();
};

const toggleAutoStart = () => {
  setAutoStart(!autoStartEnabled.value);
};

const toggleNotification = () => {
  setEnableNotification(!settings.value.enableNotification);
};

const toggleAutoExecute = () => {
  const newVal = !settings.value.autoExecutePreset;
  setAutoExecutePreset(newVal);
  if (newVal && !settings.value.presetMinutes && presetsForDisplay.value.length > 0) {
    setPresetMinutes(presetsForDisplay.value[0].minutes);
  }
};

const onPresetChange = (e: Event) => {
  const val = parseInt((e.target as HTMLSelectElement).value) || 60;
  setPresetMinutes(val);
};
</script>
