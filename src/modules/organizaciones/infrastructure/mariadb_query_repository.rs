use tracing::error;

use crate::modules::organizaciones::application::dto::search_result::SearchResult;
use crate::modules::organizaciones::application::repository::query_respository::QueryRepository;
use crate::modules::organizaciones::domain::dto::SearchParams;
use crate::modules::organizaciones::domain::organizacion::Organizacion;

pub struct MariadbQueryRepository;

impl QueryRepository for MariadbQueryRepository {
    async fn find_by_search(
        &self,
        pool: &sqlx::MySqlPool,
        params: SearchParams,
    ) -> Result<Vec<SearchResult>, String> {
        let mut query_sql = String::from("SELECT *, COUNT(*) OVER() as total FROM organizaciones");
        // agrega where en search, tipo y estado solo si vienen

        let mut where_clause = Vec::<String>::new();

        if params.search.is_some() {
            where_clause.push("CONCAT(razon_social, nombre_comercial, ruc) LIKE ?".to_owned());
        }

        if params.tipo.is_some() {
            where_clause.push("tipo = ?".to_owned());
        }

        if params.estado.is_some() {
            where_clause.push("estado = ?".to_owned());
        }

        if !where_clause.is_empty() {
            query_sql = format!("{} WHERE {}", query_sql, where_clause.join(" AND "));
        }

        // agrega limit y offset
        query_sql = format!("{} ORDER BY id DESC LIMIT ? OFFSET ?", query_sql);

        let mut query = sqlx::query_as::<_, SearchResult>(&query_sql);

        if let Some(search) = params.search {
            query = query.bind(format!("%{}%", search));
        }

        if let Some(tipo) = params.tipo {
            query = query.bind(tipo);
        }

        if let Some(estado) = params.estado {
            query = query.bind(estado);
        }

        query = query.bind(params.limit);
        query = query.bind(params.offset);

        let result = query.fetch_all(pool).await;

        if let Ok(organizaciones) = result {
            return Ok(organizaciones);
        }

        Err("Error al buscar organizaciones".to_string())
    }
    async fn find_all(&self, pool: &sqlx::MySqlPool) -> Result<Vec<Organizacion>, String> {
        let query_sql =
            "SELECT * FROM organizaciones WHERE estado = 1 ORDER BY nombre_comercial ASC";
        let result = sqlx::query_as::<_, Organizacion>(query_sql)
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn get_one_by_alias(
        &self,
        pool: &sqlx::MySqlPool,
        alias: String,
    ) -> Option<Organizacion> {
        let oferta =
            sqlx::query_as::<_, Organizacion>("SELECT * FROM organizaciones WHERE alias = ?")
                .bind(&alias)
                .fetch_optional(pool)
                .await;

        match oferta {
            Ok(Some(oferta)) => Some(oferta),
            _ => {
                error!("Oferta no encontrada con el alias: {}", alias);
                None
            }
        }
    }
}
