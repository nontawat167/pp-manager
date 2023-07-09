// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod domain;
mod error;
mod infra;
mod ipc;
mod port;
mod repository;
mod store;
mod tauriop;
mod usecase;
mod utils;

use std::sync::Arc;

pub use error::{Error, Result};
use infra::sku::SkuRepositoryImpl;
use port::repostiory::SkuRepository;
// use ipc::sku::CreateSkuInput;
use store::database::DatabaseContext;
use tauri::async_runtime::Mutex;
use tauriop::create_builder;
use usecase::SkuUseCase;

// use crate::ipc::sku::SearchSkusInput;

#[tokio::main]
async fn main() {
    let db = DatabaseContext::new(String::from("D:\\test.db"));
    let _ = DatabaseContext::run_migrations(&mut db.establish_connection());
    let db_context = Arc::new(db);

    let sku_repo: Arc<dyn SkuRepository> =
        Arc::new(SkuRepositoryImpl::new(Arc::clone(&db_context)));
    let sku_repo_mutex = Arc::new(Mutex::new(sku_repo));
    let sku_usecase = Arc::new(SkuUseCase::new(Arc::clone(&sku_repo_mutex)));

    let tauri_builder = create_builder();
    tauri_builder
        .manage(sku_usecase)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
