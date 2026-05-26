<template>
  <div
    class="w-full h-full bg-gradient-to-br from-dark-900 via-dark-850 to-dark-800 text-white flex flex-col overflow-hidden"
  >
    <!-- Custom Title Bar -->
    <TitleBar
      @show-logs="showLogViewer = true"
      @show-presets="showPresetManager = true"
      @show-settings="showSettingsPanel = true"
    />

    <!-- Main Content - Scrollable -->
    <div class="flex-1 flex flex-col p-6 gap-5 overflow-y-auto overflow-x-hidden" :class="state.isRunning ? 'justify-center' : ''">
      <!-- Countdown Ring Section -->
      <div class="flex justify-center py-1 animate-slide-up flex-shrink-0" :class="state.isRunning ? 'p-4' : ''">
        <CountdownRing
          :formatted-time="
            state.isRunning ? formattedTime : previewFormattedTime
          "
          :progress="state.isRunning ? progress : 1"
          :is-running="state.isRunning"
          :size="state.isRunning ? 260 : 160"
          :stroke-width="10"
        />
      </div>

      <!-- Status Message -->
      <div
        class="text-center animate-fade-in flex-shrink-0"
        style="animation-delay: 0.2s"
      >
        <p
          v-if="!state.isRunning && selectedMinutes > 0"
          class="text-sm text-primary-400 font-medium"
        >
          {{ shutdownTimeText }}
        </p>
        <p v-if="!state.isRunning && selectedMinutes <= 0" class="text-sm text-gray-500">选择时间开始倒计时关机</p>
      </div>

      <!-- Preset Buttons (Hidden when running) -->
      <div v-if="!state.isRunning" class="animate-slide-up flex-shrink-0" style="animation-delay: 0.3s">
        <PresetButtons
          :selected-minutes="selectedMinutes"
          :is-running="state.isRunning"
          @select="onPresetSelect"
        />
      </div>

      <!-- Custom Time Input (Hidden when running) -->
      <div v-if="!state.isRunning" class="animate-slide-up flex-shrink-0" style="animation-delay: 0.4s">
        <CustomTimeInput
          v-model="selectedMinutes"
          :is-running="state.isRunning"
          @change="onCustomTimeChange"
        />
      </div>

      <!-- Action Buttons -->
      <div
        class="flex gap-3 mt-2 animate-slide-up flex-shrink-0"
        :class="state.isRunning ? 'mt-4' : ''"
        style="animation-delay: 0.5s"
      >
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
      <div
        class="animate-fade-in flex-shrink-0"
        :class="state.isRunning ? 'mt-2' : 'mt-auto'"
        style="animation-delay: 0.6s"
      >
        <div
          class="flex items-center gap-2 p-3 rounded-xl bg-yellow-500/10 border border-yellow-500/30"
        >
          <AlertTriangleIcon class="w-4 h-4 text-yellow-400 flex-shrink-0" />
          <p class="text-xs text-yellow-400 font-medium">
            警告：关机将强制关闭所有程序，请务必提前保存工作！
          </p>
        </div>
      </div>
    </div>

    <!-- Custom Confirm Dialog -->
    <Transition name="fade">
      <div
        v-if="showDialog"
        class="absolute inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50"
        @click.self="closeDialog"
      >
        <div
          class="bg-dark-800 border border-white/10 rounded-2xl p-6 m-4 max-w-sm w-full shadow-2xl animate-slide-up"
        >
          <div class="flex items-center gap-3 mb-4">
            <div
              class="w-10 h-10 rounded-full flex items-center justify-center"
              :class="
                dialogType === 'start' ? 'bg-primary-500/20' : 'bg-red-500/20'
              "
            >
              <PowerIcon
                v-if="dialogType === 'start'"
                class="w-5 h-5 text-primary-400"
              />
              <AlertCircleIcon v-else class="w-5 h-5 text-red-400" />
            </div>
            <h3 class="text-lg font-semibold">{{ dialogTitle }}</h3>
          </div>
          <p class="text-gray-400 text-sm mb-4">{{ dialogMessage }}</p>

          <!-- Auto hide to tray checkbox - only show for start dialog -->
          <div
            v-if="dialogType === 'start'"
            class="flex items-center gap-3 p-3 rounded-xl bg-dark-700/50 border border-white/5 mb-5"
          >
            <input
              id="autoHide"
              v-model="tempAutoHide"
              type="checkbox"
              class="w-4 h-4 rounded border-gray-500 text-primary-500 focus:ring-primary-500 bg-dark-800 cursor-pointer"
            />
            <label
              for="autoHide"
              class="text-sm text-gray-300 cursor-pointer select-none flex-1"
            >
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
              :class="
                dialogType === 'start'
                  ? 'btn-primary'
                  : 'bg-red-500 hover:bg-red-600 text-white'
              "
            >
              确认
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Log Viewer -->
    <Transition name="fade">
      <LogViewer
        v-if="showLogViewer"
        :logs="logs"
        :stats="stats"
        @close="showLogViewer = false"
        @clear="handleClearLogs"
      />
    </Transition>

    <!-- Preset Manager -->
    <Transition name="fade">
      <PresetManager
        v-if="showPresetManager"
        @close="showPresetManager = false"
      />
    </Transition>

    <!-- Settings Panel -->
    <Transition name="fade">
      <SettingsPanel
        v-if="showSettingsPanel"
        @close="showSettingsPanel = false"
      />
    </Transition>

    <!-- Global Confirm Dialog -->
    <ConfirmDialog
      :visible="confirmState.visible"
      :title="confirmState.title"
      :message="confirmState.message"
      :type="confirmState.type"
      :confirm-text="confirmState.confirmText"
      :cancel-text="confirmState.cancelText"
      :show-cancel="confirmState.showCancel"
      @confirm="onDialogConfirm"
      @cancel="onDialogCancel"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from "vue";
