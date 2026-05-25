import { ref, computed, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface ShutdownState {
  isRunning: boolean;
  remainingSeconds: number;
  totalSeconds: number;
}

export function useShutdown() {
  const state = ref<ShutdownState>({
    isRunning: false,
    remainingSeconds: 0,
    totalSeconds: 0,
  });

  let timerInterval: number | null = null;

  const formattedTime = computed(() => {
    const hours = Math.floor(state.value.remainingSeconds / 3600);
    const minutes = Math.floor((state.value.remainingSeconds % 3600) / 60);
    const seconds = state.value.remainingSeconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  });

  const progress = computed(() => {
    if (state.value.totalSeconds === 0) return 0;
    return ((state.value.totalSeconds - state.value.remainingSeconds) / state.value.totalSeconds) * 100;
  });

  const startCountdown = async (minutes: number) => {
    if (state.value.isRunning) {
      await cancelShutdown();
    }

    const seconds = minutes * 60;
    state.value.totalSeconds = seconds;
    state.value.remainingSeconds = seconds;
    state.value.isRunning = true;

    try {
      // Use schedule_shutdown which handles countdown internally
      await invoke('schedule_shutdown', { seconds });
      // Start frontend timer for display only
      startDisplayTimer();
    } catch (error) {
      console.error('Failed to start shutdown:', error);
      state.value.isRunning = false;
      throw error;
    }
  };

  const startDisplayTimer = () => {
    if (timerInterval) {
      clearInterval(timerInterval);
    }

    timerInterval = window.setInterval(async () => {
      if (state.value.remainingSeconds > 0) {
        state.value.remainingSeconds--;
      } else {
        stopTimer();
        state.value.isRunning = false;
      }
    }, 1000);
  };

  const stopTimer = () => {
    if (timerInterval) {
      clearInterval(timerInterval);
      timerInterval = null;
    }
  };

  const cancelShutdown = async () => {
    stopTimer();
    state.value.isRunning = false;
    state.value.remainingSeconds = 0;
    state.value.totalSeconds = 0;

    try {
      await invoke('cancel_shutdown');
    } catch (error) {
      console.error('Failed to cancel shutdown:', error);
      throw error;
    }
  };

  const getStatus = async () => {
    try {
      const [isScheduled, remaining] = await invoke<[boolean, number]>('get_shutdown_status');
      if (isScheduled && remaining > 0) {
        state.value.isRunning = true;
        state.value.remainingSeconds = remaining;
        if (!timerInterval) {
          startDisplayTimer();
        }
      }
    } catch (error) {
      console.error('Failed to get status:', error);
    }
  };

  // Cleanup on unmount
  onUnmounted(() => {
    stopTimer();
  });

  return {
    state,
    formattedTime,
    progress,
    startCountdown,
    cancelShutdown,
    getStatus,
  };
}
