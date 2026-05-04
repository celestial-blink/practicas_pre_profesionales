use sqlx::MySqlPool;
use tracing::error;

use crate::modules::ofertas::{
    application::{
        dtos::{
            count_ofertas_by_departamento_result_dto::CountOfertasByDepartamentoResultDto,
            count_ofertas_by_organizacion_result_dto::CountOfertasByOrganizacionResultDto,
        },
        repository::query_repository::QueryRepository,
    },
    domain::oferta::Oferta,
};

pub struct MariaDbQuery;

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
}
