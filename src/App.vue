<template>
  <div class="w-full h-full bg-gradient-to-br from-dark-900 via-dark-850 to-dark-800 text-white flex flex-col overflow-hidden">
    <!-- Custom Title Bar -->
    <TitleBar />

    <!-- Main Content - Scrollable -->
    <div class="flex-1 flex flex-col p-6 gap-5 overflow-y-auto">
      <!-- Countdown Ring Section -->
      <div class="flex justify-center py-1 animate-slide-up flex-shrink-0">
        <CountdownRing
          :formatted-time="formattedTime"
          :progress="progress"
          :is-running="state.isRunning"
          :size="160"
          :stroke-width="6"
        />
      </div>

      <!-- Status Message -->
      <div class="text-center animate-fade-in flex-shrink-0" style="animation-delay: 0.2s;">
        <p v-if="state.isRunning" class="text-sm text-primary-400 font-medium">
          系统将在 {{ formattedTime }} 后关机
        </p>
        <p v-else class="text-sm text-gray-500">
          选择时间开始倒计时关机
        </p>
      </div>

      <!-- Preset Buttons -->
      <div class="animate-slide-up flex-shrink-0" style="animation-delay: 0.3s;">
        <PresetButtons
          :selected-minutes="selectedMinutes"
          :is-running="state.isRunning"
          @select="onPresetSelect"
        />
      </div>

      <!-- Custom Time Input -->
      <div class="animate-slide-up flex-shrink-0" style="animation-delay: 0.4s;">
        <CustomTimeInput
          :is-running="state.isRunning"
          @change="onCustomTimeChange"
        />
      </div>

      <!-- Action Buttons -->
      <div class="flex gap-3 mt-2 animate-slide-up flex-shrink-0" style="animation-delay: 0.5s;">
        <button
          v-if="!state.isRunning"
          @click="showStartConfirm"
          :disabled="selectedMinutes <= 0"
          class="btn-primary flex-1 flex items-center justify-center gap-2 py-2.5"
          :class="selectedMinutes <= 0 ? 'opacity-50 cursor-not-allowed' : ''"
        >
          <PlayIcon class="w-4 h-4" />
          开始倒计时
        </button>
        <button
          v-else
          @click="showCancelConfirm"
          class="btn-secondary flex-1 flex items-center justify-center gap-2 py-2.5 border-red-500/30 text-red-400 hover:bg-red-500/10 hover:text-red-300"
        >
          <SquareIcon class="w-4 h-4" />
          取消关机
        </button>
      </div>

      <!-- Warning Note -->
      <div class="animate-fade-in mt-auto flex-shrink-0" style="animation-delay: 0.6s;">
        <div class="flex items-center gap-2 p-3 rounded-xl bg-yellow-500/10 border border-yellow-500/30">
          <AlertTriangleIcon class="w-4 h-4 text-yellow-400 flex-shrink-0" />
          <p class="text-xs text-yellow-400 font-medium">
            警告：关机将强制关闭所有程序，请务必提前保存工作！
          </p>
        </div>
      </div>
    </div>

    <!-- Custom Confirm Dialog -->
    <Transition name="fade">
      <div v-if="showDialog" class="absolute inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50" @click.self="closeDialog">
        <div class="bg-dark-800 border border-white/10 rounded-2xl p-6 m-4 max-w-sm w-full shadow-2xl animate-slide-up">
          <div class="flex items-center gap-3 mb-4">
            <div class="w-10 h-10 rounded-full flex items-center justify-center" :class="dialogType === 'start' ? 'bg-primary-500/20' : 'bg-red-500/20'">
              <PowerIcon v-if="dialogType === 'start'" class="w-5 h-5 text-primary-400" />
              <AlertCircleIcon v-else class="w-5 h-5 text-red-400" />
            </div>
            <h3 class="text-lg font-semibold">{{ dialogTitle }}</h3>
          </div>
          <p class="text-gray-400 text-sm mb-4">{{ dialogMessage }}</p>
          
          <!-- Auto hide to tray checkbox - only show for start dialog -->
          <div v-if="dialogType === 'start'" class="flex items-center gap-3 p-3 rounded-xl bg-dark-700/50 border border-white/5 mb-5">
            <input
              id="autoHide"
              v-model="tempAutoHide"
              type="checkbox"
              class="w-4 h-4 rounded border-gray-500 text-primary-500 focus:ring-primary-500 bg-dark-800 cursor-pointer"
            />
            <label for="autoHide" class="text-sm text-gray-300 cursor-pointer select-none flex-1">
              开始倒计时后自动隐藏到托盘
            </label>
          </div>

          <div class="flex gap-3">
            <button @click="closeDialog" class="btn-secondary flex-1 py-2.5">
              取消
            </button>
            <button 
              @click="confirmAction" 
              class="flex-1 py-2.5 rounded-xl font-medium transition-all duration-200"
              :class="dialogType === 'start' ? 'btn-primary' : 'bg-red-500 hover:bg-red-600 text-white'"
            >
              确认
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { PlayIcon, SquareIcon, PowerIcon, AlertCircleIcon, AlertTriangleIcon } from 'lucide-vue-next';
import TitleBar from './components/TitleBar.vue';
import CountdownRing from './components/CountdownRing.vue';
import PresetButtons from './components/PresetButtons.vue';
import CustomTimeInput from './components/CustomTimeInput.vue';
import { useShutdown } from './composables/useShutdown';
import { useSettings } from './composables/useSettings';
import { invoke } from '@tauri-apps/api/core';

