use crate::modules::oferta_niveles::domain::{oferta_niveles::OfertaNiveles, oferta_niveles_repository::OfertaNivelesRepository};

pub struct CreateMultiple<T: OfertaNivelesRepository> {
    repository: T,
}

impl<T: OfertaNivelesRepository> CreateMultiple<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, niveles: Vec<OfertaNiveles>, tx: &mut sqlx::Transaction<'_, sqlx::MySql>) -> Result<(), String> {
        self.repository.create_multiple(niveles, tx).await
    }
}
