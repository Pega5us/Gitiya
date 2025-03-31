use crate::objects::object::GitObjectType;
use crate::objects::object::{FormatType, GitObject, ObjectData};

#[derive(Clone)]
pub struct GitBlob {
    data: ObjectData,
    format: FormatType,
}

impl GitBlob {
    pub fn new(data: ObjectData) -> Self {
        Self {
            data,
            format: GitObjectType::BLOB.to_string(),
        }
    }
}

impl GitObject for GitBlob {
    fn serialize(&self) -> ObjectData {
        self.data.clone()
    }

    fn deserialize(&mut self, data: ObjectData) {
        self.data = data;
    }

    fn format(&self) -> &str {
        &self.format
    }
}
