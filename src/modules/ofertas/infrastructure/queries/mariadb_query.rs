use sqlx::MySqlPool;
use tracing_log::log::error;

use crate::modules::ofertas::{
    application::{
        dtos::get_all_actives_params_dto::GetAllActivesParamsDto, ports::query_port::QueryPort,
    },
    domain::oferta::Oferta,
};

pub struct MariaDbQuery;

impl QueryPort for MariaDbQuery {
    async fn get_all_actives(
        &self,
        pool: &MySqlPool,
        params: GetAllActivesParamsDto,
    ) -> Result<Vec<Oferta>, String> {
        let query = "SELECT * FROM ofertas WHERE estado = 1 ORDER BY id DESC LIMIT ? OFFSET ?";
        let result = sqlx::query_as::<_, Oferta>(query)
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(pool)
            .await;
        match result {
            Ok(ofertas) => Ok(ofertas),
            Err(e) => {
                error!("Error al obtener las ofertas: {}", e);
                Err(e.to_string())
            },
        }
    }
}
