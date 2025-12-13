//! Tauri command handlers

pub mod items;

use tauri::command;

pub use items::{create_item, delete_item, get_items};

/// Greet a user by name
#[command]
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to feast.")
}
