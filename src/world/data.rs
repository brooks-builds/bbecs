use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

use super::query::Query;

#[derive(Debug)]
pub struct Data {
    pub data: HashMap<TypeId, Vec<Rc<dyn Any>>>,
}

impl Data {
    pub fn new() -> Self {
        let data = HashMap::new();
        Self { data }
    }

    pub fn insert(&mut self, new_data: impl Any) {
        // self.data
        //     .insert(new_data.type_id(), vec![Rc::new(new_data)]);

        let raw_data = self.data.entry(new_data.type_id()).or_insert(vec![]);
        raw_data.push(Rc::new(new_data));
    }

    pub fn query(&self, query: Query) -> Vec<&Vec<Rc<dyn Any>>> {
        query
            .data_keys
            .iter()
            .filter_map(|data_key| self.data.get(data_key))
            .collect()
    }

    pub fn query_mut<C: 'static + Send + Sync>(&mut self) -> Vec<Rc<C>> {
        let type_id = TypeId::of::<C>();
        let mut results = vec![];
        if let Some(components) = self.data.get_mut(&type_id) {
            components.clone().into_iter().for_each(|component| {
                results.push(component.downcast::<C>().unwrap());
            })
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::borrow::BorrowMut;
    use std::ops::{Deref, DerefMut};

    use crate::world::query::Query;

    use super::*;

    #[test]
    fn create_new_data() {
        let data = Data::new();
        assert_eq!(data.data.len(), 0);
    }

    #[test]
    fn insert_data() {
        let mut data = Data::new();
        let test_data_id = 32.type_id();
        data.insert(32);
        let raw_data = data.data.get(&test_data_id).unwrap()[0]
            .clone()
            .downcast::<i32>()
            .unwrap();
        assert_eq!(raw_data, Rc::new(32));
    }

    #[test]
    fn insert_multiple_data() {
        let mut data = Data::new();
        let component_1 = 32;
        let component_2 = 42;
        data.insert(component_1);
        data.insert(component_2);
        let components = data.data.get(&component_1.type_id()).unwrap();
        assert_eq!(components.len(), 2);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn query_for_multiple_data() {
        let mut data = Data::new();
        let entity_1 = 32_i32;
        let entity_2 = 64.0_f32;
        data.insert(entity_1);
        data.insert(entity_2);
        let query = Query::new().with_type::<i32>().with_type::<f32>();
        let entities = data.query(query);
        let entity_1_data = entities[0][0].clone().downcast::<i32>().unwrap();
        let entity_2_data = entities[1][0].clone().downcast::<f32>().unwrap();
        assert_eq!(entity_1, *entity_1_data);
        assert_eq!(entity_2, *entity_2_data);
    }

    #[test]
    fn mutably_query_for_data() {
        let mut data = Data::new();
        let test_component = TestComponent { x: 25 };
        data.insert(test_component);
        let mut query_results = data
            .query_mut::<TestComponent>()
            .iter_mut()
            // maybe do the get mut in the query function?
            // or more likely use an RefCell inside the Rc
            // https://doc.rust-lang.org/std/cell/index.html#introducing-mutability-inside-of-something-immutable
            .map(|component| Rc::get_mut(component).unwrap())
            .collect::<Vec<&mut TestComponent>>();
        query_results[0].x += 1;
        assert_eq!(*query_results[0], test_component);
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct TestComponent {
        x: i32,
    }

    impl TestComponent {
        pub fn increment(&mut self) {
            self.x += 1;
        }
    }
}
