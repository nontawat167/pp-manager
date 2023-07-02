// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod error;
mod ipc;
mod model;
mod store;
mod tauriop;
mod usecase;
use diesel::prelude::*;
pub use error::{Error, Result};
use model::SKUModel;
use store::database::Database;
use store::schema::skus::dsl::*;
use tauriop::create_builder;

fn main() {
    let mut db = Database::new(String::from("D:\\test.db"));
    let sks = skus.load::<SKUModel>(&mut db.connection).expect("error");
    for s in sks.into_iter() {
        println!("{:?}", s)
    }
    let tauri_builder = create_builder();
    tauri_builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
