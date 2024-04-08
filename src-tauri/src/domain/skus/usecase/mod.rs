use crate::{repository::sku::SkuRepository, Result};

use super::Sku;

pub async fn search_skus(repository: &dyn SkuRepository) -> Result<(i32, Vec<Sku>)> {
    repository.find().await
}
