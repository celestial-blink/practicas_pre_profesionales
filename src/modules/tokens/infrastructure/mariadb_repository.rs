use crate::modules::tokens::domain::{repository::TokensRepository, token::Token};

const FIND_BY_TOKEN_QUERY: &str = "SELECT * FROM tokens WHERE token = ?";

pub struct MariadbRepository {
    pool: sqlx::MySqlPool,
}

impl MariadbRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

impl TokensRepository for MariadbRepository {
    async fn find_by_token(&self, token: String) -> Option<Token> {
        let mut query = sqlx::query_as::<_, Token>(&FIND_BY_TOKEN_QUERY);
        query = query.bind(token);
        let result = query.fetch_optional(&self.pool).await;

        if let Ok(token) = result {
            return token;
        }

        None
    }
}
