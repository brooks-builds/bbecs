use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{components::Components, data_types::point::Point};

pub trait EntityDataTraits<T> {
    fn register(&mut self, name: String);
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
}

impl EntityDataTraits<Point> for EntityData {
    fn register(&mut self, name: String) {
        self.components
            .insert(name, Rc::new(RefCell::new(Components::Point(vec![]))));
    }

    fn insert(&mut self, name: &str, data: Point) {
        let wrapped_components = self.components.get(name).unwrap();
        let components = wrapped_components.borrow_mut();
    }
}
