use sqlx::MySqlPool;

use crate::modules::organizaciones::{
    application::{
        dto::search_result::SearchResult, repository::query_respository::QueryRepository,
    },
    domain::dto::SearchParams,
};

pub struct FindBySearch<T: QueryRepository> {
    pub repository: T,
}

impl<T: QueryRepository> FindBySearch<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        pool: &MySqlPool,
        params: SearchParams,
    ) -> Result<Vec<SearchResult>, String> {
        self.repository.find_by_search(pool, params).await
    }
}
