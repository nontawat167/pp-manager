// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod command;
mod domain;
mod error;
mod infra;
mod ipc;
mod port;
mod service;
mod repository;
mod store;
mod tauriop;
mod usecase;
mod utils;

use std::sync::Arc;

pub use error::{Error, Result};
use infra::{sku::SkuRepositoryImpl, tag::TagRepositoryImpl};
use port::repostiory::{SkuRepository, TagRepository};
use repository::RepoManagerBuider;
use service::TagService;
use store::Store;
// use ipc::sku::CreateSkuInput;
use store::{connection::establish_connection, database::DatabaseContext};
use store::diesel::DieselMigrationEngine;
use store::migrator::Migrator;
use tauri::async_runtime::Mutex;
use tauriop::create_builder;
use usecase::{SkuUseCase, TagUseCase};

use repository::TagRepository as TagRepo;

// use crate::ipc::sku::SearchSkusInput;

#[tokio::main]
async fn main() {
    let engine = DieselMigrationEngine::new(String::from("D:\\test.db"));
    Migrator::new().run_migrations(engine).unwrap();

    let db: DatabaseContext = DatabaseContext::new(String::from("D:\\test.db"));
    // let _ = DatabaseContext::run_migrations(&mut db.establish_connection());
    let db_context: Arc<DatabaseContext> = Arc::new(db);

    // init repo
    let sku_repo: Arc<dyn SkuRepository> =
        Arc::new(SkuRepositoryImpl::new(Arc::clone(&db_context)));
    let sku_repo_mutex = Arc::new(Mutex::new(sku_repo));

    let tag_repo: Arc<dyn TagRepository> =
        Arc::new(TagRepositoryImpl::new(Arc::clone(&db_context)));
    let tag_repo_mutex = Arc::new(Mutex::new(tag_repo));

    // init usecase
    let sku_usecase = Arc::new(SkuUseCase::new(Arc::clone(&sku_repo_mutex)));
    let tag_usecase = Arc::new(TagUseCase::new(Arc::clone(&tag_repo_mutex)));

    /* ------------------------------------ start new dependecies implementation --------------------------------------------------------- */
    let conn = establish_connection().await;
    let store = Store::new(conn);
    let tag_repo = TagRepo::new(store);

    let repositories = RepoManagerBuider::new().tag(tag_repo).build();
    let tag_service = TagService::new(repositories);

    let services = service::ServiceManagerBuider::new()
        .tag(tag_service)
        .build();

    let app = app::AppContext::new(services);

     /* ------------------------------------ end new dependecies implementation --------------------------------------------------------- */

    let tauri_builder = create_builder();
    tauri_builder
        .manage(sku_usecase)
        .manage(tag_usecase)
        .manage(app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
