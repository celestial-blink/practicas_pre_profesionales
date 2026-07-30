use sqlx::MySqlPool;

use crate::modules::organizaciones::domain::{dto::SearchParams, organizacion::Organizacion};

#[allow(async_fn_in_trait)]
pub trait QueryRepository {
    async fn find_by_search(
        &self,
        pool: &MySqlPool,
        params: SearchParams,
    ) -> Result<Vec<Organizacion>, String>;
    async fn find_all(&self, pool: &MySqlPool) -> Result<Vec<Organizacion>, String>;
    async fn get_one_by_alias(&self, pool: &MySqlPool, alias: String) -> Option<Organizacion>;
}
