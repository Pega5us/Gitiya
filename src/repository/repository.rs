use std::path::{Path, PathBuf};
use ini::Ini;

pub struct GitRepository {
    pub worktree: PathBuf,
    pub gitdirectory: PathBuf,
    pub configpath: PathBuf,
}

impl GitRepository {
    pub fn new(path: &Path, force: bool) -> Result<Self, String> {
        let worktree = path.to_path_buf();
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
