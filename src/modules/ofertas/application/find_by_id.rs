use crate::modules::ofertas::domain::{oferta::Oferta, repository::OfertaRepository};

pub struct FindById<T: OfertaRepository> {
    pub repository: T,
}

impl<T: OfertaRepository> FindById<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Option<Oferta> {
        self.repository.find_by_id(id).await
    }
}
