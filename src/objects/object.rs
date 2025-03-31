pub type ObjectData = String;

pub trait GitObject {
    fn serialize(&self) -> ObjectData;
    fn deserialize(&mut self, data: ObjectData);
}


pub enum GitObjectType {
    Commit,
    Tree,
    Tag,
    Blob,
    Unknown(String),
}

impl GitObjectType {
    pub const COMMIT: &str = "commit";
    pub const TREE: &str = "tree";
    pub const TAG: &str = "tag";
    pub const BLOB: &str = "blob";

    pub fn from_str(s: &str) -> Self {
        match s {
            Self::COMMIT => GitObjectType::Commit,
            Self::TREE => GitObjectType::Tree,
            Self::TAG => GitObjectType::Tag,
            Self::BLOB => GitObjectType::Blob,
            other => GitObjectType::Unknown(other.to_string()),
        }
    }

}