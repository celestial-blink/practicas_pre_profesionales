use crate::modules::tokens::domain::{repository::TokensRepository, token::Token};


pub struct FindByToken<P: TokensRepository> {
    repository: P,
}

impl<P: TokensRepository> FindByToken<P> {
    pub fn new(repository: P) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, token: String) -> Option<Token> {
        self.repository.find_by_token(token).await
    }
}
