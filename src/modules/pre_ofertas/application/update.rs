use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;
use crate::modules::pre_ofertas::domain::repository::PreOfertasRepository;

pub struct Update<P: PreOfertasRepository> {
    repository: P,
}

impl<P: PreOfertasRepository> Update<P> {
    pub fn new(repository: P) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, pre_ofertas: PreOfertas) -> Result<(), String> {
        self.repository.update(pre_ofertas).await
    }
}