use std::collections::HashMap;

use eyre::Result;

use crate::errors::BbEcsError;

#[derive(Debug, Default)]
pub struct BitMap {
    entity_map: HashMap<String, Vec<bool>>,
    length: usize,
}

impl BitMap {
    pub fn new() -> Self {
        Self {
            entity_map: HashMap::new(),
            length: 0,
        }
    }

    pub fn register(&mut self, name: String) {
        self.entity_map.insert(name, vec![]);
    }

    pub fn spawn_entity(&mut self) {
        self.length += 1;
        for components in &mut self.entity_map.values_mut() {
            components.push(false);
        }
    }

    pub fn insert(&mut self, name: &str) -> Result<()> {
        if let Some(components) = self.entity_map.get_mut(name) {
            components[self.length - 1] = true;
        } else {
            return Err(BbEcsError::BitMapInsertBeforeRegister.into());
        }

        Ok(())
    }

    pub fn query(&self, names: Vec<&str>) -> Result<Vec<&Vec<bool>>> {
        let mut results = vec![];

        for name in names {
            if let Some(map) = self.entity_map.get(name) {
                results.push(map);
            } else {
                return Err(BbEcsError::BitMapComponentNotFound(name.to_owned()).into());
            }
        }

        Ok(results)
    }
}