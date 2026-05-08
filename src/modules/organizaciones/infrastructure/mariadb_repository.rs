use tracing::error;

use crate::modules::organizaciones::domain::organizacion::Organizacion;
use crate::modules::organizaciones::domain::repository::OrganizacionRepository;

pub struct MariadbRepository {
    pool: sqlx::MySqlPool,
}

impl MariadbRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

impl OrganizacionRepository for MariadbRepository {
    async fn create(&self, organizacion: Organizacion) -> Result<(), String> {
        let query_sql = "INSERT INTO organizaciones (razon_social, nombre_comercial, alias, ruc, logo, tipo, estado) VALUES (?, ?, ?, ?, ?, ?, ?)";
        let _ = sqlx::query(query_sql)
            .bind(organizacion.razon_social)
            .bind(organizacion.nombre_comercial)
            .bind(organizacion.alias)
            .bind(organizacion.ruc)
            .bind(organizacion.logo)
            .bind(organizacion.tipo)
            .bind(organizacion.estado)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update(&self, organizacion: Organizacion) -> Result<(), String> {
        let query_sql = "UPDATE organizaciones SET razon_social = ?, nombre_comercial = ?, alias = ?, ruc = ?, tipo = ?, estado = ? WHERE id = ?";
        let _ = sqlx::query(query_sql)
            .bind(organizacion.razon_social)
            .bind(organizacion.nombre_comercial)
            .bind(organizacion.alias)
            .bind(organizacion.ruc)
            .bind(organizacion.tipo)
            .bind(organizacion.estado)
            .bind(organizacion.id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())

    }

    async fn find_by_id(&self, id: i32) -> Option<Organizacion> {
        let query_sql = "SELECT * FROM organizaciones WHERE id = ?";
        let result = sqlx::query_as::<_, Organizacion>(query_sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await;

        if let Ok(organizacion) = result {
            return organizacion;
        } else {
            error!("Error al buscar organizacion por id: {}", id);
        }

        None
    }

    async fn find_by_ruc(&self, ruc: String) -> Option<Organizacion> {
        let query_sql = "SELECT * FROM organizaciones WHERE ruc = ?";
        let result = sqlx::query_as::<_, Organizacion>(query_sql)
            .bind(&ruc)
            .fetch_optional(&self.pool)
            .await;

        if let Ok(organizacion) = result {
            return organizacion;
        } else {
            error!("Error al buscar organizacion por ruc: {}", ruc);
        }

        None
    }
}
