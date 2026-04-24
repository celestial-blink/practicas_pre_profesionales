use sqlx::MySqlPool;

use crate::modules::convocatorias::{
    application::{
        dtos::get_all_actives_params_dto::GetAllActivesParamsDto,
        repository::query_repository::QueryRepository,
    },
    domain::convocatoria::Convocatoria,
};

pub struct GetAllActives<T: QueryRepository> {
    pub repository: T,
}

impl<T: QueryRepository> GetAllActives<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        pool: &MySqlPool,
        params: GetAllActivesParamsDto,
    ) -> Result<Vec<Convocatoria>, String> {
        self.repository.get_all_actives(pool, params).await
    }
}
