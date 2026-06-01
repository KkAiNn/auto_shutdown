<template>
  <div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50" @click.self="$emit('close')">
    <div class="bg-dark-800 border border-white/10 rounded-2xl m-4 max-w-sm w-full shadow-2xl animate-slide-up">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-white/10">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-blue-500/20 flex items-center justify-center">
            <SettingsIcon class="w-4 h-4 text-blue-400" />
          </div>
          <div>
            <h3 class="text-lg font-semibold">设置</h3>
            <p class="text-xs text-gray-500">应用程序设置</p>
          </div>
        </div>
        <button @click="$emit('close')" class="p-2 rounded-lg hover:bg-white/10 transition-colors">
          <XIcon class="w-5 h-5 text-gray-400" />
        </button>
      </div>

      <!-- Settings Content -->
      <div class="p-4 space-y-5">
        <!-- Auto Start Toggle -->
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium">开机自启动</p>
            <p class="text-xs text-gray-500 mt-0.5">登录 Windows 后自动启动应用</p>
          </div>
          <button
            @click="toggleAutoStart"
            class="relative w-12 h-6 rounded-full transition-colors duration-200"
            :class="autoStartEnabled ? 'bg-primary-500' : 'bg-white/10'"
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
            <p class="text-sm font-medium">倒计时提醒通知</p>
            <p class="text-xs text-gray-500 mt-0.5">还剩5、3、1分钟时发送系统通知</p>
          </div>
          <button
            @click="toggleNotification"
            class="relative w-12 h-6 rounded-full transition-colors duration-200"
            :class="settings.enableNotification ? 'bg-primary-500' : 'bg-white/10'"
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
            <p class="text-sm font-medium">自动执行预设</p>
            <p class="text-xs text-gray-500 mt-0.5">应用启动后自动开始倒计时</p>
          </div>
          <button
            @click="toggleAutoExecute"
            class="relative w-12 h-6 rounded-full transition-colors duration-200"
            :class="settings.autoExecutePreset ? 'bg-primary-500' : 'bg-white/10'"
          >
            <div
              class="absolute top-1 w-4 h-4 rounded-full bg-white shadow transition-transform duration-200"
              :class="settings.autoExecutePreset ? 'translate-x-7' : 'translate-x-1'"
            />
          </button>
        </div>

        <!-- Preset Selector (shown when enabled) -->
        <div v-if="settings.autoExecutePreset" class="pl-4 border-l-2 border-primary-500/30">
          <label class="block text-xs text-gray-400 mb-2">选择预设值</label>
          <div class="relative">
            <select
              :value="settings.presetMinutes ?? presetsForDisplay[0]?.minutes ?? 60"
              @change="onPresetChange"
              class="w-full appearance-none px-4 py-2.5 rounded-xl bg-dark-700/50 border border-white/10 text-white text-sm focus:outline-none focus:border-primary-500/50 transition-colors pr-10"
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

      <!-- Footer -->
      <div class="flex items-center justify-end p-4 border-t border-white/10">
        <button
          @click="$emit('close')"
          class="btn-primary px-6 py-2"
        >
          关闭
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { XIcon, SettingsIcon, ChevronDownIcon } from 'lucide-vue-next';
import { useSettings } from '../composables/useSettings';
import { usePresets } from '../composables/usePresets';

defineEmits<{
  close: [];
}>();

const { autoStartEnabled, settings, setAutoStart, setAutoExecutePreset, setPresetMinutes, setEnableNotification } = useSettings();
const { presets } = usePresets();

const presetsForDisplay = computed(() => {
  return presets.value.slice(0, 8);
});

const toggleAutoStart = () => {
  setAutoStart(!autoStartEnabled.value);
};

const toggleNotification = () => {
  setEnableNotification(!settings.value.enableNotification);
};

const toggleAutoExecute = () => {
  const newVal = !settings.value.autoExecutePreset;
  setAutoExecutePreset(newVal);
  // When enabling, auto-select first preset if none selected
  if (newVal && !settings.value.presetMinutes && presetsForDisplay.value.length > 0) {
    setPresetMinutes(presetsForDisplay.value[0].minutes);
  }
};

const onPresetChange = (e: Event) => {
  const val = parseInt((e.target as HTMLSelectElement).value) || 60;
  setPresetMinutes(val);
};
</script>
