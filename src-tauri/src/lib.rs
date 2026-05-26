use std::sync::Mutex;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use tauri::AppHandle;
use tauri::Manager;

// Global state to track shutdown status
pub struct ShutdownState {
    pub is_scheduled: bool,
    pub remaining_seconds: u32,
}

impl Default for ShutdownState {
    fn default() -> Self {
        Self {
            is_scheduled: false,
            remaining_seconds: 0,
        }
    }
}

pub static SHUTDOWN_STATE: Mutex<ShutdownState> = Mutex::new(ShutdownState {
    is_scheduled: false,
    remaining_seconds: 0,
});

// Log entry structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub log_type: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

// Get log file path
fn get_log_path(app_handle: &AppHandle) -> PathBuf {
    let app_dir = app_handle.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    let log_dir = app_dir.join("logs");
    let _ = create_dir_all(&log_dir);
    log_dir.join("app.log")
}

// Write log to file
fn write_log_to_file(app_handle: &AppHandle, entry: &LogEntry) -> Result<(), String> {
    let log_path = get_log_path(app_handle);
    let log_line = format!(
        "[{}] [{}] {} - {}\n",
        entry.timestamp,
        entry.log_type,
        entry.message,
        entry.details.as_ref().map(|d| d.to_string()).unwrap_or_default()
    );
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| e.to_string())?;
    
    file.write_all(log_line.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

/// Add a log entry
#[tauri::command]
async fn add_log(
    app_handle: AppHandle,
    log_type: String,
    message: String,
    details: Option<serde_json::Value>,
) -> Result<(), String> {
    let entry = LogEntry {
        id: format!("{}-{}", chrono::Local::now().timestamp_millis(), rand::random::<u32>()),
        timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        log_type,
        message,
        details,
    };
    
    write_log_to_file(&app_handle, &entry)
}

/// Get all logs
#[tauri::command]
async fn get_logs(app_handle: AppHandle) -> Result<Vec<LogEntry>, String> {
    let log_path = get_log_path(&app_handle);
    
    if !log_path.exists() {
        return Ok(vec![]);
    }
    
    let content = std::fs::read_to_string(&log_path).map_err(|e| e.to_string())?;
    let mut logs = vec![];
    
    for line in content.lines() {
        if line.starts_with('[') {
            // Parse log line: [timestamp] [type] message - details
            if let Some(end_bracket) = line.find("] [") {
                let timestamp = &line[1..end_bracket];
                let rest = &line[end_bracket + 3..];
                if let Some(type_end) = rest.find("] ") {
                    let log_type = &rest[..type_end];
                    let message_and_details = &rest[type_end + 2..];
                    
                    let (message, details) = if let Some(dash_pos) = message_and_details.rfind(" - ") {
                        let msg = &message_and_details[..dash_pos];
                        let det_str = &message_and_details[dash_pos + 3..];
                        let details = if det_str.is_empty() || det_str == "{}" {
                            None
                        } else {
                            serde_json::from_str(det_str).ok()
                        };
                        (msg.to_string(), details)
                    } else {
                        (message_and_details.to_string(), None)
                    };
                    
                    logs.push(LogEntry {
                        id: format!("{}-{}", logs.len(), rand::random::<u32>()),
                        timestamp: timestamp.to_string(),
                        log_type: log_type.to_string(),
                        message,
                        details,
                    });
                }
            }
        }
    }
    
    // Reverse to show newest first
    logs.reverse();
    Ok(logs)
}

/// Clear all logs
#[tauri::command]
async fn clear_logs(app_handle: AppHandle) -> Result<(), String> {
    let log_path = get_log_path(&app_handle);
    if log_path.exists() {
        std::fs::remove_file(&log_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

// Helper function to add log entry (non-async version for use in sync contexts)
fn add_log_entry(app_handle: &AppHandle, log_type: &str, message: &str, details: Option<serde_json::Value>) -> Result<(), String> {
    let entry = LogEntry {
        id: format!("{}-{}", chrono::Local::now().timestamp_millis(), rand::random::<u32>()),
        timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        log_type: log_type.to_string(),
        message: message.to_string(),
        details,
    };
    
    write_log_to_file(app_handle, &entry)
}

/// Schedule shutdown after specified seconds
/// Uses a background thread to wait and then execute shutdown
#[tauri::command]
async fn schedule_shutdown(app_handle: AppHandle, seconds: u32) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Update state
        if let Ok(mut state) = SHUTDOWN_STATE.lock() {
            state.is_scheduled = true;
            state.remaining_seconds = seconds;
        }

        // Clone app_handle for the thread
        let app_handle_clone = app_handle.clone();

        // Spawn a background thread to handle the countdown
        thread::spawn(move || {
            let mut remaining = seconds;
            
            while remaining > 0 {
                thread::sleep(Duration::from_secs(1));
                
                // Check if shutdown was canceled
                if let Ok(state) = SHUTDOWN_STATE.lock() {
                    if !state.is_scheduled {
                        return;  // Exit thread if canceled
                    }
                }
                
                remaining -= 1;
                
                // Update remaining seconds in state
                if let Ok(mut state) = SHUTDOWN_STATE.lock() {
                    state.remaining_seconds = remaining;
                }
            }
            
            // Final check before executing shutdown
            if let Ok(state) = SHUTDOWN_STATE.lock() {
                if !state.is_scheduled {
                    return;
                }
            }
            
            #[cfg(debug_assertions)]
            {
                // Development mode: Only log, don't actually shutdown
                let _ = add_log_entry(&app_handle_clone, "shutdown_complete", "[开发模式] 时间到，跳过真实关机", None);
                println!("[开发模式] 时间已到，不执行真实关机命令。");
            }
            
            #[cfg(not(debug_assertions))]
            {
                // Release mode: Execute actual shutdown
                let _ = add_log_entry(&app_handle_clone, "shutdown_complete", "关机命令已执行，系统即将关闭", None);
                
                // Time's up - execute shutdown immediately (no Windows countdown)
                let _ = Command::new("shutdown")
                    .args(["/s", "/t", "0", "/f"])  // /t 0 = shutdown immediately
                    .creation_flags(0x08000000)
                    .output();
            }
            
            // Reset state
            if let Ok(mut state) = SHUTDOWN_STATE.lock() {
                state.is_scheduled = false;
                state.remaining_seconds = 0;
            }
        });

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Shutdown command is only supported on Windows".to_string())
    }
}

/// Cancel scheduled shutdown
#[tauri::command]
async fn cancel_shutdown() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Reset state first to stop the background thread countdown
        if let Ok(mut state) = SHUTDOWN_STATE.lock() {
            state.is_scheduled = false;
            state.remaining_seconds = 0;
        }

        // Try to cancel any pending Windows shutdown
        let _ = Command::new("shutdown")
            .args(["/a"])
            .creation_flags(0x08000000)
            .output();

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Cancel command is only supported on Windows".to_string())
    }
}

