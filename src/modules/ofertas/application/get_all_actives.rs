use sqlx::MySqlPool;

use crate::modules::ofertas::{
    application::{
        dtos::get_all_actives_params_dto::GetAllActivesParamsDto, ports::query_port::QueryPort,
    },
    domain::oferta::Oferta,
};

pub struct GetAllActives<T: QueryPort> {
    pub query_port: T,
}

impl<T: QueryPort> GetAllActives<T> {
    pub fn new(query_port: T) -> Self {
        Self { query_port }
    }

    pub async fn execute(
        &self,
        pool: &MySqlPool,
        params: GetAllActivesParamsDto,
    ) -> Result<Vec<Oferta>, String> {
        self.query_port.get_all_actives(pool, params).await
    }
}
