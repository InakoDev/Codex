mod db;
mod installer;
mod manifest;
mod commands;

use gtk::ApplicationWindow;
use tauri::{Manager, WebviewWindow};
use crate::commands::AppState;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

pub fn run() {
    // Tauri doesn't support Wayland properly due to problems upstream, this will allow it to run on Wayland.
    std::env::set_var("__GL_THREADED_OPTIMIZATIONS", "0");
    std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "linux")]
            {
                // Tauri's window decoration sucks by default, this fixes it. No touchy.
                // Unsure of the affects this has on other distributions, might need to implement a check.
                use gtk::prelude::GtkWindowExt;
                let window: WebviewWindow = app
                    .get_webview_window("main")
                    .ok_or("'main' WebviewWindow not found.")?;

                let gtk_window: ApplicationWindow = window.gtk_window()?;
                gtk_window.set_titlebar(Option::<&gtk::Widget>::None);
            }

            // Setting up the database for Codex.
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|err| format!("failed to resolve app data dir: {err}"))?;
            let db = tauri::async_runtime::block_on(db::connect(&app_data_dir))
                .map_err(|err| format!("failed to connect to database: {err}"))?;

            app.manage(AppState { db });

            // Ensuring that all courses are installed into application data directory.
            let resource_dir = app
                .path()
                .resource_dir()
                .map_err(|err| format!("failed to resolve resource dir: {err}"))?;

            if let Err(err) = installer::ensure_all_installed(&resource_dir, &app_data_dir) {
                eprintln!("warning: failed to pre-install bundled courses: {err}")
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
