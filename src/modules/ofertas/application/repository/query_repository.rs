use sqlx::MySqlPool;

use crate::modules::ofertas::{
    application::dtos::{
        count_ofertas_by_departamento_result_dto::CountOfertasByDepartamentoResultDto,
        count_ofertas_by_organizacion_result_dto::CountOfertasByOrganizacionResultDto,
    },
    domain::oferta::Oferta,
};

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
}
