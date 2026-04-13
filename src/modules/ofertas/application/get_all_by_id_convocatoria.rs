use crate::modules::ofertas::domain::{oferta::Oferta, repository::OfertaRepository};

pub struct GetAllByIdConvocatoria<T: OfertaRepository> {
    pub repository: T,
}

impl<T: OfertaRepository> GetAllByIdConvocatoria<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id_convocatoria: i32) -> Result<Vec<Oferta>, String> {
        self.repository
            .get_all_by_id_convocatoria(id_convocatoria)
            .await
    }
}
