use crate::modules::tokens::{application::get_by_token::FindByToken, domain::token::Token, infrastructure::mariadb_repository::MariadbRepository};

pub async fn auth_token(
    db: sqlx::MySqlPool,
    token: String,
) -> Option<Token> {
    let repository = MariadbRepository::new(db);
    let application = FindByToken::new(repository);
    application.execute(token).await
}
