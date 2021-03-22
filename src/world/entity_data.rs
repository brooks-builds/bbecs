use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use eyre::Result;

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

    pub fn query_one(&self, name: &str) -> Result<&Vec<ComponentData>> {
        if let Some(components) = self.components.get(name) {
            Ok(components)
        } else {
            Err(BbEcsError::ComponentNotFound(name.to_owned()).into())
        }
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

// impl EntityDataTraits<Color> for EntityData {
//     fn insert(&mut self, name: &str, data: Color) -> Result<()> {
//         let mut wrapped_components = self.components.get_mut(name).unwrap().borrow_mut();
//         let components = wrapped_components.cast_mut()?;
//         components.push(data);
//         Ok(())
//     }
// }

// impl EntityDataTraits<f32> for EntityData {
//     fn insert(&mut self, name: &str, data: f32) -> Result<()> {
//         let mut wrapped_components = self.components.get(name).unwrap().borrow_mut();
//         let numbers = wrapped_components.cast_mut()?;
//         numbers.push(data);
//         Ok(())
//     }
// }

// impl EntityDataTraits<Mesh> for EntityData {
//     fn insert(&mut self, name: &str, data: Mesh) -> Result<()> {
//         let mut wrapped_meshes = self.components.get(name).unwrap().borrow_mut();
//         let meshes = wrapped_meshes.cast_mut()?;
//         meshes.push(data);
//         Ok(())
//     }
// }

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

// impl EntityDataTraits<usize> for EntityData {
//     fn insert(&mut self, name: &str, data: usize) -> Result<()> {
//         let mut wrapped_usizes = self.components.get_mut(name).unwrap().borrow_mut();
//         let usizes: &mut Vec<usize> = wrapped_usizes.cast_mut()?;
//         usizes.push(data);
//         Ok(())
//     }
// }

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

// impl EntityDataTraits<KeyCode> for EntityData {
//     fn insert(&mut self, name: &str, data: KeyCode) -> Result<()> {
//         let mut wrapped_key_code = self.components.get_mut(name).unwrap().borrow_mut();
//         let key_code: &mut Vec<KeyCode> = wrapped_key_code.cast_mut()?;
//         key_code.push(data);
//         Ok(())
//     }
// }

// impl EntityDataTraits<String> for EntityData {
//     fn insert(&mut self, name: &str, data: String) -> Result<()> {
//         let mut wrapped_marker = self.components.get_mut(name).unwrap().borrow_mut();
//         let markers: &mut Vec<String> = wrapped_marker.cast_mut()?;
//         markers.push(data);
//         Ok(())
//     }
// }

// impl EntityDataTraits<Text> for EntityData {
//     fn insert(&mut self, name: &str, data: Text) -> Result<()> {
//         let mut wrapped_texts = self.components.get_mut(name).unwrap().borrow_mut();
//         let texts: &mut Vec<Text> = wrapped_texts.cast_mut()?;
//         texts.push(data);
//         Ok(())
//     }
// }
