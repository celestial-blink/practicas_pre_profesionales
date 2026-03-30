use crate::modules::ofertas::domain::{oferta::Oferta, repository::OfertaRepository};

pub struct Update<T: OfertaRepository> {
    pub repository: T,
}

impl<T: OfertaRepository> Update<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, oferta: Oferta) -> Result<(), String> {
        self.repository.update(oferta).await
    }
}