import {
  PlayIcon,
  SquareIcon,
  PowerIcon,
  AlertCircleIcon,
  AlertTriangleIcon,
} from "lucide-vue-next";
import TitleBar from "./components/TitleBar.vue";
import CountdownRing from "./components/CountdownRing.vue";
import PresetButtons from "./components/PresetButtons.vue";
import CustomTimeInput from "./components/CustomTimeInput.vue";
import LogViewer from "./components/LogViewer.vue";
import PresetManager from "./components/PresetManager.vue";
import SettingsPanel from "./components/SettingsPanel.vue";
import ConfirmDialog from "./components/ConfirmDialog.vue";
import { useShutdown } from "./composables/useShutdown";
import { useSettings } from "./composables/useSettings";
import { useLogger } from "./composables/useLogger";
import { useConfirm } from "./composables/useConfirm";
import { invoke } from "@tauri-apps/api/core";
import { sendNotification } from "@tauri-apps/plugin-notification";

const {
  state,
  formattedTime,
  progress,
  startCountdown,
  cancelShutdown,
  getStatus,
} = useShutdown();

const { logs, stats, addLog, clearLogs } = useLogger();
const {
  state: confirmState,
  onConfirm: onDialogConfirm,
  onCancel: onDialogCancel,
} = useConfirm();

const selectedMinutes = ref(0);
const currentTime = ref(Date.now());

// 每秒更新当前时间，用于实时计算预计关机时间
let timeUpdateInterval: number | null = null;

const startTimeUpdate = () => {
  if (timeUpdateInterval) return;
  timeUpdateInterval = window.setInterval(() => {
    currentTime.value = Date.now();
  }, 1000);
};

const stopTimeUpdate = () => {
  if (timeUpdateInterval) {
    clearInterval(timeUpdateInterval);
    timeUpdateInterval = null;
  }
};

