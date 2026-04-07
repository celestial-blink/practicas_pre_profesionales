use crate::modules::ofertas::domain::{
    dtos::oferta_with_niveles::OfertaWithNivelesDto, repository::OfertaRepository,
};

pub struct FindByIdWithNiveles<T: OfertaRepository> {
    pub repository: T,
}

impl<T: OfertaRepository> FindByIdWithNiveles<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Option<OfertaWithNivelesDto> {
        self.repository.find_by_id_with_niveles(id).await
    }
}
