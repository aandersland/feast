//! feast - A Tauri v2 application

#![warn(clippy::all)]

pub mod commands;
pub mod db;
pub mod error;
pub mod http;
pub mod parser;
pub mod utils;

use commands::{
    add_quick_list_item, add_quick_list_to_shopping, add_shopping_item, create_ingredient,
    create_item, create_manual_item, create_meal_plan, create_quick_list, create_recipe,
    create_shopping_list, delete_item, delete_manual_item, delete_meal_plan, delete_quick_list,
    delete_recipe, delete_shopping_list, get_aggregated_shopping_list, get_ingredients, get_items,
    get_manual_items, get_meal_plans, get_or_create_ingredient, get_quick_lists, get_recipe,
    get_recipes, get_shopping_lists, greet, import_recipe_from_url, move_shopping_item,
    remove_quick_list_item, restore_shopping_item, soft_delete_shopping_item, update_manual_item,
    update_meal_plan, update_quick_list, update_quick_list_item, update_recipe,
    update_shopping_item,
};
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
