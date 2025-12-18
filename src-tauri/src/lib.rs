//! feast - A Tauri v2 application

#![warn(clippy::all)]

pub mod commands;
pub mod correlation;
pub mod db;
pub mod error;
pub mod http;
pub mod logging;
pub mod parser;
pub mod utils;

use commands::{
    add_quick_list_item, add_quick_list_to_shopping, add_shopping_item, create_ingredient,
    create_item, create_manual_item, create_meal_plan, create_quick_list, create_recipe,
    create_shopping_list, delete_item, delete_manual_item, delete_meal_plan, delete_quick_list,
    delete_recipe, delete_shopping_list, get_aggregated_shopping_list, get_ingredients, get_items,
    get_manual_items, get_meal_plans, get_or_create_ingredient, get_quick_lists, get_recipe,
    get_recipes, get_shopping_lists, greet, import_recipe_from_url, log_from_frontend,
    move_shopping_item, remove_quick_list_item, restore_shopping_item, soft_delete_shopping_item,
    update_manual_item, update_meal_plan, update_quick_list, update_quick_list_item, update_recipe,
    update_shopping_item,
};
use tauri::Manager;

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin({
            use tauri_plugin_log::{Builder, RotationStrategy, Target, TargetKind};
            use crate::logging::{LogConfig, json_format, MAX_LOG_FILE_SIZE, LOG_FILE_NAME};

            // Load config - we don't have app paths yet, so use defaults initially
            // Config is loaded again in setup() with proper paths
            let config = LogConfig::default();

            let mut builder = Builder::new()
                .level(LogConfig::parse_level(&config.default_level))
                .max_file_size(MAX_LOG_FILE_SIZE as u128)
                .rotation_strategy(RotationStrategy::KeepAll)
                .format(json_format);

            // Apply per-module log levels
            for (module, level) in &config.module_levels {
                builder = builder.level_for(module.clone(), LogConfig::parse_level(level));
            }

            // Build targets based on config
            let mut targets = vec![];

            if config.file_enabled {
                targets.push(Target::new(TargetKind::LogDir {
                    file_name: Some(LOG_FILE_NAME.into()),
                }));
            }

            if config.console_enabled {
                targets.push(Target::new(TargetKind::Stdout));
            }

            // Always include webview for frontend logging bridge
            targets.push(Target::new(TargetKind::Webview));

            builder = builder.targets(targets);

            builder.build()
        })
        .setup(|app| {
            use crate::logging::LogConfig;

            // Get app directories
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            let app_config_dir = app
                .path()
                .app_config_dir()
                .expect("Failed to get app config directory");

            let app_log_dir = app
                .path()
                .app_log_dir()
                .expect("Failed to get app log directory");

            // Load logging config (for reference - plugin already initialized)
            let log_config = LogConfig::load(&app_config_dir);

            // Initialize database
            tauri::async_runtime::block_on(async {
                if let Err(e) = db::init_db(&app_data_dir).await {
                    log::error!("Failed to initialize database: {e}");
                    panic!("Failed to initialize database: {e}");
                }
            });

            log::info!(
                "Application initialized - data_dir: {:?}, log_dir: {:?}, log_level: {}",
                app_data_dir,
                app_log_dir,
                log_config.default_level
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            log_from_frontend,
            get_items,
            create_item,
            delete_item,
            // Recipe commands
            get_recipes,
            get_recipe,
            create_recipe,
            update_recipe,
            delete_recipe,
            import_recipe_from_url,
            // Ingredient commands
            get_ingredients,
            create_ingredient,
            get_or_create_ingredient,
            // Meal plan commands
            get_meal_plans,
            create_meal_plan,
            update_meal_plan,
            delete_meal_plan,
            // Shopping list commands
            get_shopping_lists,
            create_shopping_list,
            delete_shopping_list,
            add_shopping_item,
            update_shopping_item,
            soft_delete_shopping_item,
            restore_shopping_item,
            move_shopping_item,
            get_aggregated_shopping_list,
            // Quick list commands
            get_quick_lists,
            create_quick_list,
            update_quick_list,
            delete_quick_list,
            add_quick_list_item,
            update_quick_list_item,
            remove_quick_list_item,
            add_quick_list_to_shopping,
            // Manual item commands
            get_manual_items,
            create_manual_item,
            update_manual_item,
            delete_manual_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
