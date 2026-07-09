use crate::modules::pre_ofertas::domain::{
    dto::search_params::SearchParams, pre_ofertas::PreOfertas, repository::PreOfertasRepository,
};

pub struct FindBySearch<P: PreOfertasRepository> {
    repository: P,
}

impl<P: PreOfertasRepository> FindBySearch<P> {
    pub fn new(repository: P) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, params: SearchParams) -> Result<Vec<PreOfertas>, String> {
        self.repository.find_by_search(params).await
    }
}
