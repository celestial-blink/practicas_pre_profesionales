use sqlx::MySqlPool;

use crate::modules::ofertas::{
    application::dtos::get_all_actives_params_dto::GetAllActivesParamsDto, domain::oferta::Oferta,
};

pub trait QueryPort {
    async fn get_all_actives(&self, pool: &MySqlPool, params: GetAllActivesParamsDto) -> Result<Vec<Oferta>, String>;
}
