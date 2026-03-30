use crate::modules::ofertas::domain::{oferta::Oferta, repository::OfertaRepository};

pub struct GetAllByEstado<T: OfertaRepository> {
    pub repository: T,
}

impl<T: OfertaRepository> GetAllByEstado<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, estado: i8, limit: i32, offset: i32) -> Result<Vec<Oferta>, String> {
        self.repository.get_all_by_estado(estado, limit, offset).await
    }
}
