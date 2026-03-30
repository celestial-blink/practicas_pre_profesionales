use crate::modules::organizaciones::domain::{
    organizacion::Organizacion, repository::OrganizacionRepository,
};

pub struct FindById<T: OrganizacionRepository> {
    pub repository: T,
}

impl<T: OrganizacionRepository> FindById<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Option<Organizacion> {
        self.repository.find_by_id(id).await
    }
}
