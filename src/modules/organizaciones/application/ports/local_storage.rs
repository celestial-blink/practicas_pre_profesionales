use actix_multipart::form::tempfile::TempFile;

#[allow(async_fn_in_trait)]
pub trait LocalStorage {
    async fn save(&self, file: TempFile, file_name: String) -> Result<(), String>;
}
