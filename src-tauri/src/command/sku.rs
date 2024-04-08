use std::sync::Arc;

use tauri::{command, State};

use crate::domain::sku::Sku;
use crate::ipc::sku::{CreateSkuInput, SearchSkusInput};
use crate::ipc::{IpcResponse, SkuSearchResponse, SkuSearchResponse2};
use crate::repository::RepoManager2;
use crate::usecase::SkuUseCase;
use crate::Result as AppResult;

use crate::domain::skus::usecase::search_skus as domain_search_sku;

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

#[command]
pub async fn search_skus2<'r>(
    state: State<'r, RepoManager2>,
) -> Result<IpcResponse<SkuSearchResponse2>, ()> {
    let arc = state.inner().sku().to_owned();
    let sku_repo = arc.as_ref();
    let skus = domain_search_sku(sku_repo).await.unwrap();
    let response: AppResult<SkuSearchResponse2> = Ok(skus.into());
    return Ok(response.into());
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
