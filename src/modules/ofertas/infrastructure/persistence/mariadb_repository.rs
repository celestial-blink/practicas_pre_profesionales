use tracing_log::log::error;

use crate::modules::ofertas::domain::oferta::Oferta;
use crate::modules::ofertas::domain::repository::OfertaRepository;

pub struct MariaDbRepository {
    pub pool: sqlx::MySqlPool,
}

impl MariaDbRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

impl OfertaRepository for MariaDbRepository {
    async fn create(
        &self,
        oferta: crate::modules::ofertas::domain::oferta::Oferta,
    ) -> Result<(), String> {
        let query = "INSERT INTO ofertas (titulo, alias, id_organizacion, nombre_org, modalidad_practicas, vacantes, subvencion, fecha_fin_oferta, formacion, funciones, lugar_practicas, como_postular, bases, extra_info, id_region, region, distrito, estado) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&oferta.titulo)
            .bind(&oferta.alias)
            .bind(&oferta.id_organizacion)
            .bind(&oferta.nombre_org)
            .bind(&oferta.modalidad_practicas)
            .bind(&oferta.vacantes)
            .bind(&oferta.subvencion)
            .bind(&oferta.fecha_fin_oferta)
            .bind(&oferta.formacion)
            .bind(&oferta.funciones)
            .bind(&oferta.lugar_practicas)
            .bind(&oferta.como_postular)
            .bind(&oferta.bases)
            .bind(&oferta.extra_info)
            .bind(&oferta.id_region)
            .bind(&oferta.region)
            .bind(&oferta.distrito)
            .bind(&oferta.estado)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn update(
        &self,
        oferta: crate::modules::ofertas::domain::oferta::Oferta,
    ) -> Result<(), String> {
        let query = "UPDATE ofertas SET titulo = ?, alias = ?, id_organizacion = ?, nombre_org = ?, modalidad_practicas = ?, vacantes = ?, subvencion = ?, fecha_fin_oferta = ?, formacion = ?, funciones = ?, lugar_practicas = ?, como_postular = ?, bases = ?, extra_info = ?, id_region = ?, region = ?, distrito = ?, estado = ? WHERE id = ?";
        let result = sqlx::query(query)
            .bind(&oferta.titulo)
            .bind(&oferta.alias)
            .bind(&oferta.id_organizacion)
            .bind(&oferta.nombre_org)
            .bind(&oferta.modalidad_practicas)
            .bind(&oferta.vacantes)
            .bind(&oferta.subvencion)
            .bind(&oferta.fecha_fin_oferta)
            .bind(&oferta.formacion)
            .bind(&oferta.funciones)
            .bind(&oferta.lugar_practicas)
            .bind(&oferta.como_postular)
            .bind(&oferta.bases)
            .bind(&oferta.extra_info)
            .bind(&oferta.id_region)
            .bind(&oferta.region)
            .bind(&oferta.distrito)
            .bind(&oferta.estado)
            .bind(&oferta.id)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_by_id(&self, id: i32) -> Option<Oferta> {
        let query = "SELECT * FROM ofertas WHERE id = ?";
        let result = sqlx::query_as::<_, Oferta>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(oferta)) => Some(oferta),
            Ok(None) => None,
            Err(e) => {
                error!("Error al buscar la oferta: {}", e);
                None
            }
        }
    }

    async fn find_by_search(
        &self,
        params: crate::modules::ofertas::domain::dtos::search_params::SearchParams,
    ) -> Result<Vec<Oferta>, String> {
        let query = "SELECT * FROM ofertas WHERE CONCAT(ofertas.titulo, ofertas.nombre_org) LIKE ? ORDER BY id DESC LIMIT ? OFFSET ?";
        let result = sqlx::query_as::<_, Oferta>(query)
            .bind(params.search)
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(&self.pool)
            .await;
        match result {
            Ok(ofertas) => Ok(ofertas),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn get_all_by_estado(
        &self,
        estado: i8,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Oferta>, String> {
        let query = "SELECT * FROM ofertas WHERE estado = ? ORDER BY id DESC LIMIT ? OFFSET ?";
        let result = sqlx::query_as::<_, Oferta>(query)
            .bind(estado)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await;
        match result {
            Ok(ofertas) => Ok(ofertas),
            Err(e) => Err(e.to_string()),
        }
    }
}
