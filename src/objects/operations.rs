use crate::constants::CompressionType;
use crate::objects::blob::GitBlob;
use crate::objects::object::{GitObject, GitObjectType};
use crate::repository::repository::GitRepository;
use crate::util::fs_utils;
use crate::util::hash::SHA;

fn validate_object(data: &str) -> Result<(), String> {
    let data_bytes = data.as_bytes();
    // Find the first space (b' ') to separate the type
    let x = data_bytes
        .iter()
        .position(|&c| c == b' ')
        .ok_or_else(|| format!("Invalid object: Missing space separator"))?;
    // Find the null terminator (b'\x00') after the space
    let y = x + data_bytes
        .iter()
        .skip(x)
        .position(|&c| c == b'\x00')
        .ok_or_else(|| format!("Invalid object: Missing null byte separator"))?;
    // Extract the size field and convert it to an integer
    let size: usize = String::from_utf8_lossy(&data_bytes[x + 1..y])
        .parse()
        .map_err(|_| format!("Invalid object: Failed to parse size"))?;

    if size != data_bytes.len() - y - 1 {
        return Err(format!(
            "Malformed object: bad length (expected {}, found {})",
            size,
            data_bytes.len() - y - 1
        ));
    }
    Ok(())
}

pub fn object_read(repository: &GitRepository, sha: &SHA) -> Result<Box<dyn GitObject>, String> {
    let object_path = fs_utils::join_paths(
        repository.gitdirectory.as_path(),
        &["objects", &sha[0..2], &sha[2..]],
    );
    let raw_data =
        fs_utils::read_and_decompress_file(object_path.as_path(), CompressionType::Zlib)?;

    validate_object(&raw_data)?;

    if let Some(x) = raw_data.find(' ') {
        let format = &raw_data[..x];
        return match GitObjectType::from_str(format) {
            GitObjectType::Commit => Err("TODO: not supported yet!".to_string()),
            GitObjectType::Tree => Err("TODO: not supported yet!".to_string()),
            GitObjectType::Tag => Err("TODO: not supported yet!".to_string()),
            GitObjectType::Blob => Ok(Box::new(GitBlob::new(raw_data))),
            GitObjectType::Unknown(_) => Err("Unknown object type".to_string()),
        };
    }
    Err(format!("unable to read object"))
}
