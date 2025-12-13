//! feast - A Tauri v2 application

#![warn(clippy::all)]

pub mod commands;
pub mod db;
pub mod error;

use commands::{create_item, delete_item, get_items, greet};
use tauri::Manager;

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            // Initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            tauri::async_runtime::block_on(async {
                if let Err(e) = db::init_db(&app_data_dir).await {
                    log::error!("Failed to initialize database: {e}");
                    panic!("Failed to initialize database: {e}");
                }
            });

            log::info!("Application initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_items,
            create_item,
            delete_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
