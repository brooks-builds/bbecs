use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ggez::graphics::Color;

use crate::components::Component;
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

    pub fn register(&mut self, name: String, component_type: Component) {
        let components = Rc::new(RefCell::new(match component_type {
            Component::Point => Components::Point(vec![]),
            Component::F32 => Components::F32(vec![]),
        }));
        self.components.insert(name, components);
    }
}

impl EntityDataTraits<Point> for EntityData {
    fn insert(&mut self, name: &str, data: Point) {
        let wrapped_components = self.components.get(name).unwrap();
        let components = wrapped_components.borrow_mut();
    }
}

impl EntityDataTraits<Color> for EntityData {
    fn insert(&mut self, name: &str, data: Color) {
        todo!()
    }
}
