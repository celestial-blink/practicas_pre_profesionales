use sqlx::MySqlPool;

use crate::modules::convocatorias::application::{
    dtos::{search_params::SearchParams, search_result::SearchResult},
    repository::query_repository::QueryRepository,
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
