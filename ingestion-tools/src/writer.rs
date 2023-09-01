use async_trait::async_trait;
use google_cloud_storage::{
    client::Client,
    http::objects::upload::{Media, UploadObjectRequest, UploadType},
};
use std::fs::File;
use std::io::Write;
use tokio::sync::Semaphore;

use crate::custom_error;

const MAX_CONCURRENT_WRITES: usize = 8; // Change this as needed

// Define a trait for the storage provider
#[async_trait]
pub trait StorageProvider {
    async fn write(
        &self,
        path: &str,
        data: &str,
        semaphore: &Semaphore,
    ) -> Result<(), custom_error::CustomError>;
}

// Implement GCS storage
pub struct GcsStorage {
    pub bucket: String,
    pub client: Client,
}

#[async_trait]
impl StorageProvider for GcsStorage {
    async fn write(
        &self,
        path: &str,
        data: &str,
        semaphore: &Semaphore,
    ) -> Result<(), custom_error::CustomError> {
        // Implement GCS write logic here

        let permit = semaphore.acquire().await.unwrap();

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
        drop(permit);
        Ok(())
    }
}

// Implement local file system storage
pub struct LocalStorage;

#[async_trait]
impl StorageProvider for LocalStorage {
    async fn write(
        &self,
        path: &str,
        data: &str,
        semaphore: &Semaphore,
    ) -> Result<(), custom_error::CustomError> {
        // Implement local file write logic here
        let permit = semaphore.acquire().await.unwrap();

        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        println!("Writing to local path: {}", path);

        drop(permit);
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
    ) -> Result<(), custom_error::CustomError> {
        let semaphore = Semaphore::new(MAX_CONCURRENT_WRITES);

        log::info!("Starting write {}...",path);

        self.storage.write(path, data, &semaphore).await?;
        log::info!("Done write {}...",path);
        Ok(())
    }
}
