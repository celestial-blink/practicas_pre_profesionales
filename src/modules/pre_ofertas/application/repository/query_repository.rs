use sqlx::MySqlPool;

use crate::modules::pre_ofertas::application::dto::{
    search_params::SearchParams, search_result::PreOfertasSearchResult,
};

#[allow(async_fn_in_trait)]
pub trait QueryRepository {
    async fn find_by_search(
        &self,
        pool: &MySqlPool,
        params: SearchParams,
    ) -> Result<Vec<PreOfertasSearchResult>, String>;
}
