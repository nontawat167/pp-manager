use tauri::generate_handler;
use tauri::{Builder, Wry};

use crate::command::*;

pub fn create_builder() -> Builder<Wry> {
    let tauri_builder = Builder::default().invoke_handler(generate_handler![
        greet,
        // sku
        search_skus,
        create_sku,
        // tag
        search_tags,
        create_tag
    ]);
    tauri_builder
}
