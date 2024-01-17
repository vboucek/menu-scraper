use std::path::Path;
use actix_multipart::form::tempfile::TempFile;
use anyhow::{anyhow, Error};
use tokio::fs::create_dir;
use uuid::Uuid;


/// Validates uploaded picture and saves on the server. Returns name of the saved file.
pub async fn validate_and_save_picture(picture: TempFile) -> Result<String, Error> {
    const UPLOADS_DIR: &str = "./uploads";
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MB
    const ALLOWED_EXTENSIONS: [&str; 2] = ["png", "jpg"];

    // Ensure the "uploads" directory exists, create it if not
    if !Path::new(UPLOADS_DIR).exists() {
        create_dir(UPLOADS_DIR).await?;
    }

    let file_size = picture.size;

    if file_size == 0 {
        return Err(anyhow!("The uploaded file is empty."));
    }

    if file_size > MAX_FILE_SIZE {
        return Err(
            anyhow!("The uploaded file is too large. Maximum size is {} bytes.", MAX_FILE_SIZE)
        );
    }

    // Extract file extension
    let filename = picture.file_name.unwrap_or("unknown.ext".to_string());
    let extension = filename.split('.').last().unwrap_or_default();

    // Check if the file extension is allowed
    if !ALLOWED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
        return Err(anyhow!("File must be *.png or *.jpeg."));
    }

    // Generate a unique filename
    let unique_filename = Uuid::new_v4().to_string();
    let filepath = format!("{}/{}.{}", UPLOADS_DIR, unique_filename, extension);

    // Create a new file and write the image data to it
    picture.file.persist(filepath)?;

    Ok(format!("{}.{}", unique_filename, extension))
}