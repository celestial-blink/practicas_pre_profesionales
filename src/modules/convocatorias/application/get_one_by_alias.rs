use sqlx::MySqlPool;

use crate::modules::convocatorias::{
    application::repository::query_repository::QueryRepository, domain::convocatoria::Convocatoria,
};

pub struct GetOneByAlias<T: QueryRepository> {
    pub repository: T,
}

impl<T: QueryRepository> GetOneByAlias<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, pool: &MySqlPool, alias: String) -> Result<Convocatoria, String> {
        self.repository.get_one_by_alias(pool, alias).await
    }
}
