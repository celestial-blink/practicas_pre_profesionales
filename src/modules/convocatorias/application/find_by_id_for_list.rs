use crate::modules::convocatorias::application::{
    dtos::search_list_result::SearchListResult,
    repository::find_by_id_list_repository::FindByIdListRepository,
};

pub struct FindByIdForList<T: FindByIdListRepository> {
    pub repository: T,
}

impl<T: FindByIdListRepository> FindByIdForList<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Result<SearchListResult, String> {
        self.repository.find_by_id_list(id).await
    }
}
