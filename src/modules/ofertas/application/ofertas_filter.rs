use sqlx::MySqlPool;

use crate::modules::ofertas::application::{
    dtos::{
        ofertas_filter_params_dto::OfertasFilterParamsDto,
        ofertas_filter_result_dto::OfertasFilterResultDto,
    },
    repository::query_repository::QueryRepository,
};

pub struct OfertasFilter<'t, T: QueryRepository> {
    query_repository: &'t T,
}

impl<'t, T: QueryRepository> OfertasFilter<'t, T> {
    pub fn new(query_repository: &'t T) -> Self {
        Self { query_repository }
    }

    pub async fn execute(
        &self,
        pool: &MySqlPool,
        params: OfertasFilterParamsDto,
        limit: u32,
    ) -> Result<OfertasFilterResultDto, String> {
        self.query_repository
            .ofertas_filter(pool, params, limit)
            .await
    }
}
