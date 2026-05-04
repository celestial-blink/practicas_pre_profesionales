use sqlx::MySqlPool;

use crate::modules::ofertas::application::{
    dtos::count_ofertas_by_departamento_result_dto::CountOfertasByDepartamentoResultDto,
    repository::query_repository::QueryRepository,
};

pub struct GetCountOfertasByDepartamento<T: QueryRepository> {
    pub repository: T,
}

impl<T: QueryRepository> GetCountOfertasByDepartamento<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        pool: &MySqlPool,
    ) -> Result<Vec<CountOfertasByDepartamentoResultDto>, String> {
        self.repository
            .get_count_ofertas_by_departamento(pool)
            .await
    }
}
