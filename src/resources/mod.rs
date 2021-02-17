use std::collections::HashMap;

use ggez::graphics::Mesh;

pub enum ResourcesData {
    U32(u32),
    GgezMesh(Mesh),
}

impl ResourcesData {
    pub fn extract_u32(&self) -> Option<u32> {
        if let ResourcesData::U32(data) = self {
            Some(*data)
        } else {
            None
        }
    }

    pub fn extract_ggez_mesh(&self) -> Option<&Mesh> {
        if let Self::GgezMesh(mesh) = self {
            Some(mesh)
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Resources {
    data: HashMap<String, ResourcesData>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: &str, resource: ResourcesData) {
        self.data.insert(name.to_owned(), resource);
    }

    pub fn get(&self, name: &str) -> Option<&ResourcesData> {
        self.data.get(name)
    }
}
