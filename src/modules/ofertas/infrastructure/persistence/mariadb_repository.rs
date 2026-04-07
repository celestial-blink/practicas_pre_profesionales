use tracing_log::log::error;

use crate::modules::ofertas::domain::dtos::search_params_result_dto::SearchParamsResultDto;
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
    async fn create_with_niveles(
        &self,
        oferta: crate::modules::ofertas::domain::oferta::Oferta,
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
            "funciones",
            "lugar_practicas",
            "como_postular",
            "bases",
            "extra_info",
            "id_region",
            "region",
            "distrito",
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
            Ok(res) => Ok(res.last_insert_id() as i32),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn update(
        &self,
        oferta: crate::modules::ofertas::domain::oferta::Oferta,
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
    ) -> Result<Vec<SearchParamsResultDto>, String> {
        // TODO: usar sqlx::query_as::<_, SearchParamsResultDto>(query);
        let mut query = "SELECT ofertas.*, GROUP_CONCAT(oferta_niveles.id_nivel_academico) as niveles_data FROM ofertas INNER JOIN oferta_niveles ON ofertas.id = oferta_niveles.id_oferta GROUP BY (ofertas.id) ORDER BY id DESC LIMIT ? OFFSET ?";
        if params.search.is_some() {
            query = "SELECT ofertas.*, GROUP_CONCAT(oferta_niveles.id_nivel_academico) as niveles_data FROM ofertas INNER JOIN oferta_niveles ON ofertas.id = oferta_niveles.id_oferta WHERE CONCAT(ofertas.titulo, ofertas.nombre_org) LIKE ? GROUP BY (ofertas.id) ORDER BY id DESC LIMIT ? OFFSET ?";
        }

        let mut result = sqlx::query_as::<_, _>(query);
        if params.search.is_some() {
            result = result.bind(params.search);
        }
        let result = result
            .bind(params.limit)
            .bind(params.offset)
            .fetch_all(&self.pool)
            .await
        .map(|ofertas| {
            ofertas.iter().map(|oferta| {
                oferta.niveles_data = oferta.niveles.split(",").map(|n| n.parse::<i8>().unwrap()).collect();
                oferta
            }).collect()
        });

        match result {
            Ok(ofertas) => Ok(ofertas),
            Err(e) => {
                error!("Error al buscar la oferta: {}", e);
                Err(e.to_string())
            },
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

    async fn with_transaction<F, R>(&self, f: F) -> Result<R, String>
    where
        F: AsyncFnOnce() -> Result<R, String>,
    {
        let tx = self.pool.begin().await.unwrap();
        let result = f().await;

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
}
