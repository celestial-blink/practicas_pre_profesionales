use sqlx::MySqlPool;

use crate::modules::convocatorias::{
    application::dtos::get_all_actives_params_dto::GetAllActivesParamsDto,
    domain::convocatoria::Convocatoria,
};

pub trait QueryRepository {
    async fn get_all_actives(
        &self,
        pool: &MySqlPool,
        params: GetAllActivesParamsDto,
    ) -> Result<Vec<Convocatoria>, String>;

    async fn get_one_by_alias(
        &self,
        pool: &MySqlPool,
        alias: String,
    ) -> Result<Convocatoria, String>;
}
