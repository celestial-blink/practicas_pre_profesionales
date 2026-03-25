use crate::modules::organizaciones::domain::{dto::SearchParams, organizacion::Organizacion};

pub trait OrganizacionRepository {
    async fn create(&self, organizacion: Organizacion) -> Result<(), String>;
    async fn update(&self, organizacion: Organizacion) -> Result<(), String>;
    async fn find_by_id(&self, id: i32) -> Option<Organizacion>;
    async fn find_by_ruc(&self, ruc: String) -> Option<Organizacion>;
    async fn find_by_search(&self, params: SearchParams) -> Result<Vec<Organizacion>, String>;
    async fn get_all_by_estado(&self, estado: i8) -> Result<Vec<Organizacion>, String>;
}
