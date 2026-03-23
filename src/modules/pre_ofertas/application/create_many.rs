use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;
use crate::modules::pre_ofertas::domain::repository::PreOfertasRepository;

pub struct CreateMany<P: PreOfertasRepository> {
    repository: P,
}

impl<P: PreOfertasRepository> CreateMany<P> {
    pub fn new(repository: P) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, pre_ofertas: Vec<PreOfertas>) -> Result<(), String> {
        self.repository.create_many(pre_ofertas).await
    }
}