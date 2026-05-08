use crate::modules::organizaciones::domain::{dto::SearchParams, organizacion::Organizacion};

pub trait QueryRepository {
    async fn find_by_search(&self, params: SearchParams) -> Result<Vec<Organizacion>, String>;
}
