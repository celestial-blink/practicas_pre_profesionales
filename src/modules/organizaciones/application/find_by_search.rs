use crate::modules::organizaciones::domain::{
    dto::SearchParams,
    organizacion::Organizacion,
    repository::OrganizacionRepository,
};

pub struct FindBySearch<T: OrganizacionRepository> {
    pub repository: T,
}

impl<T: OrganizacionRepository> FindBySearch<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, params: SearchParams) -> Result<Vec<Organizacion>, String> {
        self.repository.find_by_search(params).await
    }
}
