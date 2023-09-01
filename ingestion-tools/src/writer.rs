use async_trait::async_trait;
use google_cloud_storage::{
    client::Client,
    http::objects::upload::{Media, UploadObjectRequest, UploadType},
};
use std::fs::File;
use std::io::Write;

// Define a trait for the storage provider
#[async_trait]
pub trait StorageProvider {
    async fn write(&self, path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>>;
}

// Implement GCS storage
pub struct GcsStorage{
    pub bucket: String,
    pub client: Client
}

#[async_trait]
impl StorageProvider for GcsStorage {
    async fn write(&self, path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implement GCS write logic here
        

        let upload_type = UploadType::Simple(Media::new(path.clone().to_string()));
        self.client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket.clone(),
                    ..Default::default()
                },
                data.clone().to_string(),
                &upload_type,
            )
            .await?;

        println!("Writing to GCS path: {}", path);
        Ok(())
    }
}

// Implement local file system storage
pub struct LocalStorage;

#[async_trait]
impl StorageProvider for LocalStorage {
    async fn write(&self, path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implement local file write logic here
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        println!("Writing to local path: {}", path);
        Ok(())
    }
}

// Main struct for writing data using StorageProvider
pub struct DataWriter<T: StorageProvider> {
    storage: T,
}

impl<T: StorageProvider> DataWriter<T> {
    pub fn new(storage: T) -> Self {
        DataWriter { storage }
    }

    pub async fn write_data(
        &self,
        path: &str,
        data: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.storage.write(path, data).await?;
        Ok(())
    }
}
