use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ggez::graphics::{Color, Mesh};

use crate::components::{CastComponents, Component};
use crate::{components::Components, data_types::point::Point};

pub trait EntityDataTraits<T> {
    fn insert(&mut self, name: &str, data: T);
}

pub struct EntityData {
    components: HashMap<String, Rc<RefCell<Components>>>,
}

impl EntityData {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register<T: Into<String>>(&mut self, name: T, component_type: Component) {
        let components = Rc::new(RefCell::new(match component_type {
            Component::Point => Components::Point(vec![]),
            Component::F32 => Components::F32(vec![]),
            Component::Color => Components::Color(vec![]),
            Component::Mesh => Components::Mesh(vec![]),
        }));
        self.components.insert(name.into(), components);
    }

    pub fn query_one(&self, name: &str) -> &Rc<RefCell<Components>> {
        self.components.get(name).unwrap()
    }
}

impl EntityDataTraits<Point> for EntityData {
    fn insert(&mut self, name: &str, data: Point) {
        let wrapped_components = self.components.get(name).unwrap();
        let mut components = wrapped_components.borrow_mut();
        let points = components.cast_mut();
        points.push(data);
    }
}

impl EntityDataTraits<Color> for EntityData {
    fn insert(&mut self, name: &str, data: Color) {
        let mut wrapped_components = self.components.get_mut(name).unwrap().borrow_mut();
        let components = wrapped_components.cast_mut();
        components.push(data);
    }
}

impl EntityDataTraits<f32> for EntityData {
    fn insert(&mut self, name: &str, data: f32) {
        let mut wrapped_components = self.components.get(name).unwrap().borrow_mut();
        let numbers = wrapped_components.cast_mut();
        numbers.push(data);
    }
}

impl EntityDataTraits<Mesh> for EntityData {
    fn insert(&mut self, name: &str, data: Mesh) {
        let mut wrapped_meshes = self.components.get(name).unwrap().borrow_mut();
        let meshes = wrapped_meshes.cast_mut();
        meshes.push(data);
    }
}

impl EntityDataTraits<u32> for EntityData {
    fn insert(&mut self, name: &str, data: u32) {
        let mut wrapped_u32s = self.components.get_mut(name).unwrap().borrow_mut();
        let u32s = wrapped_u32s.cast_mut();
        u32s.push(data);
    }
}
