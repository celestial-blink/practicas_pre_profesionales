use sqlx::MySqlPool;

use crate::modules::ofertas::application::{
    dtos::count_ofertas_by_organizacion_result_dto::CountOfertasByOrganizacionResultDto,
    repository::query_repository::QueryRepository,
};

pub struct GetCountOfertasByOrganizacion<T: QueryRepository> {
    pub repository: T,
}

impl<T: QueryRepository> GetCountOfertasByOrganizacion<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        pool: &MySqlPool,
    ) -> Result<Vec<CountOfertasByOrganizacionResultDto>, String> {
        self.repository
            .get_count_ofertas_by_organizacion(pool)
            .await
    }
}
