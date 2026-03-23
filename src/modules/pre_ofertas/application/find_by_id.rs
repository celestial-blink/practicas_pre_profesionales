use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;
use crate::modules::pre_ofertas::domain::repository::PreOfertasRepository;

pub struct FindById<P: PreOfertasRepository> {
    repository: P,
}

impl<P: PreOfertasRepository> FindById<P> {
    pub fn new(repository: P) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Option<PreOfertas> {
        self.repository.find_by_id(id).await
    }
}