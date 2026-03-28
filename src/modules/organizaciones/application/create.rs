use actix_multipart::form::tempfile::TempFile;

use crate::modules::organizaciones::{application::ports::local_storage::LocalStorage, domain::{
    organizacion::Organizacion,
    repository::OrganizacionRepository,
}};

pub struct Create<T: OrganizacionRepository, L: LocalStorage> {
    pub repository: T,
    pub local_storage: L,
}

impl<T: OrganizacionRepository, L: LocalStorage> Create<T, L> {
    pub fn new(repository: T, local_storage: L) -> Self {
        Self { repository, local_storage }
    }

    pub async fn execute(&self, organizacion: Organizacion, logo: TempFile) -> Result<(), String> {
        let _ = self.local_storage.save(logo, organizacion.logo.clone()).await?;
        self.repository.create(organizacion).await
    }
}
