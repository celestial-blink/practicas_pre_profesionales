use sqlx::MySqlPool;
use tracing_log::log::error;

use crate::modules::pre_ofertas::application::{
    dto::{search_params::SearchParams, search_result::PreOfertasSearchResult},
    repository::query_repository::QueryRepository,
};

pub struct PreOfertasQueryRepository;

impl QueryRepository for PreOfertasQueryRepository {
    async fn find_by_search(
        &self,
        pool: &MySqlPool,
        params: SearchParams,
    ) -> Result<Vec<PreOfertasSearchResult>, String> {
        let columns = [
            "id",
            "titulo",
            "id_organizacion",
            "nombre_organizacion",
            "modalidad_practicas",
            "id_region",
            "region",
            "distrito",
            "url_oferta",
            "hash_oferta",
            "estado",
            "creado_en",
            "COUNT(*) OVER() as total",
        ];

        let mut query = format!(
            "SELECT {} FROM pre_ofertas ORDER BY id DESC LIMIT ? OFFSET ?",
            columns.join(", ")
        );

        if params.search.is_some() {
            query = format!(
                "SELECT {} FROM pre_ofertas WHERE CONCAT(pre_ofertas.titulo, pre_ofertas.nombre_organizacion) LIKE ? ORDER BY id DESC LIMIT ? OFFSET ?",
                columns.join(", ")
            );
        }

        let mut result = sqlx::query_as::<_, PreOfertasSearchResult>(&query);

        if let Some(search) = params.search {
            result = result.bind(format!("%{}%", search));
        }

        let result = result
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(pool)
            .await;
        match result {
            Ok(pre_ofertas) => Ok(pre_ofertas),
            Err(e) => {
                error!("Error al buscar las pre_ofertas: {}", e);
                Err(e.to_string())
            }
        }
    }
}
