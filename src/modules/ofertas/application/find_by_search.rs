use crate::modules::ofertas::domain::{
    dtos::search_params::SearchParams, oferta::Oferta, repository::OfertaRepository,
};

pub struct FindBySearch<T: OfertaRepository> {
    pub repository: T,
}

impl<T: OfertaRepository> FindBySearch<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, params: SearchParams) -> Result<Vec<Oferta>, String> {
        self.repository.find_by_search(params).await
    }
}
