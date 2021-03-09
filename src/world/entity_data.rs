use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh};

use crate::components::{CastComponents, Component};
use crate::errors::BbEcsError;
use crate::{components::Components, data_types::point::Point};

pub trait EntityDataTraits<T> {
    fn insert(&mut self, name: &str, data: T) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct EntityData {
    pub components: HashMap<String, Rc<RefCell<Components>>>,
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
            Component::Color => Components::Color(vec![]),
            Component::Mesh => Components::Mesh(vec![]),
            Component::U32 => Components::U32(vec![]),
            Component::Usize => Components::Usize(vec![]),
            Component::Bool => Components::Bool(vec![]),
            Component::GgezKeyCode => Components::GgezKeyCode(vec![]),
        }));
        self.components.insert(name, components);
    }

    pub fn query_one(&self, name: &str) -> Option<&Rc<RefCell<Components>>> {
        self.components.get(name)
    }
}

impl EntityDataTraits<Point> for EntityData {
    fn insert(&mut self, name: &str, data: Point) -> Result<()> {
        if let Some(wrapped_components) = self.components.get(name) {
            let mut components = wrapped_components.borrow_mut();
            let points = components.cast_mut()?;
            points.push(data);
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<Color> for EntityData {
    fn insert(&mut self, name: &str, data: Color) -> Result<()> {
        let mut wrapped_components = self.components.get_mut(name).unwrap().borrow_mut();
        let components = wrapped_components.cast_mut()?;
        components.push(data);
        Ok(())
    }
}

impl EntityDataTraits<f32> for EntityData {
    fn insert(&mut self, name: &str, data: f32) -> Result<()> {
        let mut wrapped_components = self.components.get(name).unwrap().borrow_mut();
        let numbers = wrapped_components.cast_mut()?;
        numbers.push(data);
        Ok(())
    }
}

impl EntityDataTraits<Mesh> for EntityData {
    fn insert(&mut self, name: &str, data: Mesh) -> Result<()> {
        let mut wrapped_meshes = self.components.get(name).unwrap().borrow_mut();
        let meshes = wrapped_meshes.cast_mut()?;
        meshes.push(data);
        Ok(())
    }
}

impl EntityDataTraits<u32> for EntityData {
    fn insert(&mut self, name: &str, data: u32) -> Result<()> {
        let mut wrapped_u32s = self.components.get_mut(name).unwrap().borrow_mut();
        let u32s = wrapped_u32s.cast_mut()?;
        u32s.push(data);
        Ok(())
    }
}

impl EntityDataTraits<usize> for EntityData {
    fn insert(&mut self, name: &str, data: usize) -> Result<()> {
        let mut wrapped_usizes = self.components.get_mut(name).unwrap().borrow_mut();
        let usizes: &mut Vec<usize> = wrapped_usizes.cast_mut()?;
        usizes.push(data);
        Ok(())
    }
}

impl EntityDataTraits<bool> for EntityData {
    fn insert(&mut self, name: &str, data: bool) -> Result<()> {
        let mut wrapped_boolean = self.components.get_mut(name).unwrap().borrow_mut();
        let boolean: &mut Vec<bool> = wrapped_boolean.cast_mut()?;
        boolean.push(data);
        Ok(())
    }
}

impl EntityDataTraits<KeyCode> for EntityData {
    fn insert(&mut self, name: &str, data: KeyCode) -> Result<()> {
        let mut wrapped_key_code = self.components.get_mut(name).unwrap().borrow_mut();
        let key_code: &mut Vec<KeyCode> = wrapped_key_code.cast_mut()?;
        key_code.push(data);
        Ok(())
    }
}
