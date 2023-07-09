use std::sync::Arc;

use tauri::{command, State};

use crate::domain::sku::Sku;
use crate::ipc::sku::{CreateSkuInput, SearchSkusInput};
use crate::ipc::{IpcResponse, SkuSearchResponse};
use crate::usecase::SkuUseCase;

// Query
#[command]
pub async fn search_skus<'r>(
    input: SearchSkusInput,
    usecases: State<'r, Arc<SkuUseCase>>,
) -> Result<IpcResponse<SkuSearchResponse>, ()> {
    let usecases = usecases.clone().inner();
    let result = usecases.search_skus(input.clone()).await;
    let rs: IpcResponse<SkuSearchResponse> = result.into();
    Ok(rs)
}

// Mutation
#[command]
pub async fn create_sku<'r>(
    input: CreateSkuInput,
    usecases: State<'r, Arc<SkuUseCase>>,
) -> Result<IpcResponse<Sku>, ()> {
    let usecases = usecases.clone().inner();
    let result = usecases.create_sku(input.clone()).await;
    let rs: IpcResponse<Sku> = result.into();
    Ok(rs)
}
