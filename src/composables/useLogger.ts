import { ref, onMounted } from 'vue';
import { Store } from '@tauri-apps/plugin-store';
import type { LogEntry, LogType, LogStats } from '../types';

const LOGS_KEY = 'app_logs';
const STATS_KEY = 'log_stats';
const MAX_LOGS = 1000; // 最多保留1000条日志

export function useLogger() {
  const logs = ref<LogEntry[]>([]);
  const stats = ref<LogStats>({
    totalStarts: 0,
    totalExits: 0,
    totalCrashes: 0,
    totalShutdowns: 0,
    totalCanceledShutdowns: 0,
    totalCompletedShutdowns: 0,
  });
  let store: Store | null = null;

  const initStore = async () => {
    if (!store) {
      store = await Store.load('logs.json');
    }
  };

  // 生成唯一ID
  const generateId = () => {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  };

  // 添加日志
  const addLog = async (type: LogType, message: string, details?: Record<string, unknown>) => {
    await initStore();
    
    const entry: LogEntry = {
      id: generateId(),
      timestamp: new Date().toISOString(),
      type,
      message,
      details,
    };

    // 添加到内存
    logs.value.unshift(entry);
    
    // 限制日志数量
    if (logs.value.length > MAX_LOGS) {
      logs.value = logs.value.slice(0, MAX_LOGS);
    }

    // 更新统计
    updateStats(type);

    // 持久化存储
    if (store) {
      await store.set(LOGS_KEY, logs.value);
      await store.set(STATS_KEY, stats.value);
      await store.save();
    }

    // 控制台输出（调试用）
    console.log(`[${type}] ${message}`, details || '');
  };

  // 更新统计
  const updateStats = (type: LogType) => {
    switch (type) {
      case 'app_start':
        stats.value.totalStarts++;
        break;
      case 'app_exit':
        stats.value.totalExits++;
        break;
      case 'app_crash':
        stats.value.totalCrashes++;
        break;
      case 'shutdown_start':
        stats.value.totalShutdowns++;
        break;
      case 'shutdown_cancel':
        stats.value.totalCanceledShutdowns++;
        break;
      case 'shutdown_complete':
        stats.value.totalCompletedShutdowns++;
        break;
    }
  };

  // 加载日志
  const loadLogs = async () => {
    await initStore();
    if (store) {
      const savedLogs = await store.get<LogEntry[]>(LOGS_KEY);
      if (savedLogs) {
        logs.value = savedLogs;
      }
      const savedStats = await store.get<LogStats>(STATS_KEY);
      if (savedStats) {
        stats.value = savedStats;
      }
    }
  };

  // 清空日志
  const clearLogs = async () => {
    await initStore();
    logs.value = [];
    stats.value = {
      totalStarts: 0,
      totalExits: 0,
      totalCrashes: 0,
      totalShutdowns: 0,
      totalCanceledShutdowns: 0,
      totalCompletedShutdowns: 0,
    };
    if (store) {
      await store.set(LOGS_KEY, []);
      await store.set(STATS_KEY, stats.value);
      await store.save();
    }
  };

  // 获取格式化的时间
  const formatTime = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  // 获取日志类型显示文本
  const getLogTypeText = (type: LogType) => {
    const typeMap: Record<LogType, string> = {
      app_start: '应用启动',
      app_exit: '正常关闭',
      app_crash: '异常关闭',
      shutdown_start: '开始关机',
      shutdown_cancel: '取消关机',
      shutdown_complete: '关机完成',
    };
    return typeMap[type] || type;
  };

  // 获取日志类型颜色
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

  onMounted(() => {
    loadLogs();
  });

  return {
    logs,
    stats,
    addLog,
    loadLogs,
    clearLogs,
    formatTime,
    getLogTypeText,
    getLogTypeColor,
  };
}
