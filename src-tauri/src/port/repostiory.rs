use crate::domain::sku::Sku;
use crate::error::Result;

use super::response::SearchResult;
use super::sku::SkuSearchInput;

pub trait SkuRepository: Send + Sync {
    fn save(&self, sku: &mut Sku) -> Result<()>;
    fn find(&self, search_input: SkuSearchInput) -> Result<SearchResult<Sku>>;
}
