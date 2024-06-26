use std::sync::Arc;

use tauri::{command, State};

use crate::app::AppContext;
use crate::domain::tag::{NewTag, Tag};
use crate::domain::tags::usecase::get_all_tag as domain_get_all_tag;
use crate::domain::tags::Tag as DomainTag;
use crate::ipc::tag::{CreateTagInput, SearchTagsInput};
use crate::ipc::{IpcResponse, TagSearchResponse};
use crate::repository::RepoManager2;
use crate::usecase::TagUseCase;

use crate::error::Result as AppResult;

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

#[command]
pub async fn get_all_tag<'r>(state: State<'r, AppContext>) -> Result<IpcResponse<Vec<NewTag>>, ()> {
    let app = state.clone().inner();

    let service = app.services().tag_service();
    let tags = service.get_all_tag().await;

    let app_rs: AppResult<Vec<NewTag>> = Ok(tags);
    let rs: IpcResponse<Vec<NewTag>> = app_rs.into();
    return Ok(rs);
}

#[command]
pub async fn get_all_tag2<'r>(
    state: State<'r, RepoManager2>,
) -> Result<IpcResponse<Vec<DomainTag>>, ()> {
    let arc_tag_repo = state.inner().tag().to_owned();
    let tag_repo = arc_tag_repo.as_ref();

    let tags = domain_get_all_tag(tag_repo).await;

    return Ok(tags.into());
}
