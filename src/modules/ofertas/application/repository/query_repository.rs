use sqlx::MySqlPool;

use crate::modules::ofertas::{
    application::dtos::{
        count_ofertas_by_departamento_result_dto::CountOfertasByDepartamentoResultDto,
        count_ofertas_by_organizacion_result_dto::CountOfertasByOrganizacionResultDto,
        ofertas_filter_params_dto::OfertasFilterParamsDto,
        ofertas_filter_result_dto::OfertasFilterResultDto, search_params::SearchParams,
        search_result::SearchResult,
    },
    domain::oferta::Oferta,
};

#[allow(async_fn_in_trait)]
pub trait QueryRepository {
    async fn get_one_by_alias(&self, pool: &MySqlPool, alias: String) -> Option<Oferta>;
    async fn get_count_ofertas_by_departamento(
        &self,
        pool: &MySqlPool,
    ) -> Result<Vec<CountOfertasByDepartamentoResultDto>, String>;
    async fn get_count_ofertas_by_organizacion(
        &self,
        pool: &MySqlPool,
    ) -> Result<Vec<CountOfertasByOrganizacionResultDto>, String>;
    async fn ofertas_filter(
        &self,
        pool: &MySqlPool,
        params: OfertasFilterParamsDto,
        limit: u32,
    ) -> Result<OfertasFilterResultDto, String>;
    async fn find_by_search(
        &self,
        pool: &MySqlPool,
        params: SearchParams,
    ) -> Result<Vec<SearchResult>, String>;
}