const formatDayText = (now: Date, shutdownTime: Date) => {
  const timeStr = shutdownTime.toLocaleTimeString("zh-CN", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  });

  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const shutdownDay = new Date(shutdownTime.getFullYear(), shutdownTime.getMonth(), shutdownTime.getDate());
  const dayDiff = Math.round((shutdownDay.getTime() - today.getTime()) / (1000 * 60 * 60 * 24));

  if (dayDiff === 0) {
    return timeStr;
  } else if (dayDiff === 1) {
    return `明天 ${timeStr}`;
  } else if (dayDiff === 2) {
    return `后天 ${timeStr}`;
  } else {
    const dateStr = shutdownTime.toLocaleDateString("zh-CN", {
      month: "2-digit",
      day: "2-digit",
    });
    return `${dateStr} ${timeStr}`;
  }
};

// 计算预计关机时间文本（实时更新）
const shutdownTimeText = computed(() => {
  const seconds = state.value.isRunning 
    ? state.value.remainingSeconds 
    : selectedMinutes.value * 60;
  if (seconds <= 0) return "";
  const now = new Date(currentTime.value);
  const shutdownTime = new Date(
    currentTime.value + seconds * 1000
  );
  const dayText = formatDayText(now, shutdownTime);
  if (dayText.includes("明天") || dayText.includes("后天") || dayText.match(/^\d{2}\/\d{2}/)) {
    return dayText + " 自动关机";
  }
  return `预计 ${dayText} 自动关机`;
});

// 预览倒计时时间（未开始时的显示）
const previewFormattedTime = computed(() => {
  if (selectedMinutes.value <= 0) return "00:00:00";
  const hours = Math.floor(selectedMinutes.value / 60);
  const minutes = selectedMinutes.value % 60;
  const seconds = 0;
  return `${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`;
});

// Dialog state
const showDialog = ref(false);
const dialogType = ref<"start" | "cancel">("start");
const dialogTitle = ref("");
const dialogMessage = ref("");
const tempAutoHide = ref(false);
const showLogViewer = ref(false);
const showPresetManager = ref(false);
const showSettingsPanel = ref(false);

const onPresetSelect = (minutes: number) => {
  selectedMinutes.value = minutes;
  if (minutes > 0 && !state.value.isRunning) {
    startTimeUpdate();
  }
};

const onCustomTimeChange = (totalMinutes: number) => {
  selectedMinutes.value = totalMinutes;
  if (totalMinutes > 0 && !state.value.isRunning) {
    startTimeUpdate();
  } else if (totalMinutes <= 0) {
    stopTimeUpdate();
  }
};

const showStartConfirm = () => {
  if (selectedMinutes.value <= 0) return;

  const hours = Math.floor(selectedMinutes.value / 60);
  const mins = selectedMinutes.value % 60;
  let timeText = "";
  if (hours > 0) {
    timeText = `${hours}小时${mins > 0 ? mins + "分钟" : ""}`;
  } else {
    timeText = `${mins}分钟`;
  }

  // Load saved setting
  tempAutoHide.value = autoHideToTray.value;

  dialogType.value = "start";
  dialogTitle.value = "确认开始倒计时";
  dialogMessage.value = `系统将在 ${timeText} 后自动关机。请确保已保存所有工作。`;
  showDialog.value = true;
};

const showCancelConfirm = () => {
  dialogType.value = "cancel";
  dialogTitle.value = "确认取消关机";
  dialogMessage.value = "确定要取消当前的关机计划吗？";
  showDialog.value = true;
};

const closeDialog = () => {
  showDialog.value = false;
};

