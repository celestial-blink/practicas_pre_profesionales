use sqlx::{MySql, Transaction};
use tracing_log::log::error;

use crate::modules::ofertas::domain::dtos::oferta_with_niveles::OfertaWithNivelesDto;
use crate::modules::ofertas::domain::oferta::Oferta;
use crate::modules::ofertas::domain::repository::OfertaRepository;
use crate::modules::ofertas::infrastructure::dtos::oferta_with_niveles_dto::OfertaWithNivelesDto as InfraOfertaWithNivelesDto;

pub struct MariaDbRepository {
    pub pool: sqlx::MySqlPool,
}

impl MariaDbRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

impl OfertaRepository for MariaDbRepository {
    async fn create_with_niveles(
        &self,
        oferta: crate::modules::ofertas::domain::oferta::Oferta,
        tx: &mut Transaction<'_, sqlx::MySql>,
    ) -> Result<i32, String> {
        let columns = [
            "id_convocatoria",
            "titulo",
            "alias",
            "id_organizacion",
            "nombre_org",
            "logo_org",
            "alias_org",
            "modalidad_practicas",
            "vacantes",
            "subvencion",
            "fecha_fin_oferta",
            "formacion",
            "carreras",
            "funciones",
            "lugar_practicas",
            "como_postular",
            "bases",
            "extra_info",
            "id_region",
            "region",
            "distrito",
            "niveles",
            "estado",
        ];

        let query = format!(
            "INSERT INTO ofertas ({}) VALUES ({})",
            columns.join(", "),
            columns
                .iter()
                .map(|_| "?")
                .collect::<Vec<&str>>()
                .join(", ")
        );
        let result = sqlx::query(&query)
            .bind(&oferta.id_convocatoria)
            .bind(&oferta.titulo)
            .bind(&oferta.alias)
            .bind(&oferta.id_organizacion)
            .bind(&oferta.nombre_org)
            .bind(&oferta.logo_org)
            .bind(&oferta.alias_org)
            .bind(&oferta.modalidad_practicas)
            .bind(&oferta.vacantes)
            .bind(&oferta.subvencion)
            .bind(&oferta.fecha_fin_oferta)
            .bind(&oferta.formacion)
            .bind(&oferta.carreras)
            .bind(&oferta.funciones)
            .bind(&oferta.lugar_practicas)
            .bind(&oferta.como_postular)
            .bind(&oferta.bases)
            .bind(&oferta.extra_info)
            .bind(&oferta.id_region)
            .bind(&oferta.region)
            .bind(&oferta.distrito)
            .bind(&oferta.niveles)
            .bind(&oferta.estado)
            .execute(&mut **tx)
            .await;
        match result {
            Ok(res) => Ok(res.last_insert_id() as i32),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn update(
        &self,
        oferta: crate::modules::ofertas::domain::oferta::Oferta,
        tx: &mut Transaction<'_, sqlx::MySql>,
    ) -> Result<(), String> {
        let columns = [
            "id_convocatoria",
            "titulo",
            "alias",
            "id_organizacion",
            "nombre_org",
            "logo_org",
            "alias_org",
            "modalidad_practicas",
            "vacantes",
            "subvencion",
            "fecha_fin_oferta",
            "formacion",
            "funciones",
            "lugar_practicas",
            "como_postular",
            "bases",
            "extra_info",
            "id_region",
            "region",
            "distrito",
            "niveles",
            "estado",
        ];
        let query = format!(
            "UPDATE ofertas SET {} WHERE id = ?",
            columns
                .iter()
                .map(|item| format!("{} = ?", item))
                .collect::<Vec<String>>()
                .join(", ")
        );
        let result = sqlx::query(&query)
            .bind(&oferta.id_convocatoria)
            .bind(&oferta.titulo)
            .bind(&oferta.alias)
            .bind(&oferta.id_organizacion)
            .bind(&oferta.nombre_org)
            .bind(&oferta.logo_org)
            .bind(&oferta.alias_org)
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
            .bind(&oferta.niveles)
            .bind(&oferta.estado)
            .bind(&oferta.id)
            .execute(&mut **tx)
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
        let mut query = "SELECT * FROM ofertas ORDER BY id DESC LIMIT ? OFFSET ?";
        if params.search.is_some() {
            query = "SELECT * FROM ofertas WHERE CONCAT(ofertas.titulo, ofertas.nombre_org) LIKE ? ORDER BY id DESC LIMIT ? OFFSET ?";
        }

        let mut result = sqlx::query_as::<_, Oferta>(query);

        if let Some(search) = params.search {
            result = result.bind(format!("%{}%", search));
        }

        let result = result
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(ofertas) => Ok(ofertas),
            Err(e) => {
                error!("Error al buscar la oferta: {}", e);
                Err(e.to_string())
            }
        }
    }

    async fn with_transaction<F, R>(&self, f: F) -> Result<R, String>
    where
        F: AsyncFnOnce(&mut Transaction<'_, sqlx::MySql>) -> Result<R, String>,
    {
        let mut tx = self.pool.begin().await.unwrap();

        let result = f(&mut tx).await;

        match result {
            Ok(r) => {
                tx.commit().await.unwrap();
                Ok(r)
            }
            Err(e) => {
                error!("Error al ejecutar la transacción: {}", e);
                tx.rollback().await.unwrap();
                Err(e)
            }
        }
    }

    async fn find_by_id_with_niveles(&self, id: i32) -> Option<OfertaWithNivelesDto> {
        let query = "SELECT ofertas.*, GROUP_CONCAT(oferta_niveles.id_nivel_academico) as niveles_data FROM ofertas LEFT JOIN oferta_niveles ON ofertas.id = oferta_niveles.id_oferta WHERE ofertas.id = ? GROUP BY (ofertas.id)";

        let result = sqlx::query_as::<_, InfraOfertaWithNivelesDto>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await;
        match result {
            Ok(Some(oferta)) => {
                let niveles_data: Vec<i8> = if oferta.niveles_data.is_some() {
                    oferta
                        .niveles_data
                        .unwrap()
                        .split(",")
                        .into_iter()
                        .map(|x| x.parse::<i8>().unwrap_or(0))
                        .collect()
                } else {
                    vec![]
                };
                Some(OfertaWithNivelesDto {
                    id: oferta.id,
                    id_convocatoria: oferta.id_convocatoria,
                    titulo: oferta.titulo,
                    alias: oferta.alias,
                    id_organizacion: oferta.id_organizacion,
                    nombre_org: oferta.nombre_org,
                    logo_org: oferta.logo_org,
                    alias_org: oferta.alias_org,
                    modalidad_practicas: oferta.modalidad_practicas,
                    vacantes: oferta.vacantes,
                    subvencion: oferta.subvencion,
                    fecha_fin_oferta: oferta.fecha_fin_oferta,
                    formacion: oferta.formacion,
                    carreras: oferta.carreras,
                    funciones: oferta.funciones,
                    lugar_practicas: oferta.lugar_practicas,
                    como_postular: oferta.como_postular,
                    bases: oferta.bases,
                    extra_info: oferta.extra_info,
                    id_region: oferta.id_region,
                    region: oferta.region,
                    distrito: oferta.distrito,
                    niveles: oferta.niveles,
                    niveles_data: niveles_data,
                    estado: oferta.estado,
                    creado_en: oferta.creado_en,
                })
            }
            Ok(None) => None,
            Err(e) => {
                error!("Error al buscar la oferta: {}", e);
                None
            }
        }
    }

    async fn get_all_by_id_convocatoria(&self, id_convocatoria: i32) -> Result<Vec<Oferta>, String> {
        let query = "SELECT * FROM ofertas WHERE id_convocatoria = ?";
        let result = sqlx::query_as::<MySql, Oferta>(query)
            .bind(id_convocatoria)
            .fetch_all(&self.pool)
            .await;
        match result {
            Ok(ofertas) => Ok(ofertas),
            Err(e) => Err(e.to_string()),
        }
    }
}
