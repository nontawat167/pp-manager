use tauri::command;

use crate::ipc::IpcResponse;
use crate::usecase::get_all_sku;
use crate::model::*;

#[command]
pub fn get_skus() -> IpcResponse<Vec<Sku>> {
    get_all_sku().into()
}