const confirmAction = async () => {
  closeDialog();

  if (dialogType.value === "start") {
    // Save the auto hide setting
    await saveSettings(tempAutoHide.value);

    // Start shutdown
    await startShutdown();

    // Auto hide to tray if enabled
    if (tempAutoHide.value) {
      setTimeout(async () => {
        try {
          await invoke("hide_to_tray");
        } catch (error) {
          console.error("Failed to hide to tray:", error);
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
      // 停止预览定时器
      stopTimeUpdate();
      // 计算预计关机时间
      const now = new Date();
      const shutdownTime = new Date(
        Date.now() + selectedMinutes.value * 60 * 1000
      );

      // 格式化倒计时时长
      let minsLeft = selectedMinutes.value;
      let countdownLabel = '';
      const minsPerDay = 24 * 60;
      const minsPerHour = 60;
      if (minsLeft >= minsPerDay) {
        const days = Math.floor(minsLeft / minsPerDay);
        minsLeft = minsLeft % minsPerDay;
        countdownLabel += `${days}天`;
      }
      if (minsLeft >= minsPerHour) {
        const hours = Math.floor(minsLeft / minsPerHour);
        minsLeft = minsLeft % minsPerHour;
        countdownLabel += `${hours}小时`;
      }
      if (minsLeft > 0) {
        countdownLabel += `${minsLeft}分钟`;
      }

      // 使用同样的格式化函数
      const daySuffix = formatDayText(now, shutdownTime);

      // Log shutdown start
      await addLog(
        "shutdown_start",
        `关机倒计时${countdownLabel}，${daySuffix}自动关机`,
        {}
      );
    } catch (error) {
      console.error("Failed to start shutdown:", error);
      alert("启动关机失败，请确保以管理员身份运行应用");
    }
  }
};

const doCancelShutdown = async () => {
  try {
    // 先保存剩余时间，因为 cancelShutdown 会重置它
    const remaining = formattedTime.value;
    await cancelShutdown();
    // Log shutdown cancel
    await addLog("shutdown_cancel", `剩余${remaining}时被关闭，手动关闭`, {});
  } catch (error) {
    console.error("Failed to cancel shutdown:", error);
  }
};

const { confirm } = useConfirm();

const handleClearLogs = async () => {
  const confirmed = await confirm({
    title: "清空日志",
    message: "确定要清空所有日志吗？此操作不可恢复。",
    type: "danger",
    confirmText: "清空",
    cancelText: "取消",
  });

  if (confirmed) {
    await clearLogs();
  }
};

const {
  autoHideToTray,
  saveSettings,
  settings,
  loadSettings,
} = useSettings();

// Track notified minutes to avoid multiple notifications
const notifiedMinutes = ref<Set<number>>(new Set());

// Watch for countdown key points (5, 3, 1 minutes)
watch(
  () => state.value.remainingSeconds,
  (newSeconds) => {
    if (!settings.value.enableNotification || !state.value.isRunning) return;

    const remainingMinutes = Math.ceil(newSeconds / 60);
    const triggerPoints = [5, 3, 1];

    if (triggerPoints.includes(remainingMinutes) && !notifiedMinutes.value.has(remainingMinutes)) {
      notifiedMinutes.value.add(remainingMinutes);
      sendNotification({
        title: "自动关机提醒",
        body: `还剩 ${remainingMinutes} 分钟即将自动关机，请立即保存文档，避免重要数据丢失！`,
      });
    }
  }
);

// Reset notified minutes when countdown starts
// Also detect countdown completion for logging
watch(
  () => state.value.isRunning,
  (isRunning, wasRunning) => {
    if (isRunning) {
      notifiedMinutes.value.clear();
    } else if (wasRunning && state.value.remainingSeconds === 0) {
      // Countdown completed normally, log completion
      addLog('shutdown_complete', '关机倒计时已结束', {});
    }
  }
);

onMounted(async () => {
  // Check for existing shutdown status
  getStatus();
  // Load settings first
  await loadSettings();
  // Auto-start if enabled
  if (settings.value.autoExecutePreset && settings.value.presetMinutes) {
    try {
      await startCountdown(settings.value.presetMinutes);
    } catch (e) {
      console.error('Auto-start failed:', e);
    }
  }
  // Log app start
  await addLog("app_start", "应用程序启动", {});
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
