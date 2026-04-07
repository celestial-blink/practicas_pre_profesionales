use super::oferta_niveles::OfertaNiveles;

pub trait OfertaNivelesRepository {
    async fn remove_by_nivel_academico(
        &self,
        id_oferta: i32,
        id_niveles_academicos: Vec<i8>,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
    ) -> Result<(), String>;
    async fn remove_by_id_oferta(
        &self,
        id_oferta: i32,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
    ) -> Result<(), String>;
    async fn create_multiple(
        &self,
        niveles: Vec<OfertaNiveles>,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
    ) -> Result<(), String>;
}
