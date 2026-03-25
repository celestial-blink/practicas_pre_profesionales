use crate::modules::organizaciones::domain::dto::SearchParams;
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
        todo!()
    }

    async fn update(&self, organizacion: Organizacion) -> Result<(), String> {
        todo!()
    }

    async fn find_by_id(&self, id: i32) -> Option<Organizacion> {
        todo!()
    }

    async fn find_by_ruc(&self, ruc: String) -> Option<Organizacion> {
        todo!()
    }

    async fn find_by_search(&self, params: SearchParams) -> Result<Vec<Organizacion>, String> {
        let mut query_sql = String::from("SELECT * FROM organizaciones");
        // agrega where en search, tipo y estado solo si vienen

        let mut where_clause = Vec::<String>::new();

        if params.search.is_some() {
            where_clause.push("CONCAT(razon_social, nombre_comercial, ruc) LIKE ?".to_owned());
        }

        if params.tipo.is_some() {
            where_clause.push("tipo = ?".to_owned());
        }

        if params.estado.is_some() {
            where_clause.push("estado = ?".to_owned());
        }

        if !where_clause.is_empty() {
            query_sql = format!("{} WHERE {}", query_sql, where_clause.join(" AND "));
        }

        // agrega limit y offset
        query_sql = format!("{} LIMIT ? OFFSET ?", query_sql);

        dbg!(&query_sql);

        let mut query = sqlx::query_as::<_, Organizacion>(&query_sql);

        if let Some(search) = params.search {
            query = query.bind(format!("%{}%", search));
        }

        if let Some(tipo) = params.tipo {
            query = query.bind(tipo);
        }

        if let Some(estado) = params.estado {
            query = query.bind(estado);
        }

        query = query.bind(params.limit);
        query = query.bind(params.offset);

        let result = query.fetch_all(&self.pool).await;

        if let Ok(organizaciones) = result {
            return Ok(organizaciones);
        }

        Err("Error al buscar organizaciones".to_string())
    }

    async fn get_all_by_estado(&self, estado: i8) -> Result<Vec<Organizacion>, String> {
        let query_sql = "SELECT * FROM organizaciones WHERE estado = ?";
        let result = sqlx::query_as::<_, Organizacion>(query_sql)
            .bind(estado)
            .fetch_all(&self.pool)
            .await;

        if let Ok(organizaciones) = result {
            return Ok(organizaciones);
        }

        Err("Error al buscar organizaciones".to_string())
    }
}
