use crate::modules::convocatorias::domain::{
    convocatoria::Convocatoria, dtos::search_params::SearchParams,
};

pub trait ConvocatoriaRepository {
    async fn create(&self, convocatoria: Convocatoria) -> Result<(), String>;
    async fn update(&self, convocatoria: Convocatoria) -> Result<(), String>;
    async fn find_by_id(&self, id: i32) -> Option<Convocatoria>;
    async fn find_by_search(&self, params: SearchParams) -> Result<Vec<Convocatoria>, String>;
    async fn get_all_by_estado(
        &self,
        estado: i8,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Convocatoria>, String>;
}
