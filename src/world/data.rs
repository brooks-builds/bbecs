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
}

#[cfg(test)]
mod tests {
    use std::any::Any;

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
}
