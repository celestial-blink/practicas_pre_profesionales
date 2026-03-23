use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;
use crate::modules::pre_ofertas::domain::repository::PreOfertasRepository;

pub struct Create<P: PreOfertasRepository> {
    repository: P,
}

impl<P: PreOfertasRepository> Create<P> {
    pub fn new(repository: P) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, pre_ofertas: PreOfertas) -> Result<(), String> {
        self.repository.create(pre_ofertas).await
    }
}