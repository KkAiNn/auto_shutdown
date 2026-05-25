use std::sync::Mutex;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

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

/// Schedule shutdown after specified seconds
/// Uses a background thread to wait and then execute shutdown
#[tauri::command]
async fn schedule_shutdown(seconds: u32) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Update state
        if let Ok(mut state) = SHUTDOWN_STATE.lock() {
            state.is_scheduled = true;
            state.remaining_seconds = seconds;
        }

        // Spawn a background thread to handle the countdown
        thread::spawn(move || {
            let mut remaining = seconds;
            
            while remaining > 0 {
                thread::sleep(Duration::from_secs(1));
                remaining -= 1;
                
                // Update remaining seconds in state
                if let Ok(mut state) = SHUTDOWN_STATE.lock() {
                    state.remaining_seconds = remaining;
                }
            }
            
            // Time's up - execute shutdown immediately (no Windows countdown)
            let _ = Command::new("shutdown")
                .args(["/s", "/t", "0", "/f"])  // /t 0 = shutdown immediately
                .creation_flags(0x08000000)
                .output();
            
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
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            schedule_shutdown,
            cancel_shutdown,
            get_shutdown_status,
            update_remaining_seconds,
            hide_to_tray,
            show_from_tray
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
