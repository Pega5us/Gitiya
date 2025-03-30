
use std::path::Path;
use crate::repository::repository::GitRepository;
use crate::constants;

/// Recursively searches upward from the given `start_dir` to find the root of a Git repository.
///
/// # Arguments
/// * `start_dir` - The path representing the starting directory.
///
/// # Returns
/// * `Ok(GitRepository)` - If a `.git` directory is found, returns a `GitRepository` instance.
/// * `Err(String)` - If no `.git` directory is found before reaching the filesystem root.
pub fn find_repository_root(start_dir: &Path) -> Result<GitRepository, String> {
    let mut current_dir = start_dir;
     while !current_dir.join(constants::GIT_DIRECTORY).is_dir() {
        current_dir = current_dir.parent().ok_or("Reached filesystem root, no Git repository found")?;
    }
    GitRepository::new(current_dir, false)
}