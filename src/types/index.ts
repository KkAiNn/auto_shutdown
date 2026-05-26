// 日志条目类型
export interface LogEntry {
  id: string;
  timestamp: string;
  type: LogType;
  message: string;
  details?: Record<string, unknown>;
}

// 日志类型
export type LogType =
  | 'app_start'      // 应用启动
  | 'app_exit'       // 应用正常关闭
  | 'app_crash'      // 应用异常关闭
  | 'shutdown_start' // 开始关机倒计时
  | 'shutdown_cancel'// 取消关机
  | 'shutdown_complete'; // 关机命令执行完成

// 日志统计信息
export interface LogStats {
  totalStarts: number;
  totalExits: number;
  totalCrashes: number;
  totalShutdowns: number;
  totalCanceledShutdowns: number;
  totalCompletedShutdowns: number;
}
