use std::collections::HashMap;

use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh};

use crate::data_types::point::Point;

use super::resource::{Resource, ResourceCast};

pub trait ResourceDataLens<T> {
    fn get(&self, name: &str) -> Result<&T>;
    fn get_mut(&mut self, name: &str) -> Result<&mut T>;
}

pub struct ResourcesData {
    resources: HashMap<String, Resource>,
}

impl ResourcesData {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a resource into storage, overwriting any resource that had the same name
    pub fn insert(&mut self, name: String, resource: Resource) {
        self.resources.insert(name, resource);
    }
}

impl Default for ResourcesData {
    fn default() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}

impl ResourceDataLens<Point> for ResourcesData {
    fn get(&self, name: &str) -> Result<&Point> {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> Result<&mut Point> {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}

impl ResourceDataLens<Color> for ResourcesData {
    fn get(&self, name: &str) -> Result<&Color> {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> Result<&mut Color> {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}

impl ResourceDataLens<Mesh> for ResourcesData {
    fn get(&self, name: &str) -> Result<&Mesh> {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> Result<&mut Mesh> {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}

impl ResourceDataLens<u32> for ResourcesData {
    fn get(&self, name: &str) -> Result<&u32> {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> Result<&mut u32> {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}

impl ResourceDataLens<f32> for ResourcesData {
    fn get(&self, name: &str) -> Result<&f32> {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> Result<&mut f32> {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}

impl ResourceDataLens<usize> for ResourcesData {
    fn get(&self, name: &str) -> Result<&usize> {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> Result<&mut usize> {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}

impl ResourceDataLens<bool> for ResourcesData {
    fn get(&self, name: &str) -> Result<&bool> {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> Result<&mut bool> {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}

impl ResourceDataLens<KeyCode> for ResourcesData {
    fn get(&self, name: &str) -> Result<&KeyCode> {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> Result<&mut KeyCode> {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}
