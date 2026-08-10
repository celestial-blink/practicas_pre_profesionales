use crate::modules::convocatorias::domain::convocatoria::Convocatoria;

#[allow(async_fn_in_trait)]
pub trait ConvocatoriaRepository {
    async fn create(&self, convocatoria: Convocatoria) -> Result<(), String>;
    async fn update(&self, convocatoria: Convocatoria) -> Result<(), String>;
    async fn find_by_id(&self, id: i32) -> Option<Convocatoria>;
}
