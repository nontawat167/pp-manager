use crate::domain::sku::Sku;
use crate::ipc::sku::{CreateSkuInput, SearchSkusInput};
use crate::ipc::{IpcResponse, SkuSearchResponse};
use crate::port::input::SearchOperator;
use crate::port::repostiory::SkuRepository;
use crate::port::sku::SkuSearchInput;
use crate::port::SkuOrderBy;

pub struct SkuUseCase {
    sku_repo: Box<dyn SkuRepository>,
}

impl SkuUseCase {
    pub fn new(sku_repo: Box<dyn SkuRepository>) -> Self {
        Self { sku_repo }
    }

    pub async fn create_sku(&self, input: CreateSkuInput) -> IpcResponse<Sku> {
        let mut sku = Sku::new(input.name, input.price, input.product_type);
        let _ = self.sku_repo.save(&mut sku).await;
        Ok(sku).into()
    }

    pub async fn search_skus(&self, input: SearchSkusInput) -> IpcResponse<SkuSearchResponse> {
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

        if let Some(o) = input.order_by {
            match &o[..] {
                "created_at" => search.order_by = Some(SkuOrderBy::CreatedAt),
                "updated_at" => search.order_by = Some(SkuOrderBy::UpdatedAt),
                _ => {}
            }
        }

        let search_result = self.sku_repo.find(search).await;
        match search_result {
            Ok(rs) => {
                return Ok(SkuSearchResponse {
                    total: rs.total,
                    skus: rs.items,
                })
                .into()
            }
            Err(err) => return Err(format!("usecase Error: {}", err).into()).into(),
        };
    }
}
