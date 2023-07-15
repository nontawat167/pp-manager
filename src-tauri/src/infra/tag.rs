use std::sync::Arc;

use diesel::prelude::*;
use diesel::ExpressionMethods;

use crate::domain::tag::Tag;
use crate::domain::NEW_ENTITY_ID;
use crate::error::Result;
use crate::port::input::SearchOperator;
use crate::port::repostiory::TagRepository;
use crate::port::response::SearchResult;
use crate::port::tag::SqlOrder;
use crate::port::tag::TagOrderBy;
use crate::port::tag::TagSearchInput;
use crate::store::database::DatabaseContext;
use crate::store::schema::tags;
use crate::utils::generate_uuid;

pub struct TagRepositoryImpl {
    db_context: Arc<DatabaseContext>,
}

impl TagRepositoryImpl {
    pub fn new(db_context: Arc<DatabaseContext>) -> Self {
        Self { db_context }
    }
}

impl TagRepositoryImpl {
    fn apply_search_filters<'a>(
        query: tags::BoxedQuery<'a, diesel::sqlite::Sqlite>,
        search_input: &'a TagSearchInput,
    ) -> tags::BoxedQuery<'a, diesel::sqlite::Sqlite> {
        let mut filtered_query = query;

        if let Some(k) = &search_input.kind {
            match k {
                SearchOperator::Equal(search_kind) => {
                    filtered_query = filtered_query.filter(tags::kind.eq(search_kind));
                }
            }
        }

        filtered_query
    }

    fn apply_paging<'a>(
        query: tags::BoxedQuery<'a, diesel::sqlite::Sqlite>,
        search_input: &'a TagSearchInput,
    ) -> tags::BoxedQuery<'a, diesel::sqlite::Sqlite> {
        let mut new_query = query;
        let page: u32 = search_input.page.unwrap_or(1);
        let per_page = search_input.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;
        new_query = new_query.offset(offset as i64).limit(per_page as i64);
        new_query
    }

    fn apply_ordering<'a>(
        query: tags::BoxedQuery<'a, diesel::sqlite::Sqlite>,
        search_input: &'a TagSearchInput,
    ) -> tags::BoxedQuery<'a, diesel::sqlite::Sqlite> {
        let mut new_query = query;

        if let Some(order_by) = &search_input.order_by {
            match order_by {
                TagOrderBy::Name(SqlOrder::ASC) => new_query = new_query.order(tags::name.asc()),
                TagOrderBy::Name(SqlOrder::DESC) => new_query = new_query.order(tags::name.desc()),
            }
        }
        new_query
    }
}

impl TagRepository for TagRepositoryImpl {
    fn save(&self, tag: &mut Tag) -> Result<()> {
        let mut connection = self.db_context.establish_connection();

        let result = if String::from(NEW_ENTITY_ID).eq(tag.id()) {
            diesel::insert_into(tags::table)
                .values((
                    tags::id.eq(generate_uuid()),
                    tags::name.eq(tag.name()),
                    tags::kind.eq(tag.kind()),
                    tags::color.eq(tag.color()),
                ))
                .execute(&mut connection)
        } else {
            diesel::update(tags::table.filter(tags::id.eq(tag.id())))
                .set((
                    tags::name.eq(tag.name()),
                    tags::kind.eq(tag.kind()),
                    tags::color.eq(tag.color()),
                ))
                .execute(&mut connection)
        };

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Error saving Tag: {}", err).into()),
        }
    }

    fn find(&self, search_input: TagSearchInput) -> Result<SearchResult<Tag>> {
        let mut connection = self.db_context.establish_connection();
        let mut query = tags::table.into_boxed();
        let mut count_query = tags::table.into_boxed();

        query = Self::apply_search_filters(query, &search_input);
        query = Self::apply_paging(query, &search_input);
        query = Self::apply_ordering(query, &search_input);

        count_query = Self::apply_search_filters(count_query, &search_input);

        let amount = count_query
            .count()
            .get_result::<i64>(&mut connection)
            .unwrap() as i32;
        let result = query.load::<Tag>(&mut connection);

        match result {
            Ok(s) => Ok(SearchResult {
                total: amount,
                items: s,
            }),
            Err(err) => Err(format!("Error searching users: {}", err).into()),
        }
    }
}