const {
  state,
  formattedTime,
  progress,
  startCountdown,
  cancelShutdown,
  getStatus,
} = useShutdown();

const { autoHideToTray, saveSettings } = useSettings();

const selectedMinutes = ref(0);

// Dialog state
const showDialog = ref(false);
const dialogType = ref<'start' | 'cancel'>('start');
const dialogTitle = ref('');
const dialogMessage = ref('');
const tempAutoHide = ref(false);

const onPresetSelect = (minutes: number) => {
  selectedMinutes.value = minutes;
};

const onCustomTimeChange = (totalMinutes: number) => {
  selectedMinutes.value = totalMinutes;
};

const showStartConfirm = () => {
  if (selectedMinutes.value <= 0) return;
  
  const hours = Math.floor(selectedMinutes.value / 60);
  const mins = selectedMinutes.value % 60;
  let timeText = '';
  if (hours > 0) {
    timeText = `${hours}小时${mins > 0 ? mins + '分钟' : ''}`;
  } else {
    timeText = `${mins}分钟`;
  }
  
  // Load saved setting
  tempAutoHide.value = autoHideToTray.value;
  
  dialogType.value = 'start';
  dialogTitle.value = '确认开始倒计时';
  dialogMessage.value = `系统将在 ${timeText} 后自动关机。请确保已保存所有工作。`;
  showDialog.value = true;
};

const showCancelConfirm = () => {
  dialogType.value = 'cancel';
  dialogTitle.value = '确认取消关机';
  dialogMessage.value = '确定要取消当前的关机计划吗？';
  showDialog.value = true;
};

const closeDialog = () => {
  showDialog.value = false;
};

const confirmAction = async () => {
  closeDialog();
  
  if (dialogType.value === 'start') {
    // Save the auto hide setting
    await saveSettings(tempAutoHide.value);
    
    // Start shutdown
    await startShutdown();
    
    // Auto hide to tray if enabled
    if (tempAutoHide.value) {
      setTimeout(async () => {
        try {
          await invoke('hide_to_tray');
        } catch (error) {
          console.error('Failed to hide to tray:', error);
        }
      }, 1000);
    }
  } else {
    await doCancelShutdown();
  }
};

const startShutdown = async () => {
  if (selectedMinutes.value > 0) {
    try {
      await startCountdown(selectedMinutes.value);
    } catch (error) {
      console.error('Failed to start shutdown:', error);
      alert('启动关机失败，请确保以管理员身份运行应用');
    }
  }
};

const doCancelShutdown = async () => {
  try {
    await cancelShutdown();
  } catch (error) {
    console.error('Failed to cancel shutdown:', error);
  }
};

onMounted(() => {
  // Check for existing shutdown status
  getStatus();
});
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
