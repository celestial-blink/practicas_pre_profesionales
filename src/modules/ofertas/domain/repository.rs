use crate::modules::ofertas::domain::{dtos::search_params::SearchParams, oferta::Oferta};

pub trait OfertaRepository {
    async fn create(&self, oferta: Oferta) -> Result<(), String>;
    async fn update(&self, oferta: Oferta) -> Result<(), String>;
    async fn find_by_id(&self, id: i32) -> Option<Oferta>;
    async fn find_by_search(&self, params: SearchParams) -> Result<Vec<Oferta>, String>;
    async fn get_all_by_estado(&self, estado: i8, limit: i32, offset: i32) -> Result<Vec<Oferta>, String>;
}
