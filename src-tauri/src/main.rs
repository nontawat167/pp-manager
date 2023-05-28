// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod store;
mod tauriop;
mod ipc;
mod error;
mod usecase;
mod model;

pub use error::{Error, Result};
use store::prepare_database;
use tauriop::create_builder;

fn main() {
    prepare_database().unwrap();
    let tauri_builder = create_builder();
    tauri_builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
