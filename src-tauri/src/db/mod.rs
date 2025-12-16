//! Database module

pub mod ingredients;
pub mod items;
pub mod manual_items;
pub mod meal_plans;
pub mod pool;
pub mod quick_lists;
pub mod recipes;
pub mod shopping_lists;

pub use pool::{get_db_pool, init_db};
