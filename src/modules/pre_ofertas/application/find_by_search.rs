use sqlx::MySqlPool;

use crate::modules::pre_ofertas::application::{
    dto::{search_params::SearchParams, search_result::PreOfertasSearchResult},
    repository::query_repository::QueryRepository,
};

pub struct FindBySearch<P: QueryRepository> {
    repository: P,
}

impl<P: QueryRepository> FindBySearch<P> {
    pub fn new(repository: P) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        pool: &MySqlPool,
        params: SearchParams,
    ) -> Result<Vec<PreOfertasSearchResult>, String> {
        self.repository.find_by_search(pool, params).await
    }
}
