use sqlx::Transaction;

use crate::modules::ofertas::domain::{
    dtos::oferta_with_niveles::OfertaWithNivelesDto, oferta::Oferta,
};

#[allow(async_fn_in_trait)]
pub trait OfertaRepository {
    async fn create_with_niveles(
        &self,
        oferta: Oferta,
        tx: &mut Transaction<'_, sqlx::MySql>,
    ) -> Result<i32, String>;
    async fn update(
        &self,
        oferta: Oferta,
        tx: &mut Transaction<'_, sqlx::MySql>,
    ) -> Result<(), String>;
    async fn find_by_id(&self, id: i32) -> Option<Oferta>;
    async fn find_by_id_with_niveles(&self, id: i32) -> Option<OfertaWithNivelesDto>;
    async fn with_transaction<F, R>(&self, f: F) -> Result<R, String>
    where
        F: AsyncFnOnce(&mut Transaction<'_, sqlx::MySql>) -> Result<R, String>;

    async fn get_all_by_id_convocatoria(&self, id_convocatoria: i32)
    -> Result<Vec<Oferta>, String>;
}
