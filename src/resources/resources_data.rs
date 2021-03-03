use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

use ggez::graphics::Color;

use crate::data_types::point::Point;

use super::resource::{Resource, ResourceCast};

pub trait ResourceDataLens<T> {
    fn get(&self, name: &str) -> &T;
    fn get_mut(&mut self, name: &str) -> &mut T;
}

pub struct ResourcesData {
    resources: HashMap<String, Resource>,
}

impl ResourcesData {
    pub fn new() -> Self {
        Self::default()
    }

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
    fn get(&self, name: &str) -> &Point {
        let resource = self.resources.get(name).unwrap().cast();
        resource
    }

    fn get_mut(&mut self, name: &str) -> &mut Point {
        todo!()
    }
}

impl ResourceDataLens<Color> for ResourcesData {
    fn get(&self, name: &str) -> &Color {
        self.resources.get(name).unwrap().cast()
    }

    fn get_mut(&mut self, name: &str) -> &mut Color {
        self.resources.get_mut(name).unwrap().cast_mut()
    }
}
