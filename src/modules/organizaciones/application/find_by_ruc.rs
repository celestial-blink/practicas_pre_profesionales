use crate::modules::organizaciones::domain::{
    organizacion::Organizacion, repository::OrganizacionRepository,
};

pub struct FindByRuc<T: OrganizacionRepository> {
    pub repository: T,
}

impl<T: OrganizacionRepository> FindByRuc<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, ruc: String) -> Option<Organizacion> {
        self.repository.find_by_ruc(ruc).await
    }
}
