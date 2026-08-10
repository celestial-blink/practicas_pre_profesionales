use crate::modules::convocatorias::application::dtos::{
    search_list_result::SearchListResult, search_params::SearchParams,
};

#[allow(async_fn_in_trait)]
pub trait SearchListRepository {
    async fn find_by_search_for_list(
        &self,
        params: SearchParams,
    ) -> Result<Vec<SearchListResult>, String>;
}
