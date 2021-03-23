use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::errors::BbEcsError;
use crate::{components::ComponentData, data_types::point::Point};

pub trait EntityDataTraits<T> {
    fn insert(&mut self, name: &str, data: T) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct EntityData {
    pub components: HashMap<String, Vec<ComponentData>>,
}

impl EntityData {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String) -> Result<()> {
        if self.components.contains_key(&name) {
            return Err(BbEcsError::ComponentAlreadyRegistered(name).into());
        }
        let components = vec![];
        self.components.insert(name, components);
        Ok(())
    }

    pub fn query(
        &self,
        names: Vec<&str>,
        bitmap: Vec<&Vec<bool>>,
    ) -> Result<Vec<Vec<&ComponentData>>> {
        let mut current_name_indexes = vec![0; bitmap.len()];
        let mut results = vec![vec![]; names.len()];
        let mut components = vec![];

        for name in names {
            if let Some(raw_components) = self.components.get(name) {
                components.push(raw_components);
            } else {
                return Err(BbEcsError::ComponentNotFound(name.to_owned()).into());
            }
        }

        // for each entity in the bitmap
        for entity_index in 0..bitmap[0].len() {
            // for each component in the entity
            let mut is_adding_component = true;
            for (components_bitmap_index, components_bitmap) in bitmap.iter().enumerate() {
                // update the name index
                if components_bitmap[entity_index] {
                    if entity_index != 0 {
                        current_name_indexes[components_bitmap_index] += 1;
                    }
                } else {
                    is_adding_component = false;
                }
            }
            // are all of the components queried for present
            if is_adding_component {
                for (component_index, component) in components.iter().enumerate() {
                    results[component_index]
                        .push(&component[current_name_indexes[component_index]]);
                }
            }
        }

        Ok(results)
    }

    pub fn delete_entities_by_index(&mut self, bitmap: HashMap<String, Vec<usize>>) -> Result<()> {
        for (component_name, mut indexes_to_delete) in bitmap {
            indexes_to_delete.reverse();
            if let Some(components) = self.components.get_mut(&component_name) {
                for index in indexes_to_delete {
                    components.remove(index);
                }
            }
        }

        Ok(())
    }
}

impl EntityDataTraits<Point> for EntityData {
    fn insert(&mut self, name: &str, data: Point) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::Point(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<Color> for EntityData {
    fn insert(&mut self, name: &str, data: Color) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::Color(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<f32> for EntityData {
    fn insert(&mut self, name: &str, data: f32) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::F32(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<Mesh> for EntityData {
    fn insert(&mut self, name: &str, data: Mesh) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::Mesh(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<u32> for EntityData {
    fn insert(&mut self, name: &str, data: u32) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::U32(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<usize> for EntityData {
    fn insert(&mut self, name: &str, data: usize) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::Usize(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<bool> for EntityData {
    fn insert(&mut self, name: &str, data: bool) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::Bool(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<KeyCode> for EntityData {
    fn insert(&mut self, name: &str, data: KeyCode) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::GgezKeyCode(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<String> for EntityData {
    fn insert(&mut self, name: &str, data: String) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::Marker(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}

impl EntityDataTraits<Text> for EntityData {
    fn insert(&mut self, name: &str, data: Text) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components.push(ComponentData::GgezText(Rc::new(RefCell::new(data))));
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}
