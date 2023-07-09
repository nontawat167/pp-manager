use tauri::generate_handler;
use tauri::{Builder, Wry};

use crate::command::*;

pub fn create_builder() -> Builder<Wry> {
    let tauri_builder =
        Builder::default().invoke_handler(generate_handler![greet, search_skus, create_sku]);
    tauri_builder
}
