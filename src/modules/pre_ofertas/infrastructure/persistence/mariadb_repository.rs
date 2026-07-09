use tracing_log::log::error;

use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;
use crate::modules::pre_ofertas::domain::repository::PreOfertasRepository;

const CREATE_MANY_QUERY: &str = "INSERT INTO pre_ofertas (titulo, id_organizacion, nombre_organizacion, modalidad_practicas, id_region, region, distrito, url_oferta, hash_oferta, estado) VALUES";

pub struct MariadbRepository {
    pool: sqlx::MySqlPool,
}

impl MariadbRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

impl PreOfertasRepository for MariadbRepository {
    async fn find_by_id(&self, id: i32) -> Option<PreOfertas> {
        let query = "SELECT * FROM pre_ofertas WHERE id = ?";
        let result = sqlx::query_as::<_, PreOfertas>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(convocatoria)) => Some(convocatoria),
            Ok(None) => None,
            Err(e) => {
                error!("Error al buscar la convocatoria: {}", e);
                None
            }
        }
    }

    async fn create(&self, pre_ofertas: PreOfertas) -> Result<(), String> {
        let columns = [
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
        ];
        let query = format!(
            "INSERT INTO pre_ofertas ({}) VALUES ({})",
            columns.join(", "),
            columns
                .iter()
                .map(|_| "?")
                .collect::<Vec<&str>>()
                .join(", ")
        );
        let result = sqlx::query(&query)
            .bind(&pre_ofertas.titulo)
            .bind(&pre_ofertas.id_organizacion)
            .bind(&pre_ofertas.nombre_organizacion)
            .bind(&pre_ofertas.modalidad_practicas)
            .bind(&pre_ofertas.id_region)
            .bind(&pre_ofertas.region)
            .bind(&pre_ofertas.distrito)
            .bind(&pre_ofertas.url_oferta)
            .bind(&pre_ofertas.hash_oferta)
            .bind(&pre_ofertas.estado)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn create_many(&self, pre_ofertas: Vec<PreOfertas>) -> Result<(), String> {
        let sql_values = pre_ofertas
            .iter()
            .map(|pre_oferta| {
                format!(
                    "('{}', {}, '{}', {}, {}, '{}', '{}', '{}', '{}', {})",
                    pre_oferta.titulo,
                    pre_oferta.id_organizacion,
                    pre_oferta.nombre_organizacion,
                    pre_oferta.modalidad_practicas,
                    pre_oferta.id_region,
                    pre_oferta.region,
                    pre_oferta.distrito,
                    pre_oferta.url_oferta,
                    pre_oferta.hash_oferta,
                    pre_oferta.estado,
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let sql = format!("{} {}", CREATE_MANY_QUERY, sql_values);
        let mut query = sqlx::query(&sql);
        for pre_oferta in pre_ofertas {
            query = query.bind(pre_oferta.titulo);
            query = query.bind(pre_oferta.id_organizacion);
            query = query.bind(pre_oferta.nombre_organizacion);
            query = query.bind(pre_oferta.modalidad_practicas);
            query = query.bind(pre_oferta.id_region);
            query = query.bind(pre_oferta.region);
            query = query.bind(pre_oferta.distrito);
            query = query.bind(pre_oferta.url_oferta);
            query = query.bind(pre_oferta.hash_oferta);
            query = query.bind(pre_oferta.estado);
        }
        query.execute(&self.pool).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update(&self, pre_ofertas: PreOfertas) -> Result<(), String> {
        let columns = [
            "titulo = ?",
            "id_organizacion = ?",
            "nombre_organizacion = ?",
            "modalidad_practicas = ?",
            "id_region = ?",
            "region = ?",
            "distrito = ?",
            "url_oferta = ?",
            "hash_oferta = ?",
            "estado = ?",
        ];
        let query = format!("UPDATE pre_ofertas SET {} WHERE id = ?", columns.join(", "));
        let result = sqlx::query(&query)
            .bind(&pre_ofertas.titulo)
            .bind(&pre_ofertas.id_organizacion)
            .bind(&pre_ofertas.nombre_organizacion)
            .bind(&pre_ofertas.modalidad_practicas)
            .bind(&pre_ofertas.id_region)
            .bind(&pre_ofertas.region)
            .bind(&pre_ofertas.distrito)
            .bind(&pre_ofertas.url_oferta)
            .bind(&pre_ofertas.hash_oferta)
            .bind(&pre_ofertas.estado)
            .bind(&pre_ofertas.id)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_by_search(
        &self,
        params: crate::modules::pre_ofertas::domain::dto::search_params::SearchParams,
    ) -> Result<Vec<PreOfertas>, String> {
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

        let mut result = sqlx::query_as::<_, PreOfertas>(&query);

        if let Some(search) = params.search {
            result = result.bind(format!("%{}%", search));
        }

        let result = result
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(&self.pool)
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
