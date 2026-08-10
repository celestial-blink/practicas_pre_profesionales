use crate::modules::convocatorias::application::{
    dtos::{search_list_result::SearchListResult, search_params::SearchParams},
    repository::search_list_repository::SearchListRepository,
};

pub struct FindBySearchForList<T: SearchListRepository> {
    pub repository: T,
}

impl<T: SearchListRepository> FindBySearchForList<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, params: SearchParams) -> Result<Vec<SearchListResult>, String> {
        self.repository.find_by_search_for_list(params).await
    }
}
