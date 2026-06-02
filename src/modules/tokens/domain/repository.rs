use crate::modules::tokens::domain::token::Token;

#[allow(async_fn_in_trait)]
pub trait TokensRepository {
    async fn find_by_token(&self, token: String) -> Option<Token>;
}
