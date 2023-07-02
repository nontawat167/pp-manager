use tauri::generate_handler;
use tauri::{Builder, Wry};

use crate::command::*;

pub fn create_builder() -> Builder<Wry> {
    let tauri_builder = Builder::default().invoke_handler(generate_handler![greet]);
    tauri_builder
}
