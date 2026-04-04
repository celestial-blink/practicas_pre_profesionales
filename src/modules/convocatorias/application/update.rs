use crate::modules::convocatorias::domain::{
    convocatoria::Convocatoria, repository::ConvocatoriaRepository,
};

pub struct Update<T: ConvocatoriaRepository> {
    pub repository: T,
}

impl<T: ConvocatoriaRepository> Update<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, convocatoria: Convocatoria) -> Result<(), String> {
        self.repository.update(convocatoria).await
    }
}
