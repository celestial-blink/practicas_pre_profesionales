use crate::modules::organizaciones::domain::organizacion::Organizacion;

#[allow(async_fn_in_trait)]
pub trait OrganizacionRepository {
    async fn create(&self, organizacion: Organizacion) -> Result<(), String>;
    async fn update(&self, organizacion: Organizacion) -> Result<(), String>;
    async fn find_by_id(&self, id: i32) -> Option<Organizacion>;
    async fn find_by_ruc(&self, ruc: String) -> Option<Organizacion>;
}
