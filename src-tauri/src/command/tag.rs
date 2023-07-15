use std::sync::Arc;

use tauri::{command, State};

use crate::domain::tag::Tag;
use crate::ipc::tag::{CreateTagInput, SearchTagsInput};
use crate::ipc::{IpcResponse, TagSearchResponse};
use crate::usecase::TagUseCase;

// Query
#[command]
pub async fn search_tags<'r>(
    input: SearchTagsInput,
    usecases: State<'r, Arc<TagUseCase>>,
) -> Result<IpcResponse<TagSearchResponse>, ()> {
    let usecases = usecases.clone().inner();
    let result = usecases.search_tags(input.clone()).await;
    let rs: IpcResponse<TagSearchResponse> = result.into();
    Ok(rs)
}

// Mutation
#[command]
pub async fn create_tag<'r>(
    input: CreateTagInput,
    usecases: State<'r, Arc<TagUseCase>>,
) -> Result<IpcResponse<Tag>, ()> {
    let usecases = usecases.clone().inner();
    let result = usecases.create_tag(input.clone()).await;
    let rs: IpcResponse<Tag> = result.into();
    Ok(rs)
}
