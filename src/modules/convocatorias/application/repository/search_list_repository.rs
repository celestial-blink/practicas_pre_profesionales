use crate::modules::convocatorias::{application::dtos::search_list_result::SearchListResult, domain::dtos::search_params::SearchParams};

pub trait SearchListRepository {
    async fn find_by_search_for_list(&self, params: SearchParams) -> Result<Vec<SearchListResult>, String>;
}
