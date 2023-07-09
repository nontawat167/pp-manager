use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::sku::Sku;
use crate::error::Result;
use crate::ipc::sku::{CreateSkuInput, SearchSkusInput};
use crate::ipc::SkuSearchResponse;
use crate::port::input::SearchOperator;
use crate::port::repostiory::SkuRepository;
use crate::port::sku::{SkuSearchInput, SqlOrder};
use crate::port::SkuOrderBy;

pub struct SkuUseCase {
    sku_repo: Arc<Mutex<Arc<dyn SkuRepository>>>,
}

impl SkuUseCase {
    pub fn new(sku_repo: Arc<Mutex<Arc<dyn SkuRepository>>>) -> Self {
        Self {
            sku_repo: sku_repo.clone(),
        }
    }

    pub async fn create_sku(&self, input: CreateSkuInput) -> Result<Sku> {
        let mut sku = Sku::new(input.name, input.price, input.product_type);
        let sku_repo = self.sku_repo.lock().await;
        let _ = sku_repo.save(&mut sku);
        Ok(sku)
    }

    pub async fn search_skus(&self, input: SearchSkusInput) -> Result<SkuSearchResponse> {
        let mut search = SkuSearchInput::default();

        if let Some(n) = input.name {
            search.name = Some(SearchOperator::Equal(n));
        }
        if let Some(p) = input.price {
            search.price = Some(SearchOperator::Equal(p));
        }
        if let Some(p) = input.product_type {
            search.product_type = Some(SearchOperator::Equal(p));
        }

        if let Some(p) = input.page {
            search.page = Some(p)
        }
        if let Some(p) = input.per_page {
            search.per_page = Some(p)
        }

        if let Some(o) = input.order {
            match &o[..] {
                "created_at:ASC" => search.order_by = Some(SkuOrderBy::CreatedAt(SqlOrder::ASC)),
                "created_at:DESC" => search.order_by = Some(SkuOrderBy::CreatedAt(SqlOrder::DESC)),
                "updated_at:ASC" => search.order_by = Some(SkuOrderBy::UpdatedAt(SqlOrder::ASC)),
                "updated_at:DESC" => search.order_by = Some(SkuOrderBy::UpdatedAt(SqlOrder::DESC)),
                _ => {}
            }
        }

        let sku_repo = self.sku_repo.lock().await;
        let search_result = sku_repo.find(search);
        match search_result {
            Ok(rs) => {
                return Ok(SkuSearchResponse {
                    total: rs.total,
                    skus: rs.items,
                })
            }
            Err(err) => return Err(format!("usecase Error: {}", err).into()).into(),
        };
    }
}
