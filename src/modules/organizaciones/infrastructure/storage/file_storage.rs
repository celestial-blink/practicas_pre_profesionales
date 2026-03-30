use std::os::unix::fs::PermissionsExt;

use actix_multipart::form::tempfile::TempFile;

use crate::modules::organizaciones::application::ports::local_storage::LocalStorage;

pub struct FileStorage;

impl LocalStorage for FileStorage {
    async fn save(&self, file: TempFile, file_name: String) -> Result<(), String> {
        let base_path = std::env::var("UPLOAD_LOGO_DIR").expect("STORAGE_DIR must be set");
        let full_path = format!("{}/{}", base_path, file_name);
        let _ = file.file.persist(&full_path).map_err(|e| e.to_string())?;
        // poner permisos al grupo
        std::fs::set_permissions(&full_path, std::fs::Permissions::from_mode(0o660))
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
