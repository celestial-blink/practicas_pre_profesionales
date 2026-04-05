use super::oferta_niveles::OfertaNiveles;

pub trait OfertaNivelesRepository {
    async fn remove_by_nivel_academico(&self, id_oferta: i32, id_niveles_academicos: Vec<i8>) -> Result<(), String>;
    async fn create_multiple(&self, niveles: Vec<OfertaNiveles>) -> Result<(), String>;
}
