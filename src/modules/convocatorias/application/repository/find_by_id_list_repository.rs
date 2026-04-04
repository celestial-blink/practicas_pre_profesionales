use crate::modules::convocatorias::{
    application::dtos::search_list_result::SearchListResult
};

pub trait FindByIdListRepository {
    async fn find_by_id_list(&self, id: i32) -> Result<SearchListResult, String>;
}
