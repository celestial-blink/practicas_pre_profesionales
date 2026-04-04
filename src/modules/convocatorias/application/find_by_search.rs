use crate::modules::convocatorias::domain::{
    convocatoria::Convocatoria, dtos::search_params::SearchParams, repository::ConvocatoriaRepository
};

pub struct FindBySearch<T: ConvocatoriaRepository> {
    pub repository: T,
}

impl<T: ConvocatoriaRepository> FindBySearch<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, params: SearchParams) -> Result<Vec<Convocatoria>, String> {
        self.repository.find_by_search(params).await
    }
}
