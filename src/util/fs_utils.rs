use std::fs;
use std::path::{Path, PathBuf};

/// Checks if a directory is empty.
///
/// # Arguments
/// * `directory` - Path of the directory to check.
///
/// # Returns
/// * `Ok(true)` if the directory is empty.
/// * `Ok(false)` if the directory is not empty.
/// * `Err(String)` if an error occurs while reading the directory.
pub fn is_directory_empty(directory: &Path) -> Result<bool, String> {
    let mut entries = fs::read_dir(directory).map_err(|e| {
        format!(
            "Failed to read directory {}: error: {}",
            directory.display(),
            e
        )
    })?;
    Ok(entries.next().is_none())
}

/// Creates a directory and all parent directories if needed.
///
/// # Arguments
/// * `directory` - Path representing the directory to create.
///
/// # Returns
/// * `Ok(())` if the directory was created or already exists.
/// * `Err(String)` if an error occurs while creating the directory.
pub fn create_directory(directory: &Path) -> Result<(), String> {
    fs::create_dir_all(directory).map_err(|e| {
        format!(
            "Failed to create directory {}: error: {}",
            directory.display(),
            e
        )
    })
}

/// Joins base_path with other provided paths
///
/// # Arguments
/// * `base_path` - parent directory path
/// * `paths`  - subpaths to be appended in base_path
///
/// # Returns
/// * `PathBuf` representing the computed path.
pub fn join_paths(base_path: &Path, paths: &[&str]) -> PathBuf {
    let mut full_path = base_path.to_path_buf();
    for p in paths {
        full_path.push(p);
    }
    full_path
}

/// Joins multiple path components to a given base_path and optionally creates the resulting directory.
///
/// # Arguments
/// * `base_path` - parent directory path
/// * `paths`  - subpaths to be appended in base_path
/// * `mkdir`  - if true, then computed path will be created if it doesn't exist
///
/// * `Ok(PathBuf)` - The fully constructed path if it exists or was successfully created.
/// * `Err(String)` - An error message if the path is not a directory, or if it doesn't exist and wasn't created.
pub fn join_paths_and_mkdir(base_path: &Path, paths: &[&str], mkdir: bool) -> Result<PathBuf, String> {
    let full_path = join_paths(base_path, paths);
    if full_path.exists() {
        if full_path.is_dir() {
            return Ok(full_path);
        } else {
            return Err(format!("{} is not a directory.", full_path.display()));
        }
    }
    if mkdir {
        create_directory(full_path.as_path())?;
        return Ok(full_path);
    }
    return Err(format!(
        "{} doesn't exist, neither created!",
        full_path.display()
    ));
}

/// Writes the given text to a specified file inside the given directory.
///
/// # Arguments
/// * `base_path` - The directory where the file should be created.
/// * `filename` - The name of the file to write.
/// * `text` - The content to write inside the file.
///
/// # Returns
/// * `Ok(())` - If the file is written successfully.
/// * `Err(String)` - If an error occurs while rwriting
pub fn write_to_file(base_path: &Path, filename: &str, text: &str) -> Result<(), String> {
    let file_path = base_path.join(filename);
    fs::write(&file_path, text)
        .map_err(|e| format!("Failed to write '{}': {}", file_path.display(), e))
}
