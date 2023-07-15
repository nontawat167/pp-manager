use crate::domain::sku::Sku;
use crate::domain::tag::Tag;
use crate::error::Result;

use super::response::SearchResult;
use super::sku::SkuSearchInput;
use super::tag::TagSearchInput;

pub trait SkuRepository: Send + Sync {
    fn save(&self, sku: &mut Sku) -> Result<()>;
    fn find(&self, search_input: SkuSearchInput) -> Result<SearchResult<Sku>>;
}

pub trait TagRepository: Send + Sync {
    fn save(&self, tag: &mut Tag) -> Result<()>;
    fn find(&self, search_input: TagSearchInput) -> Result<SearchResult<Tag>>;
}
