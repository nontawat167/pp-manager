use async_trait::async_trait;

use crate::error::Result;
use crate::domain::sku::Sku;

use super::response::SearchResult;
use super::sku::SkuSearchInput;

#[async_trait]
pub trait SkuRepository {
    async fn save(&self, sku: &mut Sku) -> Result<()>;
    async fn find(&self, search_input: SkuSearchInput) -> Result<SearchResult<Sku>>;
}
