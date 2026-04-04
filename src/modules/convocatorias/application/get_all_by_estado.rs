use crate::modules::convocatorias::domain::{
    convocatoria::Convocatoria, repository::ConvocatoriaRepository,
};

pub struct GetAllByEstado<T: ConvocatoriaRepository> {
    pub repository: T,
}

impl<T: ConvocatoriaRepository> GetAllByEstado<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        estado: i8,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Convocatoria>, String> {
        self.repository
            .get_all_by_estado(estado, limit, offset)
            .await
    }
}
