use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;

#[allow(async_fn_in_trait)]
pub trait PreOfertasRepository {
    async fn find_by_id(&self, id: i32) -> Option<PreOfertas>;
    async fn create(&self, pre_ofertas: PreOfertas) -> Result<(), String>;
    async fn create_many(&self, pre_ofertas: Vec<PreOfertas>) -> Result<(), String>;
    async fn update(&self, pre_ofertas: PreOfertas) -> Result<(), String>;
}
