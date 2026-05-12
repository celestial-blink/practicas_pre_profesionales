use crate::modules::organizaciones::{
    application::repository::query_respository::QueryRepository, domain::organizacion::Organizacion,
};

pub struct FindAll<T: QueryRepository> {
    pub repository: T,
}

impl<T: QueryRepository> FindAll<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Organizacion>, String> {
        self.repository.find_all().await
    }
}
