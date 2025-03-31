use crate::objects::object::{GitObject, ObjectData};

#[derive(Clone)]
pub struct GitBlob {
    data: ObjectData,
}

impl GitBlob {
    pub fn new(data: ObjectData) -> Self {
        Self { data }
    }
}

impl GitObject for GitBlob {
    fn serialize(&self) -> ObjectData {
        self.data.clone()
    }

    fn deserialize(&mut self, data: ObjectData) {
        self.data = data;
    }
}