use sqlx::MySqlPool;

use crate::modules::ofertas::{
    application::repository::query_repository::QueryRepository, domain::oferta::Oferta,
};

pub struct GetOneByAlias<'t, T: QueryRepository> {
    query_repository: &'t T,
}

impl<'t, T: QueryRepository> GetOneByAlias<'t, T> {
    pub fn new(query_repository: &'t T) -> Self {
        Self { query_repository }
    }

    pub async fn execute(&self, pool: &MySqlPool, alias: String) -> Option<Oferta> {
        self.query_repository.get_one_by_alias(pool, alias).await
    }
}
