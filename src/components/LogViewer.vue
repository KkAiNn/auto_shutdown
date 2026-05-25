<template>
  <div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50" @click.self="$emit('close')">
    <div class="bg-dark-800 border border-white/10 rounded-2xl m-4 max-w-lg w-full shadow-2xl animate-slide-up flex flex-col max-h-[80vh]">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-white/10">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-primary-500/20 flex items-center justify-center">
            <FileTextIcon class="w-4 h-4 text-primary-400" />
          </div>
          <div>
            <h3 class="text-lg font-semibold">运行日志</h3>
            <p class="text-xs text-gray-500">共 {{ logs.length }} 条记录</p>
          </div>
        </div>
        <button @click="$emit('close')" class="p-2 rounded-lg hover:bg-white/10 transition-colors">
          <XIcon class="w-5 h-5 text-gray-400" />
        </button>
      </div>

      <!-- Stats -->
      <div class="grid grid-cols-3 gap-3 p-4 border-b border-white/10">
        <div class="text-center p-3 rounded-xl bg-yellow-500/10 border border-yellow-500/20">
          <p class="text-2xl font-bold text-yellow-400">{{ stats.totalShutdowns }}</p>
          <p class="text-xs text-yellow-400/70">开始倒计时</p>
        </div>
        <div class="text-center p-3 rounded-xl bg-orange-500/10 border border-orange-500/20">
          <p class="text-2xl font-bold text-orange-400">{{ stats.totalShutdowns - stats.totalCompletedShutdowns }}</p>
          <p class="text-xs text-orange-400/70">取消关机</p>
        </div>
        <div class="text-center p-3 rounded-xl bg-purple-500/10 border border-purple-500/20">
          <p class="text-2xl font-bold text-purple-400">{{ stats.totalCompletedShutdowns }}</p>
          <p class="text-xs text-purple-400/70">完成关机</p>
        </div>
      </div>

      <!-- Log List -->
      <div class="flex-1 overflow-y-auto p-4 space-y-2">
        <div v-if="logs.length === 0" class="text-center py-8 text-gray-500">
          <FileTextIcon class="w-12 h-12 mx-auto mb-3 opacity-30" />
          <p>暂无日志记录</p>
        </div>
        <div
          v-for="log in logs.slice(0, 50)"
          :key="log.id"
          class="p-3 rounded-xl bg-dark-700/50 border border-white/5 hover:bg-dark-700 transition-colors"
        >
          <div class="flex items-start gap-3">
            <div class="w-2 h-2 rounded-full mt-2 flex-shrink-0" :class="getLogTypeDotColor(log.type)" />
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 mb-1">
                <span class="text-xs font-medium" :class="getLogTypeColor(log.type)">
                  {{ getLogTypeText(log.type) }}
                </span>
                <span class="text-xs text-gray-500">{{ formatTime(log.timestamp) }}</span>
              </div>
              <p class="text-sm text-gray-300 truncate">{{ log.message }}</p>
              <div v-if="log.details && Object.keys(log.details).length > 0" class="text-xs text-gray-500 mt-1 space-y-0.5">
                <div v-for="(value, key) in log.details" :key="key" class="flex items-center gap-1">
                  <span class="text-gray-600">{{ formatDetailKey(key) }}:</span>
                  <span class="text-gray-400">{{ formatDetailValue(value) }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div v-if="logs.length > 50" class="text-center py-2 text-xs text-gray-500">
          仅显示最近 50 条记录，共 {{ logs.length }} 条
        </div>
      </div>

      <!-- Footer -->
      <div class="flex items-center justify-between p-4 border-t border-white/10">
        <button
          @click="clearAllLogs"
          class="px-4 py-2 rounded-xl text-sm font-medium text-red-400 hover:bg-red-500/10 transition-colors"
        >
          清空日志
        </button>
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
import { FileTextIcon, XIcon } from 'lucide-vue-next';
import type { LogEntry, LogType, LogStats } from '../types';

interface Props {
  logs: LogEntry[];
  stats: LogStats;
}

defineProps<Props>();
const emit = defineEmits<{
  close: [];
  clear: [];
}>();

const clearAllLogs = () => {
  emit('clear');
};

const formatTime = (timestamp: string) => {
  const date = new Date(timestamp);
  return date.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
};

const formatDetailKey = (key: string) => {
  const keyMap: Record<string, string> = {
    minutes: '分钟',
    autoHide: '自动隐藏',
    remainingTime: '剩余时间',
    timestamp: '时间戳',
  };
  return keyMap[key] || key;
};

const formatDetailValue = (value: unknown) => {
  if (typeof value === 'boolean') {
    return value ? '是' : '否';
  }
  if (typeof value === 'string' && value.includes('T')) {
    // ISO timestamp
    const date = new Date(value);
    if (!isNaN(date.getTime())) {
      return date.toLocaleString('zh-CN', {
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
      });
    }
  }
  return String(value);
};

const getLogTypeText = (type: LogType) => {
  const typeMap: Record<LogType, string> = {
    app_start: '启动',
    app_exit: '退出',
    app_crash: '异常',
    shutdown_start: '开始关机',
    shutdown_cancel: '取消关机',
    shutdown_complete: '关机完成',
  };
  return typeMap[type] || type;
};

const getLogTypeColor = (type: LogType) => {
  const colorMap: Record<LogType, string> = {
    app_start: 'text-green-400',
    app_exit: 'text-blue-400',
    app_crash: 'text-red-400',
    shutdown_start: 'text-yellow-400',
    shutdown_cancel: 'text-orange-400',
    shutdown_complete: 'text-purple-400',
  };
  return colorMap[type] || 'text-gray-400';
};

const getLogTypeDotColor = (type: LogType) => {
  const colorMap: Record<LogType, string> = {
    app_start: 'bg-green-400',
    app_exit: 'bg-blue-400',
    app_crash: 'bg-red-400',
    shutdown_start: 'bg-yellow-400',
    shutdown_cancel: 'bg-orange-400',
    shutdown_complete: 'bg-purple-400',
  };
  return colorMap[type] || 'bg-gray-400';
};
</script>
