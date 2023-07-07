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

pub use error::{Error, Result};
use infra::sku::SkuRepositoryImpl;
use ipc::sku::CreateSkuInput;
use std::sync::Arc;
use store::database::DatabaseContext;
use tauri::async_runtime::Mutex;
use tauriop::create_builder;
use usecase::SkuUseCase;

use crate::ipc::sku::SearchSkusInput;

#[tokio::main]
async fn main() {
    let db = DatabaseContext::new(String::from("D:\\test.db"));
    let _ = DatabaseContext::run_migrations(&mut db.establish_connection());
    let db_context = Arc::new(Mutex::new(db));

    let sku_repo = Box::new(SkuRepositoryImpl::new(Arc::clone(&db_context)));
    let sku_usecase = SkuUseCase::new(sku_repo);

    let mock_input = CreateSkuInput {
        name: "test_name2".to_owned(),
        price: 30,
        product_type: "TEST_TYPE".to_owned(),
    };

    let _ = &sku_usecase.create_sku(mock_input).await;

    // println!("{:?}", result);

    let mock_search = SearchSkusInput {
        name: Some("test_name2".to_owned()),
        price: None,
        product_type: None,

        page: None,
        per_page: None,
        order_by: None,
    };
    let search = &sku_usecase.search_skus(mock_search).await;
    println!("{:?}", search);

    // let res = store.find().await;
    // let sks = res.unwrap();
    // for s in sks.into_iter() {
    //     println!("{:?}", s)
    // }
    let tauri_builder = create_builder();
    tauri_builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
