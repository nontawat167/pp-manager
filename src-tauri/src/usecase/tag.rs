use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::tag::Tag;
use crate::error::Result;
use crate::ipc::tag::{CreateTagInput, SearchTagsInput};
use crate::ipc::TagSearchResponse;
use crate::port::input::SearchOperator;
use crate::port::repostiory::TagRepository;
use crate::port::tag::{SqlOrder, TagOrderBy, TagSearchInput};

pub struct TagUseCase {
    tag_repo: Arc<Mutex<Arc<dyn TagRepository>>>,
}

impl TagUseCase {
    pub fn new(tag_repo: Arc<Mutex<Arc<dyn TagRepository>>>) -> Self {
        Self {
            tag_repo: tag_repo.clone(),
        }
    }

    pub async fn create_tag(&self, input: CreateTagInput) -> Result<Tag> {
        let mut tag = Tag::new(input.name, input.kind, input.color);
        let sku_repo = self.tag_repo.lock().await;
        let _ = sku_repo.save(&mut tag);
        Ok(tag)
    }

    pub async fn search_tags(&self, input: SearchTagsInput) -> Result<TagSearchResponse> {
        let mut search = TagSearchInput::default();

        if let Some(k) = input.kind {
            search.kind = Some(SearchOperator::Equal(k));
        }

        if let Some(p) = input.page {
            search.page = Some(p)
        }
        if let Some(p) = input.per_page {
            search.per_page = Some(p)
        }

        if let Some(o) = input.order {
            match &o[..] {
                "name:ASC" => search.order_by = Some(TagOrderBy::Name(SqlOrder::ASC)),
                "name:DESC" => search.order_by = Some(TagOrderBy::Name(SqlOrder::DESC)),
                _ => {}
            }
        }

        let tag_repo = self.tag_repo.lock().await;
        let search_result = tag_repo.find(search);
        match search_result {
            Ok(rs) => {
                return Ok(TagSearchResponse {
                    total: rs.total,
                    tags: rs.items,
                })
            }
            Err(err) => return Err(format!("usecase Error: {}", err).into()).into(),
        };
    }
}
