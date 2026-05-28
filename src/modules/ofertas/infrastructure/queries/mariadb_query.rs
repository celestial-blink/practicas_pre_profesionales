use sqlx::MySqlPool;
use std::vec;
use tracing::error;

use crate::{
    general_types::Total,
    modules::ofertas::{
        application::{
            dtos::{
                count_ofertas_by_departamento_result_dto::CountOfertasByDepartamentoResultDto,
                count_ofertas_by_organizacion_result_dto::CountOfertasByOrganizacionResultDto,
                ofertas_filter_params_dto::OfertasFilterParamsDto,
                ofertas_filter_result_dto::OfertasFilterResultDto,
            },
            repository::query_repository::QueryRepository,
        },
        domain::oferta::Oferta,
    },
};

pub struct MariaDbQuery {}

impl MariaDbQuery {
    pub fn new() -> Self {
        Self {}
    }
}

impl QueryRepository for MariaDbQuery {
    async fn get_one_by_alias(&self, pool: &MySqlPool, alias: String) -> Option<Oferta> {
        let oferta = sqlx::query_as::<_, Oferta>("SELECT * FROM ofertas WHERE alias = ?")
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

    async fn get_count_ofertas_by_departamento(
        &self,
        pool: &MySqlPool,
    ) -> Result<Vec<CountOfertasByDepartamentoResultDto>, String> {
        sqlx::query_as::<_, CountOfertasByDepartamentoResultDto>("SELECT SUM(ofertas.vacantes) AS vacantes, ofertas.region AS departamento, ofertas.id_region AS id_departamento FROM ofertas WHERE ofertas.estado = 1 AND ofertas.fecha_fin_oferta >= CURRENT_TIMESTAMP GROUP BY region, id_region ORDER BY vacantes DESC")
            .fetch_all(pool)
            .await
        .map_err(|e| { format!("Error en obtener ofertas, {}", e.to_string()) })
    }

    async fn get_count_ofertas_by_organizacion(
        &self,
        pool: &MySqlPool,
    ) -> Result<Vec<CountOfertasByOrganizacionResultDto>, String> {
        sqlx::query_as::<_, CountOfertasByOrganizacionResultDto>("SELECT SUM(ofertas.vacantes) AS vacantes, ofertas.nombre_org AS organizacion, ofertas.id_organizacion, ofertas.alias_org AS alias, ofertas.logo_org AS logo FROM ofertas WHERE ofertas.estado = 1 AND ofertas.fecha_fin_oferta >= CURRENT_TIMESTAMP GROUP BY ofertas.nombre_org, ofertas.id_organizacion, ofertas.alias_org, ofertas.logo_org ORDER BY vacantes DESC")
            .fetch_all(pool)
            .await
        .map_err(|e| { format!("Error en obtener ofertas, {}", e.to_string()) })
    }

    async fn ofertas_filter(
        &self,
        pool: &MySqlPool,
        params: OfertasFilterParamsDto,
        limit: u32,
    ) -> Result<OfertasFilterResultDto, String> {
        let mut conn = pool
            .acquire()
            .await
            .map_err(|e| format!("Error en obtener ofertas, {}", e.to_string()))?;
        let table_name = if params.niveles.is_some() {
            "ofertas LEFT JOIN oferta_niveles ON ofertas.id = oferta_niveles.id_oferta"
        } else {
            "ofertas"
        };
        // busca en ofertas activas
        let mut active_conditional_str =
            String::from("ofertas.estado = 1 AND ofertas.fecha_fin_oferta >= CURRENT_TIMESTAMP");

        let id_organizacion = params.id_organizacion.unwrap_or(vec![]);
        if id_organizacion.len() > 0 {
            let prepare_id_organizacion = format!(
                " AND ofertas.id_organizacion IN ({})",
                vec!["?"; id_organizacion.len()].join(",")
            );
            active_conditional_str.push_str(&prepare_id_organizacion);
        }

        if let Some(modalidad) = params.modalidad_practicas {
            if modalidad == 2 {
                active_conditional_str.push_str(" AND ofertas.modalidad_practicas = ?");
            } else {
                active_conditional_str.push_str(" AND ofertas.modalidad_practicas != ?");
            }
        }

        if params.id_region.is_some() {
            active_conditional_str.push_str(" AND ofertas.id_region = ?");
        }

        let niveles = params.niveles.unwrap_or(vec![]);
        if !niveles.is_empty() {
            let prepare_niveles = format!(
                " AND oferta_niveles.id_nivel_academico IN ({})",
                vec!["?"; niveles.len()].join(",")
            );
            active_conditional_str.push_str(&prepare_niveles);
        }

        let search = params.search.unwrap_or("".to_string());
        if search.trim().len() > 0 {
            active_conditional_str.push_str(
                " AND MATCH(ofertas.titulo, ofertas.carreras) AGAINST (? IN BOOLEAN MODE)",
            );
        }

        let query_string = format!(
            "SELECT SQL_CALC_FOUND_ROWS * FROM {} WHERE {} LIMIT ? OFFSET ?",
            table_name, active_conditional_str
        );

        let mut actives_ofertas = sqlx::query_as::<_, Oferta>(&query_string);

        if id_organizacion.len() > 0 {
            for id_org in id_organizacion {
                actives_ofertas = actives_ofertas.bind(id_org);
            }
        }

        if let Some(modalidad) = params.modalidad_practicas {
            let prepare_modalidad = match modalidad {
                0 => 1,
                1 => 0,
                _ => 2,
            };
            actives_ofertas = actives_ofertas.bind(prepare_modalidad);
        }

        if params.id_region.is_some() {
            actives_ofertas = actives_ofertas.bind(params.id_region.unwrap());
        }

        if !niveles.is_empty() {
            for nivel in niveles {
                actives_ofertas = actives_ofertas.bind(nivel);
            }
        }

        if search.trim().len() > 0 {
            // solo agrega + a caracteres que sea igual o mas de 3
            let search_str = search
                .split_whitespace()
                .map(|word| {
                    let word = word.trim();
                    if word.len() > 7 {
                        format!(">{} +{}*", word, &word[..3])
                    } else if word.len() >= 3 {
                        format!("+{}", word)
                    } else {
                        word.to_string()
                    }
                })
                .filter(|word| word.len() > 0)
                .collect::<Vec<String>>()
                .join(" ");

            actives_ofertas = actives_ofertas.bind(search_str);
        }

        actives_ofertas = actives_ofertas.bind(limit).bind(params.offset);

        let actives_ofertas = actives_ofertas
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| format!("Error en obtener ofertas, {}", e.to_string()))?;

        // debug consulta y params
        // dbg!(&table_name);
        // dbg!(&active_conditional_str);
        // dbg!(&query_string, params.offset, limit);
        dbg!(&query_string);

        // obtiene el total de ofertas activas si hay resultados
        let mut total_actives_ofertas = actives_ofertas.len();

        if total_actives_ofertas > 0 {
            let query_string = "SELECT FOUND_ROWS() as total";

            let total = sqlx::query_as::<_, Total>(query_string)
                .fetch_one(&mut *conn)
                .await
                .map_err(|e| format!("Error en obtener ofertas, {}", e.to_string()))?;

            total_actives_ofertas = total.total as usize;
        }

        // optine 30 ofertas vencidas solo con el filtro de estao y fin_convocatoria, solo si total_actives_ofertas es igual a 0,
        let vencidas_ofertas = match total_actives_ofertas {
            0 => {
                let query_string = "SELECT * FROM ofertas WHERE ofertas.estado = 1 AND ofertas.fecha_fin_oferta < CURRENT_TIMESTAMP LIMIT 30";
                sqlx::query_as::<_, Oferta>(query_string)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| format!("Error en obtener ofertas, {}", e.to_string()))?
            }
            _ => vec![],
        };

        Ok(OfertasFilterResultDto {
            ofertas_activas: actives_ofertas,
            ofertas_vencidas: vencidas_ofertas,
            total_activas: total_actives_ofertas as i32,
        })
    }
}