/// Get current shutdown status
#[tauri::command]
fn get_shutdown_status() -> (bool, u32) {
    if let Ok(state) = SHUTDOWN_STATE.lock() {
        (state.is_scheduled, state.remaining_seconds)
    } else {
        (false, 0)
    }
}

/// Update remaining seconds (called by frontend timer for display only)
#[tauri::command]
fn update_remaining_seconds(seconds: u32) {
    if let Ok(mut state) = SHUTDOWN_STATE.lock() {
        state.remaining_seconds = seconds;
        if seconds == 0 {
            state.is_scheduled = false;
        }
    }
}

/// Hide window to tray
#[tauri::command]
async fn hide_to_tray(window: tauri::WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

/// Show window from tray
#[tauri::command]
async fn show_from_tray(window: tauri::WebviewWindow) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::tray::{TrayIconBuilder, TrayIconEvent};
    use tauri::menu::{Menu, MenuItem};
    use tauri::Manager;

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // Focus existing window when second instance launches
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            schedule_shutdown,
            cancel_shutdown,
            get_shutdown_status,
            update_remaining_seconds,
            hide_to_tray,
            show_from_tray,
            add_log,
            get_logs,
            clear_logs
        ])
        .setup(|app| {
            // Create tray menu
            let show_i = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            // Create tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            #[cfg(not(debug_assertions))]
                            {
                                // Check if shutdown countdown is running before exiting
                                let should_handover = {
                                    if let Ok(state) = SHUTDOWN_STATE.lock() {
                                        state.is_scheduled && state.remaining_seconds > 0
                                    } else {
                                        false
                                    }
                                };

                                if should_handover {
                                    // Try to read settings to check if handover is enabled
                                    let handover_enabled = if let Ok(app_dir) = app.path().app_data_dir() {
                                        let settings_path = app_dir.join("settings.json");
                                        if settings_path.exists() {
                                            if let Ok(settings_content) = std::fs::read_to_string(&settings_path) {
                                                if let Ok(settings_json) = serde_json::from_str::<serde_json::Value>(&settings_content) {
                                                    settings_json.get("enableExitHandover")
                                                        .and_then(|v| v.as_bool())
                                                        .unwrap_or(true) // default to enabled
                                                } else {
                                                    true
                                                }
                                            } else {
                                                true
                                            }
                                        } else {
                                            true
                                        }
                                    } else {
                                        true
                                    };

                                    if handover_enabled {
                                        // Handover shutdown countdown to Windows before app exits
                                        let remaining_opt = SHUTDOWN_STATE.lock()
                                            .ok()
                                            .map(|s| s.remaining_seconds)
                                            .filter(|&s| s > 0);

                                        if let Some(remaining) = remaining_opt {
                                            let remaining_str = remaining.to_string();
                                            let _ = Command::new("shutdown")
                                                .args(["/s", "/t", &remaining_str, "/c", "Auto shutdown scheduled"])
                                                .creation_flags(0x08000000)
                                                .output();

                                            let msg = format!("倒计时移交Windows系统接管，剩余{}秒", remaining);
                                            let _ = add_log_entry(&app, "app_exit", &msg, None);
                                        }
                                    } else {
                                        let _ = add_log_entry(&app, "app_exit", "用户主动退出应用，未移交倒计时", None);
                                    }
                                } else {
                                    let _ = add_log_entry(&app, "app_exit", "用户主动退出应用", None);
                                }
                            }

                            #[cfg(debug_assertions)]
                            {
                                let _ = add_log_entry(&app, "app_exit", "用户主动退出应用", None);
                            }

                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::DoubleClick { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            use tauri::WindowEvent;
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Prevent window from closing, just hide it to tray
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
