use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;
use crate::modules::pre_ofertas::domain::repository::PreOfertasRepository;

pub struct FindAll<P: PreOfertasRepository> {
    repository: P,
}

impl<P: PreOfertasRepository> FindAll<P> {
    pub fn new(repository: P) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Vec<PreOfertas> {
        self.repository.find_all().await
    }
}