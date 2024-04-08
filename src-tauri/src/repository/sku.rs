use async_trait::async_trait;

use crate::{domain::skus::Sku, Result};

#[async_trait]
pub trait SkuRepository: Send + Sync  {
    async fn find(&self) -> Result<(i32, Vec<Sku>)>;
}