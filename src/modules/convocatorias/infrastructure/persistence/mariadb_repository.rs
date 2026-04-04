use crate::modules::convocatorias::{application::{dtos::search_list_result::SearchListResult, repository::{find_by_id_list_repository::FindByIdListRepository, search_list_repository::SearchListRepository}}, domain::{
    convocatoria::Convocatoria, dtos::search_params::SearchParams,
    repository::ConvocatoriaRepository,
}};
use tracing_log::log::error;

pub struct MariaDbRepository {
    pub pool: sqlx::MySqlPool,
}

impl MariaDbRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

impl ConvocatoriaRepository for MariaDbRepository {
    async fn create(&self, convocatoria: Convocatoria) -> Result<(), String> {
        let query = "INSERT INTO convocatorias (titulo, alias, id_organizacion, nombre_org, logo_org, alias_org, fin_convocatoria, carreras, departamentos, subvenciones,modalidades, nivel_estudios, texto, finalizan_todos, estado) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&convocatoria.titulo)
            .bind(&convocatoria.alias)
            .bind(&convocatoria.id_organizacion)
            .bind(&convocatoria.nombre_org)
            .bind(&convocatoria.logo_org)
            .bind(&convocatoria.alias_org)
            .bind(&convocatoria.fin_convocatoria)
            .bind(&convocatoria.carreras)
            .bind(&convocatoria.departamentos)
            .bind(&convocatoria.subvenciones)
            .bind(&convocatoria.modalidades)
            .bind(&convocatoria.nivel_estudios)
            .bind(&convocatoria.texto)
            .bind(&convocatoria.finalizan_todos)
            .bind(&convocatoria.estado)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn update(&self, convocatoria: Convocatoria) -> Result<(), String> {
        let query = "UPDATE convocatorias SET titulo = ?, alias = ?, id_organizacion = ?, nombre_org = ?, logo_org = ?, alias_org = ?, fin_convocatoria = ?, carreras = ?, departamentos = ?, subvenciones = ?,modalidades = ?, nivel_estudios = ?, texto = ?, finalizan_todos = ?, estado = ? WHERE id = ?";
        let result = sqlx::query(query)
            .bind(&convocatoria.titulo)
            .bind(&convocatoria.alias)
            .bind(&convocatoria.id_organizacion)
            .bind(&convocatoria.nombre_org)
            .bind(&convocatoria.logo_org)
            .bind(&convocatoria.alias_org)
            .bind(&convocatoria.fin_convocatoria)
            .bind(&convocatoria.carreras)
            .bind(&convocatoria.departamentos)
            .bind(&convocatoria.subvenciones)
            .bind(&convocatoria.modalidades)
            .bind(&convocatoria.nivel_estudios)
            .bind(&convocatoria.texto)
            .bind(&convocatoria.finalizan_todos)
            .bind(&convocatoria.estado)
            .bind(&convocatoria.id)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_by_id(&self, id: i32) -> Option<Convocatoria> {
        let query = "SELECT * FROM convocatorias WHERE id = ?";
        let result = sqlx::query_as::<_, Convocatoria>(query)
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

    async fn find_by_search(&self, params: SearchParams) -> Result<Vec<Convocatoria>, String> {
        let query = "SELECT * FROM convocatorias WHERE CONCAT(convocatorias.titulo, convocatorias.nombre_org) LIKE ? ORDER BY id DESC LIMIT ? OFFSET ?";
        let result = sqlx::query_as::<_, Convocatoria>(query)
            .bind(params.search)
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(&self.pool)
            .await;
        match result {
            Ok(convocatorias) => Ok(convocatorias),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn get_all_by_estado(
        &self,
        estado: i8,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Convocatoria>, String> {
        let query =
            "SELECT * FROM convocatorias WHERE estado = ? ORDER BY id DESC LIMIT ? OFFSET ?";
        let result = sqlx::query_as::<_, Convocatoria>(query)
            .bind(estado)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await;
        match result {
            Ok(convocatorias) => Ok(convocatorias),
            Err(e) => Err(e.to_string()),
        }
    }
}


impl SearchListRepository for MariaDbRepository {
    async fn find_by_search_for_list(&self, params: SearchParams) -> Result<Vec<SearchListResult>, String> {
        let query = "SELECT (id, titulo, nombre_org, estado, creado_en) FROM convocatorias WHERE CONCAT(convocatorias.titulo, convocatorias.nombre_org) LIKE ? ORDER BY id DESC LIMIT ? OFFSET ?";
        let result = sqlx::query_as::<_, SearchListResult>(query)
            .bind(params.search)
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(&self.pool)
            .await;
        match result {
            Ok(convocatorias) => Ok(convocatorias),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl FindByIdListRepository for MariaDbRepository {
    async fn find_by_id_list(&self, id: i32) -> Result<SearchListResult, String> {
        let query = "SELECT (id, titulo, nombre_org, estado, creado_en) FROM convocatorias WHERE id = ?";
        let result = sqlx::query_as::<_, SearchListResult>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await;
        match result {
            Ok(Some(convocatoria)) => Ok(convocatoria),
            Ok(None) => Err("Convocatoria no encontrada".to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}
