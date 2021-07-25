use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt::Debug,
    rc::Rc,
};

use eyre::Result;

use crate::entities::Entities;

pub type QueryResult = Vec<Vec<Rc<RefCell<dyn Any>>>>;

#[derive(Debug)]
pub struct Query<'a> {
    pub type_ids: Vec<TypeId>,
    pub indexes: Vec<usize>,
    entities: &'a Entities,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            type_ids: vec![],
            indexes: vec![],
            entities,
        }
    }

    pub fn with_component<T: 'static + Sized + Debug>(mut self) -> Self {
        self.type_ids.push(TypeId::of::<T>());
        self
    }

    pub fn run(mut self) -> Result<(Vec<usize>, QueryResult)> {
        for (index, entity_bitmask) in self.entities.bitmask.iter().enumerate() {
            let mut has_component = true;
            for type_id in self.type_ids.iter() {
                if !self.entities.entity_contains(type_id, *entity_bitmask) {
                    has_component = false;
                    break;
                }
            }
            if has_component {
                self.indexes.push(index);
            }
        }
        Ok((self.indexes.clone(), self.entities.run_query(self)?))
    }
}
