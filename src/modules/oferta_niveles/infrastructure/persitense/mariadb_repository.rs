use crate::modules::oferta_niveles::domain::{
    oferta_niveles::OfertaNiveles, oferta_niveles_repository::OfertaNivelesRepository,
};

pub struct MariaDbRepository {
    pool: sqlx::MySqlPool,
}

impl MariaDbRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

impl OfertaNivelesRepository for MariaDbRepository {
    async fn create_multiple(&self, niveles: Vec<OfertaNiveles>) -> Result<(), String> {
        let sql_insert = format!(
            "INSERT INTO oferta_niveles (id_oferta, id_nivel_estudio) VALUES {}",
            niveles
                .iter()
                .map(|_| "(?, ?)".to_owned())
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut query = sqlx::query(&sql_insert);
        for nivel in niveles {
            query = query.bind(nivel.id_oferta).bind(nivel.id_nivel_academico);
        }

        query.execute(&self.pool).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn remove_by_nivel_academico(
        &self,
        id_oferta: i32,
        id_niveles_academicos: Vec<i8>,
    ) -> Result<(), String> {
        let sql_delete_by_nivel_academico = format!(
            "DELETE FROM oferta_niveles WHERE id_oferta = ? AND id_nivel_academico NOT IN ({})",
            id_niveles_academicos
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        sqlx::query(&sql_delete_by_nivel_academico)
            .bind(id_oferta)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
