use sqlx::MySqlPool;
use tracing::error;

use crate::modules::ofertas::{
    application::repository::query_repository::QueryRepository, domain::oferta::Oferta,
};

pub struct MariaDbQuery;

impl QueryRepository for MariaDbQuery {
    async fn get_one_by_alias(&self, pool: &MySqlPool, alias: String) -> Option<Oferta> {
        let oferta = sqlx::query_as::<_, Oferta>("SELECT * FROM ofertas WHERE alias = ?")
            .bind(&alias)
            .fetch_optional(pool)
            .await;

        match oferta {
            Ok(Some(oferta)) => Some(oferta),
            _ => {
                error!("Oferta no encontrada con el alias: {}", alias);
                None
            }
        }
    }
}
