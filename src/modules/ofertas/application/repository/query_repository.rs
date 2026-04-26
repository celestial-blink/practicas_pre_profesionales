use sqlx::MySqlPool;

use crate::modules::ofertas::domain::oferta::Oferta;

pub trait QueryRepository {
    async fn get_one_by_alias(&self, pool: &MySqlPool, alias: String) -> Option<Oferta>;
}
