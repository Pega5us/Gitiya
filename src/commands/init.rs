use crate::constants;
use crate::fs_utils;
use std::path::{Path, PathBuf};
use ini::Ini;

pub struct GitRepository {
    worktree: PathBuf,
    gitdirectory: PathBuf,
    configpath: PathBuf,
}

impl GitRepository {
    pub fn new(path: &String, force: bool) -> Result<Self, String> {
        let worktree = PathBuf::from(path);
        let gitdirectory = worktree.join(".git");

        if !force && !gitdirectory.is_dir() {
            return Err("Not a Git repository".to_string());
        }

        // Read configuration file in .git/config
        let configpath = gitdirectory.join("config");
        let mut config = Ini::new();
        if configpath.exists() {
             config = Ini::load_from_file(&configpath).map_err(|e| e.to_string())?;
        } else if !force {
            return Err("Configuration file missing".to_string());
        }

       // Check repository format version
        if let Some(version_str) = config.get_from(Some("core"), "repositoryformatversion") {
            let version: i64 = version_str.parse().unwrap_or(0);
            if version != 0 {
                return Err(format!("Unsupported repositoryformatversion: {}", version));
            }
        }

        Ok(Self {
            worktree,
            gitdirectory,
            configpath,
        })
    }
}

/// Initializes a new repository at the given path.
///
/// # Arguments
/// * `path` :  The path where the repository should be initialized.
///
/// # Example
/// ```
/// cmd_init(".");
/// ```
pub fn cmd_init(path: String) {
    match create_repository(&path) {
        Ok(_) => println!("git repository created successfully!"),
        Err(e) => {
            eprintln!("Error: {}, while initialising git repository", e);
        }
    }
}

fn create_default_ini(file_path: &Path) -> Result<(), String> {
    let mut config = Ini::new();
    config
        .with_section(Some("core"))
        .set("repositoryformatversion", "0")
        .set("filemode", "true")
        .set("bare", "false");
    config.write_to_file(file_path).map_err(|e| {
        format!(
            "Failed to write ini file: {}, error: {}",
            file_path.display(),
            e
        )
    })
}

fn init_repository(repository: &GitRepository) -> Result<(), String> {
    let gitdir: &Path = repository.gitdirectory.as_path();
    fs_utils::join_paths_and_mkdir(gitdir, &["branches"], true)?;
    fs_utils::join_paths_and_mkdir(gitdir, &["objects"], true)?;
    fs_utils::join_paths_and_mkdir(gitdir, &["refs", "tags"], true)?;
    fs_utils::join_paths_and_mkdir(gitdir, &["refs", "heads"], true)?;
    fs_utils::write_to_file(gitdir, "description", constants::DEFAULT_DESCRIPTION)?;
    fs_utils::write_to_file(gitdir, "HEAD", constants::DEFAULT_HEAD)?;
    create_default_ini(repository.configpath.as_path())?;
    Ok(())
}

fn create_repository(path: &String) -> Result<(), String> {
    let repository = GitRepository::new(path, true)?;

    // First, we make sure the path either doesn't exist or is an empty dir.
    if repository.worktree.exists() {
        if !repository.worktree.is_dir() {
            return Err(format!("{} is not a directory", path));
        }
        if repository.gitdirectory.exists()
            && !fs_utils::is_directory_empty(repository.gitdirectory.as_path())?
        {
            return Err(format!("{} is not empty!", path));
        }
    } else {
        fs_utils::create_directory(repository.worktree.as_path())?;
    }
    init_repository(&repository)?;
    return Ok(());
}
