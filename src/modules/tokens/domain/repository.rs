use crate::modules::tokens::domain::token::Token;

pub trait TokensRepository {
    async fn find_by_token(&self, token: String) -> Option<Token>;
}
