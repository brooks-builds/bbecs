use std::collections::{BTreeMap, HashMap};

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

    pub fn query(&self, names: Vec<&str>) -> Result<BTreeMap<String, &Vec<bool>>> {
        let mut results = BTreeMap::new();

        for name in names {
            if let Some(map) = self.entity_map.get(name) {
                results.insert(name.to_owned(), map);
            } else {
                return Err(BbEcsError::BitMapComponentNotFound(name.to_owned()).into());
            }
        }

        Ok(results)
    }

    pub fn calculate_component_indexes_to_delete(
        &self,
        entity_indexes: &[usize],
    ) -> Result<HashMap<String, Vec<usize>>> {
        let mut component_indexes_to_delete = HashMap::new();

        for (component_name, bitmap) in &self.entity_map {
            let mut indexes_to_delete = vec![];

            for entity_index in entity_indexes {
                if !bitmap[*entity_index] {
                    continue;
                }

                indexes_to_delete
                    .push(entity_index - self.count_falses_before_index(bitmap, *entity_index)?);
            }

            component_indexes_to_delete.insert(component_name.to_owned(), indexes_to_delete);
        }

        Ok(component_indexes_to_delete)
    }

    pub fn delete_entities_by_index(&mut self, mut entity_indexes: Vec<usize>) -> Result<()> {
        entity_indexes.reverse();
        for components in self.entity_map.values_mut() {
            for entity_index in &entity_indexes {
                components.remove(*entity_index);
            }
        }
        self.length -= entity_indexes.len();
        Ok(())
    }

    fn count_falses_before_index(&self, components: &[bool], index: usize) -> Result<usize> {
        if index >= components.len() {
            return Err(BbEcsError::OutOfRangeInVector.into());
        }

        let total_falses: &Vec<bool> = &components[0..index]
            .iter()
            .filter(|current| !*current)
            .cloned()
            .collect();

        Ok(total_falses.len())
    }
}
