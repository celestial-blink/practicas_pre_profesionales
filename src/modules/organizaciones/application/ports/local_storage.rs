use actix_multipart::form::tempfile::TempFile;

pub trait LocalStorage {
    async fn save(&self, file: TempFile, file_name: String) -> Result<(), String>;
}
