mod commands;
mod db;
mod entities;
mod installer;
mod manifest;
mod migrations;
mod progress;

use tauri::{Manager, WebviewWindow};

use crate::commands::AppState;

pub fn run() {
    // Tauri doesn't support Wayland properly due to problems upstream, this will allow it to run on Wayland.
    std::env::set_var("__GL_THREADED_OPTIMIZATIONS", "1");
    std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "linux")]
            {
                // Tauri's window decoration sucks by default, this fixes it.
                // Unsure of the affects this has on other distributions, might need to implement a check.
                // Seems to have a performance impact for loading the window contents.

                // TODO: Upgrade Tauri to '>2.11.x' for Tao 0.36.
                //  This should fix the problem without needing this.
                //  Although testing dev branch, it's slow just like doing this.

                use gtk::prelude::GtkWindowExt;
                use gtk::ApplicationWindow;

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

            let db_state = app.state::<AppState>();

            match tauri::async_runtime::block_on(installer::ensure_all_installed(&db_state.db, &resource_dir, &app_data_dir)) {
                Ok(results) => {
                    for result in results.iter().filter(|r| r.changed) {
                        eprintln!("reconciled course '{}' -> version {}", result.id, result.version)
                    }
                }

                Err(err) => {
                    eprintln!("warning: failed to reconcile bundled courses: {err}")
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::check_courses,
            commands::update_course
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
