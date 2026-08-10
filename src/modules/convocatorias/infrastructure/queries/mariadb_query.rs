use sqlx::MySqlPool;
use tracing_log::log::error;

use crate::modules::convocatorias::{
    application::{
        dtos::{search_params::SearchParams, search_result::SearchResult},
        repository::query_repository::QueryRepository,
    },
    domain::convocatoria::Convocatoria,
};

pub struct MariaDbQuery;

impl MariaDbQuery {
    pub fn new() -> Self {
        Self {}
    }
}

impl QueryRepository for MariaDbQuery {
    async fn get_all_actives(
        &self,
        pool: &sqlx::MySqlPool,
        params: crate::modules::convocatorias::application::dtos::get_all_actives_params_dto::GetAllActivesParamsDto,
    ) -> Result<Vec<crate::modules::convocatorias::domain::convocatoria::Convocatoria>, String>
    {
        let data_columns = [
            "id",
            "titulo",
            "alias",
            "id_organizacion",
            "nombre_org",
            "logo_org",
            "alias_org",
            "fin_convocatoria",
            "vacantes",
            "carreras",
            "departamentos",
            "subvenciones",
            "modalidades",
            "nivel_estudios",
            "finalizan_todos",
            "estado",
            "actualizado_en",
            "creado_en",
            "'' AS texto",
        ];
        let columns = match params.include_texto {
            true => "*".to_string(),
            false => data_columns.join(", "),
        };
        let query = format!(
            "SELECT {} FROM convocatorias WHERE estado = 1 AND fin_convocatoria >= CURDATE() ORDER BY id DESC LIMIT {}, {}",
            columns, params.offset, params.limit
        );

        let result = sqlx::query_as::<_, Convocatoria>(&query)
            .fetch_all(pool)
            .await;

        match result {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("Error al obtener las convocatorias: {}", e);
                Err(e.to_string())
            }
        }
    }

    async fn get_one_by_alias(
        &self,
        pool: &sqlx::MySqlPool,
        alias: String,
    ) -> Result<Convocatoria, String> {
        let query = "SELECT * FROM convocatorias WHERE alias = ?";

        let result = sqlx::query_as::<_, Convocatoria>(&query)
            .bind(alias)
            .fetch_optional(pool)
            .await;

        match result {
            Ok(Some(data)) => Ok(data),
            Ok(None) => Err("Convocatoria no encontrada".to_string()),
            Err(e) => {
                error!("Error al obtener la convocatoria: {}", e);
                Err(e.to_string())
            }
        }
    }

    async fn find_by_search(
        &self,
        pool: &MySqlPool,
        params: SearchParams,
    ) -> Result<Vec<SearchResult>, String> {
        let columns = [
            "id",
            "titulo",
            "alias",
            "id_organizacion",
            "nombre_org",
            "logo_org",
            "alias_org",
            "fin_convocatoria",
            "vacantes",
            "carreras",
            "NULL as texto",
            "departamentos",
            "subvenciones",
            "modalidades",
            "nivel_estudios",
            "finalizan_todos",
            "estado",
            "actualizado_en",
            "creado_en",
            "COUNT(*) OVER() as total",
        ];

        let mut query = format!(
            "SELECT {} FROM convocatorias ORDER BY id DESC LIMIT ? OFFSET ?",
            columns.join(", ")
        );

        if params.search.is_some() {
            query = format!(
                "SELECT {} FROM convocatorias WHERE CONCAT(convocatorias.titulo, convocatorias.nombre_org) LIKE ? ORDER BY id DESC LIMIT ? OFFSET ?",
                columns.join(", ")
            );
        }

        let mut result = sqlx::query_as::<_, SearchResult>(&query);

        if let Some(search) = params.search {
            result = result.bind(format!("%{}%", search));
        }

        let result = result
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(pool)
            .await;
        match result {
            Ok(convocatorias) => Ok(convocatorias),
            Err(e) => {
                error!("Error al buscar las convocatorias: {}", e);
                Err(e.to_string())
            }
        }
    }
}
