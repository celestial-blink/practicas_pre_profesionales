use crate::modules::convocatorias::domain::{
    convocatoria::Convocatoria, repository::ConvocatoriaRepository,
};

pub struct FindById<T: ConvocatoriaRepository> {
    pub repository: T,
}

impl<T: ConvocatoriaRepository> FindById<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Option<Convocatoria> {
        self.repository.find_by_id(id).await
    }
}
