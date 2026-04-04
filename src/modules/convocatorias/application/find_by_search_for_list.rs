use crate::modules::convocatorias::{
    application::{
        dtos::search_list_result::SearchListResult,
        repository::search_list_repository::SearchListRepository,
    },
    domain::dtos::search_params::SearchParams,
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
