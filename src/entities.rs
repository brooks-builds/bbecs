use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
};

use crate::query_builder::Query;

pub type Components = HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>;

#[derive(Debug, Default)]
pub struct Entities {
    pub components: Components,
    entity_count: usize,
    pub bitmask: Vec<u128>,
    bitmap: HashMap<TypeId, u128>,
}

impl Entities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_component(&mut self, component: impl Any) {
        self.components.insert(component.type_id(), vec![]);
        self.bitmap.insert(component.type_id(), 0);
    }

    pub fn register_entity(&mut self) -> &mut Self {
        self.entity_count += 1;
        self.add_empty_cell_to_all_components();
        self.bitmask.push(0);
        self
    }

    pub fn with_component(&mut self, component: impl Any) -> &mut Self {
        let type_id = component.type_id();
        if let Some(components) = self.components.get_mut(&type_id) {
            if let Some(last_component) = components.last_mut() {
                *last_component = Some(Rc::new(RefCell::new(component)));
            }
        } else {
            self.add_component(type_id, component);
        }
        let bitmap_value = self.bitmap.get(&type_id).unwrap();
        let last_bitmask = self.bitmask.last_mut().unwrap();
        *last_bitmask |= bitmap_value;
        self
    }

    pub fn add_empty_cell_to_all_components(&mut self) {
        for components in self.components.values_mut() {
            components.push(None);
        }
    }

    pub fn add_component(&mut self, type_id: TypeId, component: impl Any) {
        let mut components: Vec<Option<Rc<RefCell<dyn Any>>>> = vec![];
        for _ in 0..self.entity_count - 1 {
            components.push(None);
        }
        components.push(Some(Rc::new(RefCell::new(component))));
        self.components.insert(type_id, components);
        let new_bitmask = 2_u128.pow(self.bitmap.len() as u32);
        self.bitmap.insert(type_id, new_bitmask);
    }

    pub fn new_query(&self) -> Query {
        Query::new(self)
    }

    pub fn run_query(&self, query: Query) -> Vec<Vec<Rc<RefCell<dyn Any>>>> {
        let mut result = vec![];
        for type_id in query.type_ids.iter() {
            let mut queried_components = vec![];
            let components = self
                .components
                .get(type_id)
                .expect("Quering for a component that has not been registered");
            for (index, component) in components.iter().enumerate() {
                if query.indexes.contains(&index) {
                    queried_components.push(component.as_ref().unwrap().clone());
                }
            }
            result.push(queried_components);
        }
        result
    }

    pub fn entity_contains(&self, type_id: &TypeId, bitmask: u128) -> bool {
        if let Some(bitmap) = self.bitmap.get(&type_id) {
            bitmask & *bitmap == *bitmap
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_adding_entity() {
        let mut entities = Entities::new();
        entities
            .register_entity()
            .with_component(Size(10.0))
            .with_component(Location(0.0, 0.0));
        for components in entities.components.values() {
            assert_eq!(components.len(), entities.entity_count);
        }
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_querying_for_entities() {
        let mut entities = Entities::new();
        entities
            .register_entity()
            .with_component(Size(10.0))
            .with_component(Location(0.0, 0.0));

        entities
            .register_entity()
            .with_component(Size(11.0))
            .with_component(Location(1.0, 1.0));

        entities.register_entity().with_component(Size(12.0));

        let query = entities
            .new_query()
            .with_component::<Location>()
            .with_component::<Size>()
            .run();

        assert_eq!(query.len(), 2);
        assert_eq!(query[0].len(), 2);
        assert_eq!(
            query[0][0].borrow().downcast_ref::<Location>().unwrap().0,
            0.0
        );
        assert_eq!(
            query[0][1].borrow().downcast_ref::<Location>().unwrap().1,
            1.0
        );
        assert_eq!(query[1][0].borrow().downcast_ref::<Size>().unwrap().0, 10.0);
        assert_eq!(query[1][1].borrow().downcast_ref::<Size>().unwrap().0, 11.0);
    }

    #[derive(Debug)]
    struct Size(f32);

    #[derive(Debug)]
    struct Location(f32, f32);
}