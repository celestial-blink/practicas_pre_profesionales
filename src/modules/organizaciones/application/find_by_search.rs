use crate::modules::organizaciones::{
    application::repository::query_respository::QueryRepository,
    domain::{dto::SearchParams, organizacion::Organizacion},
};

pub struct FindBySearch<T: QueryRepository> {
    pub repository: T,
}

impl<T: QueryRepository> FindBySearch<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, params: SearchParams) -> Result<Vec<Organizacion>, String> {
        self.repository.find_by_search(params).await
    }
}
