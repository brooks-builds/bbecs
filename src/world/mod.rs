mod entity_data;

use std::hash::Hash;

use entity_data::EntityData;
use ggez::graphics::{Color, Mesh};

use crate::components::Component;
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::{ResourceDataLens, ResourcesData};

use self::entity_data::EntityDataTraits;

pub trait WorldMethods<T> {
    fn with_component<S: Into<&'static str>>(&mut self, name: S, data: T) -> &mut Self;
    fn add_resource<S: Into<String>>(&mut self, name: S, data: T);
    fn get_resource(&self, name: &str) -> &T;
}

pub struct World {
    entity_data: EntityData,
    resources: ResourcesData,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<S: Into<String>>(&mut self, name: S, component_type: Component) {
        self.entity_data.register(name.into(), component_type);
    }

    pub fn spawn_entity(&mut self) -> &mut Self {
        self
    }
}

// todo implement the following methods
// register component
// insert spawn entity
// with component
// query one
// insert resource
// get resource

impl Default for World {
    fn default() -> Self {
        Self {
            entity_data: EntityData::new(),
            resources: ResourcesData::new(),
        }
    }
}

impl WorldMethods<Point> for World {
    fn with_component<S: Into<&'static str>>(&mut self, name: S, data: Point) -> &mut Self {
        self.entity_data.insert(name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Point) {
        self.resources.insert(name.into(), Resource::Point(data));
    }

    fn get_resource(&self, name: &str) -> &Point {
        todo!()
    }
}

impl WorldMethods<Color> for World {
    fn with_component<S: Into<&'static str>>(&mut self, name: S, data: Color) -> &mut Self {
        self.entity_data.insert(name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Color) {
        self.resources.insert(name.into(), Resource::Color(data));
    }

    fn get_resource(&self, name: &str) -> &Color {
        todo!()
    }
}

impl WorldMethods<Mesh> for World {
    fn with_component<S: Into<&'static str>>(&mut self, name: S, data: Mesh) -> &mut Self {
        todo!()
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Mesh) {
        self.resources.insert(name.into(), Resource::Mesh(data));
    }

    fn get_resource(&self, name: &str) -> &Mesh {
        todo!()
    }
}

impl WorldMethods<u32> for World {
    fn with_component<S: Into<&'static str>>(&mut self, name: S, data: u32) -> &mut Self {
        todo!()
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: u32) {
        self.resources.insert(name.into(), Resource::U32(data));
    }

    fn get_resource(&self, name: &str) -> &u32 {
        todo!()
    }
}

impl WorldMethods<f32> for World {
    fn with_component<S: Into<&'static str>>(&mut self, name: S, data: f32) -> &mut Self {
        self.entity_data.insert(name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: f32) {
        self.resources.insert(name.into(), Resource::F32(data));
    }

    fn get_resource(&self, name: &str) -> &f32 {
        todo!()
    }
}

impl WorldMethods<usize> for World {
    fn with_component<S: Into<&'static str>>(&mut self, name: S, data: usize) -> &mut Self {
        todo!()
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: usize) {
        self.resources.insert(name.into(), Resource::Usize(data));
    }

    fn get_resource(&self, name: &str) -> &usize {
        todo!()
    }
}
