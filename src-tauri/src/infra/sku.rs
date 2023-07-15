use std::sync::Arc;

use diesel::prelude::*;
use diesel::ExpressionMethods;

use crate::domain::sku::Sku;
use crate::domain::NEW_ENTITY_ID;
use crate::error::Result;
use crate::port::input::SearchOperator;
use crate::port::repostiory::SkuRepository;
use crate::port::response::SearchResult;
use crate::port::sku::SkuOrderBy;
use crate::port::sku::SkuSearchInput;
use crate::port::sku::SqlOrder;
use crate::repository::schema::skus;
use crate::repository::schema::skus::*;
use crate::store::database::DatabaseContext;
use crate::utils::generate_uuid;
use crate::utils::get_timestamp;

pub struct SkuRepositoryImpl {
    db_context: Arc<DatabaseContext>,
}

impl SkuRepositoryImpl {
    pub fn new(db_context: Arc<DatabaseContext>) -> Self {
        Self { db_context }
    }
}

impl SkuRepositoryImpl {
    fn apply_search_filters<'a>(
        query: skus::BoxedQuery<'a, diesel::sqlite::Sqlite>,
        search_input: &'a SkuSearchInput,
    ) -> skus::BoxedQuery<'a, diesel::sqlite::Sqlite> {
        let mut filtered_query = query;

        if let Some(n) = &search_input.name {
            match n {
                SearchOperator::Equal(search_name) => {
                    let search_str = format!("{}{}", search_name, "%");
                    filtered_query = filtered_query.filter(skus::name.like(search_str));
                }
            }
        }

        if let Some(p) = &search_input.price {
            match p {
                SearchOperator::Equal(search_price) => {
                    filtered_query = filtered_query.filter(skus::price.eq(search_price));
                }
            }
        }

        if let Some(pro) = &search_input.product_type {
            match pro {
                SearchOperator::Equal(search_product_type) => {
                    filtered_query =
                        filtered_query.filter(skus::product_type.eq(search_product_type));
                }
            }
        }

        filtered_query
    }

    fn apply_paging<'a>(
        query: skus::BoxedQuery<'a, diesel::sqlite::Sqlite>,
        search_input: &'a SkuSearchInput,
    ) -> skus::BoxedQuery<'a, diesel::sqlite::Sqlite> {
        let mut new_query = query;
        let page: u32 = search_input.page.unwrap_or(1);
        let per_page = search_input.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;
        new_query = new_query.offset(offset as i64).limit(per_page as i64);
        new_query
    }

    fn apply_ordering<'a>(
        query: skus::BoxedQuery<'a, diesel::sqlite::Sqlite>,
        search_input: &'a SkuSearchInput,
    ) -> skus::BoxedQuery<'a, diesel::sqlite::Sqlite> {
        let mut new_query = query;

        if let Some(order_by) = &search_input.order_by {
            match order_by {
                SkuOrderBy::CreatedAt(SqlOrder::ASC) => {
                    new_query = new_query.order(created_at.asc())
                }
                SkuOrderBy::CreatedAt(SqlOrder::DESC) => {
                    new_query = new_query.order(created_at.desc())
                }
                SkuOrderBy::UpdatedAt(SqlOrder::ASC) => {
                    new_query = new_query.order(updated_at.asc())
                }
                SkuOrderBy::UpdatedAt(SqlOrder::DESC) => {
                    new_query = new_query.order(updated_at.desc())
                }
            }
        }
        new_query
    }
}

impl SkuRepository for SkuRepositoryImpl {
    fn save(&self, sku: &mut Sku) -> Result<()> {
        let mut connection = self.db_context.establish_connection();

        let result = if String::from(NEW_ENTITY_ID).eq(sku.id()) {
            diesel::insert_into(table)
                .values((
                    id.eq(generate_uuid()),
                    created_at.eq(sku.created_at()),
                    updated_at.eq(sku.updated_at()),
                    deleted_at.eq(sku.deleted_at()),
                    name.eq(sku.name()),
                    price.eq(sku.price()),
                    product_type.eq(sku.product_type()),
                ))
                .execute(&mut connection)
        } else {
            diesel::update(table.filter(id.eq(sku.id())))
                .set((
                    created_at.eq(sku.created_at()),
                    updated_at.eq(get_timestamp()),
                    deleted_at.eq(sku.deleted_at()),
                    name.eq(sku.name()),
                    price.eq(sku.price()),
                    product_type.eq(sku.product_type()),
                ))
                .execute(&mut connection)
        };

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Error saving SKU: {}", err).into()),
        }
    }

    fn find(&self, search_input: SkuSearchInput) -> Result<SearchResult<Sku>> {
        let mut connection = self.db_context.establish_connection();
        let mut query = table.into_boxed();
        let mut count_query = table.into_boxed();

        query = Self::apply_search_filters(query, &search_input);
        query = Self::apply_paging(query, &search_input);
        query = Self::apply_ordering(query, &search_input);

        count_query = Self::apply_search_filters(count_query, &search_input);

        let amount = count_query
            .count()
            .get_result::<i64>(&mut connection)
            .unwrap() as i32;
        let result = query.load::<Sku>(&mut connection);

        match result {
            Ok(s) => Ok(SearchResult {
                total: amount,
                items: s,
            }),
            Err(err) => Err(format!("Error searching users: {}", err).into()),
        }
    }
}
