use async_trait::async_trait;
use std::path::Path;
use tokio::{fs::File, io::AsyncReadExt};
use tracing::debug;

use crate::{
    RayDashboardClient,
    utils::packaging::{create_package, get_uri_for_directory, get_uri_for_package},
};

#[async_trait]
pub trait PackagesAPI {
    /// Check if a package exists on the Ray cluster
    async fn package_exists(&self, package_uri: &str) -> crate::Result<bool>;

    /// Upload a package to the Ray cluster
    async fn upload_package(&self, package_uri: &str, data: Vec<u8>) -> crate::Result<()>;

    /// Upload a package file to the Ray cluster
    async fn upload_package_file(&self, package_path: &Path) -> crate::Result<String>;

    /// Upload a directory as a package to the Ray cluster (always respects .gitignore)
    async fn upload_directory(&self, directory: &Path) -> crate::Result<String>;

    /// Upload a package if it doesn't already exist
    async fn upload_package_if_needed(&self, package_uri: &str, data: Vec<u8>) -> crate::Result<()>;

    /// Upload a directory if it doesn't already exist (always respects .gitignore)
    async fn upload_directory_if_needed(&self, directory: &Path) -> crate::Result<String>;
}

#[async_trait]
impl PackagesAPI for RayDashboardClient {
    async fn package_exists(&self, package_uri: &str) -> crate::Result<bool> {
        // Parse URI to get protocol and package name
        let parts: Vec<&str> = package_uri.split("://").collect();
        if parts.len() != 2 {
            return Err(crate::Error::Generic(format!(
                "Invalid package URI: {}",
                package_uri
            )));
        }
        let (protocol, package_name) = (parts[0], parts[1]);

        let path = format!("/api/packages/{}/{}", protocol, package_name);
        let request = self.base_request(reqwest::Method::GET, &path)?;

        match request.send().await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => Ok(true),
                reqwest::StatusCode::NOT_FOUND => Ok(false),
                _ => Err(crate::Error::Generic(format!(
                    "Unexpected status code checking package existence: {}",
                    response.status()
                ))),
            },
            Err(e) => Err(crate::Error::from(e)),
        }
    }

    async fn upload_package(&self, package_uri: &str, data: Vec<u8>) -> crate::Result<()> {
        // Parse URI to get protocol and package name
        let parts: Vec<&str> = package_uri.split("://").collect();
        if parts.len() != 2 {
            return Err(crate::Error::Generic(format!(
                "Invalid package URI: {}",
                package_uri
            )));
        }
        let (protocol, package_name) = (parts[0], parts[1]);

        let path = format!("/api/packages/{}/{}", protocol, package_name);
        let request = self.base_request(reqwest::Method::PUT, &path)?;

        debug!("Uploading package {} ({} bytes)", package_uri, data.len());

        let response = request.body(data).send().await?.error_for_status()?;

        debug!("Package uploaded successfully: {}", response.status());
        Ok(())
    }

    async fn upload_package_file(&self, package_path: &Path) -> crate::Result<String> {
        let package_uri = get_uri_for_package(package_path)?;

        let mut file = File::open(package_path).await?;

        let mut data = Vec::new();
        file.read_to_end(&mut data).await?;

        self.upload_package(&package_uri, data).await?;
        Ok(package_uri)
    }

    async fn upload_directory(&self, directory: &Path) -> crate::Result<String> {
        // Create a temporary zip file
        let temp_dir = std::env::temp_dir();
        let package_uri = get_uri_for_directory(directory)?;
        let temp_package_path = temp_dir.join(format!("ray_pkg_{}.zip", uuid::Uuid::new_v4()));

        // Create the package
        create_package(directory, &temp_package_path)?;

        // Read the package data
        let mut file = File::open(&temp_package_path).await?;

        let mut data = Vec::new();
        file.read_to_end(&mut data).await?;

        // Upload the package
        self.upload_package(&package_uri, data).await?;

        // Clean up temp file
        std::fs::remove_file(&temp_package_path)?;

        Ok(package_uri)
    }

    async fn upload_package_if_needed(&self, package_uri: &str, data: Vec<u8>) -> crate::Result<()> {
        if !self.package_exists(package_uri).await? {
            self.upload_package(package_uri, data).await?;
        } else {
            debug!("Package {} already exists, skipping upload", package_uri);
        }
        Ok(())
    }

    async fn upload_directory_if_needed(&self, directory: &Path) -> crate::Result<String> {
        let package_uri = get_uri_for_directory(directory)?;

        if !self.package_exists(&package_uri).await? {
            self.upload_directory(directory).await?;
        } else {
            debug!("Package {} already exists, skipping upload", package_uri);
        }

        Ok(package_uri)
    }
}
