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
        todo!()
    }

    async fn find_all(&self) -> Vec<PreOfertas> {
        todo!()
    }

    async fn create(&self, pre_ofertas: PreOfertas) -> Result<(), String> {
        todo!()
    }

    async fn create_many(&self, pre_ofertas: Vec<PreOfertas>) -> Result<(), String> {
        let sql_values = pre_ofertas.iter().map(|pre_oferta| {
            format!("('{}', {}, '{}', {}, {}, '{}', '{}', '{}', '{}', {})",
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
        }).collect::<Vec<String>>().join(", ");
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
        todo!()
    }
}
