mod common;

use std::fs;

use ray_dashboard_sdk::{PackagesAPI, RayDashboardClient};

#[tokio::test]
async fn test_upload_and_check_package() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();

    // Create a temporary directory with some files
    let temp_dir = tempfile::tempdir().unwrap();

    // Create some test files
    fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
    fs::write(temp_dir.path().join("file2.txt"), "content2").unwrap();

    // Upload the directory
    let package_uri = client.upload_directory(temp_dir.path()).await.unwrap();

    // Verify the package exists
    let exists = client.package_exists(&package_uri).await.unwrap();
    assert!(exists, "Package should exist after upload");
}

#[tokio::test]
async fn test_upload_directory_if_needed_idempotent() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();

    // Create a temporary directory with some files
    let temp_dir = tempfile::tempdir().unwrap();

    // Create some test files
    fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
    fs::write(temp_dir.path().join("file2.txt"), "content2").unwrap();

    // Upload the directory twice
    let uri1 = client.upload_directory_if_needed(temp_dir.path()).await.unwrap();
    let uri2 = client.upload_directory_if_needed(temp_dir.path()).await.unwrap();

    // Both URIs should be the same since the content is identical
    assert_eq!(uri1.to_string(), uri2.to_string());
}

#[tokio::test]
async fn test_package_does_not_exist() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();

    // Create a package URI that definitely doesn't exist
    let non_existent_uri = "gcs://_ray_pkg_nonexistent.zip";

    let exists = client.package_exists(non_existent_uri).await.unwrap();
    assert!(!exists, "Non-existent package should not exist");
}
