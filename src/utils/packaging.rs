use ignore::WalkBuilder;
use sha1::{Digest, Sha1};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use tracing::{debug, warn};
use zip::{ZipWriter, write::SimpleFileOptions};

use crate::constants::{RAY_PKG_PREFIX, RAY_PKG_PROTOCOL};

/// Create a zip package from a directory
/// Always respects .gitignore files
pub fn create_package(source_dir: &Path, output_path: &Path) -> crate::Result<()> {
    debug!("Creating package from {:?} to {:?}", source_dir, output_path);

    // Create parent directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);
    let options_zip = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // Use ignore crate for gitignore support - collect files to include
    let walker = WalkBuilder::new(source_dir)
        .standard_filters(true)
        .git_ignore(true)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                let rel_path = path
                    .strip_prefix(source_dir)
                    .map_err(|e| crate::Error::Generic(format!("Failed to compute relative path: {}", e)))?;

                if path.is_file() {
                    debug!("Adding file to zip: {:?}", rel_path);
                    zip.start_file(rel_path.to_string_lossy().to_string(), options_zip)?;

                    let mut file = File::open(path)?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    zip.write_all(&buffer)?;
                } else if path.is_dir() && !rel_path.as_os_str().is_empty() {
                    debug!("Adding directory to zip: {:?}", rel_path);
                    zip.add_directory(rel_path.to_string_lossy().to_string(), options_zip)?;
                }
            }
            Err(e) => {
                warn!("Error walking directory: {}", e);
            }
        }
    }

    zip.finish()?;

    debug!("Package created successfully at {:?}", output_path);
    Ok(())
}

/// Compute a hash of a directory's contents
/// Always respects .gitignore files
pub fn hash_directory(directory: &Path) -> crate::Result<String> {
    let mut hasher = Sha1::new();

    // Use ignore crate for gitignore support - always enabled
    let walker = WalkBuilder::new(directory)
        .standard_filters(true)
        .git_ignore(true)
        .build();

    let mut paths: Vec<PathBuf> = Vec::new();
    for result in walker {
        match result {
            Ok(entry) => {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    paths.push(entry.path().to_path_buf());
                }
            }
            Err(e) => {
                warn!("Error walking directory: {}", e);
            }
        }
    }

    // Sort for deterministic hashing
    paths.sort();

    for path in paths {
        let rel_path = path
            .strip_prefix(directory)
            .map_err(|e| crate::Error::Generic(format!("Failed to compute relative path: {}", e)))?;

        // Hash the relative path
        hasher.update(rel_path.to_string_lossy().as_bytes());

        // Hash the file contents
        let mut file = File::open(&path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        hasher.update(&buffer);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Get a URI for a directory (creates a content-addressed URI)
/// Always respects .gitignore files
pub fn get_uri_for_directory(directory: &Path) -> crate::Result<String> {
    let hash = hash_directory(directory)?;
    let package_name = format!("{}{}.zip", RAY_PKG_PREFIX, hash);
    Ok(format!("{}://{}", RAY_PKG_PROTOCOL, package_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_hash_directory_deterministic() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create some test files
        fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "content2").unwrap();

        let hash1 = hash_directory(temp_dir.path()).unwrap();
        let hash2 = hash_directory(temp_dir.path()).unwrap();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_create_package() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create some test files
        fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "content2").unwrap();

        let output_dir = tempfile::tempdir().unwrap();
        let output_path = output_dir.path().join("package.zip");
        create_package(temp_dir.path(), &output_path).unwrap();

        assert!(output_path.exists());
    }

    #[test]
    fn test_get_uri_for_directory() {
        let temp_dir = tempfile::tempdir().unwrap();

        fs::write(temp_dir.path().join("file.txt"), "content").unwrap();

        let uri = get_uri_for_directory(temp_dir.path()).unwrap();

        assert!(uri.starts_with(&format!("{}://", RAY_PKG_PROTOCOL)));
        assert!(uri.contains(RAY_PKG_PREFIX));
        assert!(uri.ends_with(".zip"));
    }
}
