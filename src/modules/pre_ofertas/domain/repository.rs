use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;

pub trait PreOfertasRepository {
    async fn find_by_id(&self, id: i32) -> Option<PreOfertas>;
    async fn find_all(&self) -> Vec<PreOfertas>;
    async fn create(&self, pre_ofertas: PreOfertas) -> Result<(), String>;
    async fn create_many(&self, pre_ofertas: Vec<PreOfertas>) -> Result<(), String>;
    async fn update(&self, pre_ofertas: PreOfertas) -> Result<(), String>;
}