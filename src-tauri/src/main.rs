#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod config;

use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, State,
};
use tokio::sync::Mutex;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenBalance {
    pub provider: String,
    pub balance: Option<f64>,
    pub used: Option<f64>,
    pub limit: Option<f64>,
    pub error: Option<String>,
    pub last_updated: Option<String>,
}

pub struct AppState {
    config: Arc<Mutex<config::Config>>,
}

#[tauri::command]
async fn fetch_all_balances(state: State<'_, AppState>) -> Result<Vec<TokenBalance>, String> {
    let config = state.config.lock().await.clone();

    let mut results = Vec::new();

    // Fetch OpenAI balance
    if let Some(api_key) = config.openai_api_key {
        match api::openai::fetch_balance(&api_key).await {
            Ok(balance) => {
                results.push(TokenBalance {
                    provider: "OpenAI".to_string(),
                    balance: Some(balance.remaining),
                    used: Some(balance.used),
                    limit: None,
                    error: None,
                    last_updated: Some(chrono::Local::now().format("%H:%M:%S").to_string()),
                });
            }
            Err(e) => {
                results.push(TokenBalance {
                    provider: "OpenAI".to_string(),
                    balance: None,
                    used: None,
                    limit: None,
                    error: Some(e),
                    last_updated: None,
                });
            }
        }
    } else {
        results.push(TokenBalance {
            provider: "OpenAI".to_string(),
            balance: None,
            used: None,
            limit: None,
            error: Some("API key not configured".to_string()),
            last_updated: None,
        });
    }

    // Fetch Copilot balance
    if let Some(token) = config.github_token {
        match api::copilot::fetch_usage(&token).await {
            Ok(usage) => {
                results.push(TokenBalance {
                    provider: "Copilot".to_string(),
                    balance: None,
                    used: Some(usage.used as f64),
                    limit: Some(usage.limit as f64),
                    error: None,
                    last_updated: Some(chrono::Local::now().format("%H:%M:%S").to_string()),
                });
            }
            Err(e) => {
                results.push(TokenBalance {
                    provider: "Copilot".to_string(),
                    balance: None,
                    used: None,
                    limit: None,
                    error: Some(e),
                    last_updated: None,
                });
            }
        }
    } else {
        results.push(TokenBalance {
            provider: "Copilot".to_string(),
            balance: None,
            used: None,
            limit: None,
            error: Some("GitHub token not configured".to_string()),
            last_updated: None,
        });
    }

    Ok(results)
}

#[tauri::command]
async fn open_settings(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
        let _ = app.emit("open-settings", ());
    }
}

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<config::Config, String> {
    let config = state.config.lock().await.clone();
    Ok(config)
}

#[tauri::command]
async fn save_config(
    state: State<'_, AppState>,
    openai_api_key: Option<String>,
    github_token: Option<String>,
    refresh_interval: Option<u64>,
) -> Result<(), String> {
    let mut current = state.config.lock().await;

    if let Some(key) = openai_api_key {
        current.openai_api_key = if key.is_empty() { None } else { Some(key) };
    }
    if let Some(token) = github_token {
        current.github_token = if token.is_empty() { None } else { Some(token) };
    }
    if let Some(interval) = refresh_interval {
        current.refresh_interval = interval;
    }

    let config_path = config::get_config_path();
    current.save(&config_path)?;

    Ok(())
}

fn main() {
    let config_path = config::get_config_path();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            config: Arc::new(Mutex::new(config::Config::load(&config_path))),
        })
        .setup(|app| {
            // Create tray menu - 右键菜单：配置 + 退出
            let settings_item = MenuItem::with_id(app, "settings", "⚙️ 配置", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "❌ 退出", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false) // 右键显示菜单
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "settings" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                            let _ = app.emit("open-settings", ());
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // 左键点击显示主窗口
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch_all_balances, open_settings, get_config, save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}