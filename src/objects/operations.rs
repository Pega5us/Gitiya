use crate::constants::CompressionType;
use crate::objects::blob::GitBlob;
use crate::objects::object::{GitObject, GitObjectType};
use crate::repository::repository::GitRepository;
use crate::util::fs_utils;
use crate::util::hash::SHA;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use hex;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::Write;

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

pub fn write_object(object: Box<dyn GitObject>, repository: &GitRepository) -> Result<SHA, String> {
    let data = object.serialize();
    let header = format!("{} {}\0", object.format(), data.len());

    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    let sha = SHA::new(&hex::encode(result)).unwrap();
    let parent_path = fs_utils::join_paths_and_mkdir(
        repository.gitdirectory.as_path(),
        &["objects", sha.get_directory()],
        true,
    )?;
    let file_path = fs_utils::join_paths(parent_path.as_path(), &[sha.get_file()]);
    if !file_path.exists() {
        let file = File::create(&file_path).unwrap();
        let mut encoder = ZlibEncoder::new(file, Compression::default());
        encoder.write_all(&result).unwrap();
        encoder.finish().unwrap();
    }
    Ok(sha)
}
