use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::components::Component;

pub struct Data {
    pub data: HashMap<String, Cell<Vec<Component>>>,
}

impl Data {
    pub fn new() -> Self {
        let data = HashMap::new();
        Self { data }
    }

    pub fn insert(&mut self, entity_name: &str, new_data: Component) {
        let raw_data = self.data.entry(entity_name.to_owned()).or_default();
        raw_data.get_mut().push(new_data);
    }

    pub fn query_one_mut(&mut self, component_name: &str) -> Option<&mut Cell<Vec<Component>>> {
        self.data.get_mut(component_name)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    #[test]
    fn create_new_data() {
        let data = Data::new();
        assert_eq!(data.data.len(), 0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn insert_data() {
        let mut data = Data::new();
        let mut location = Component::create_vector_2(10.0, 10.0);
        data.insert("location", location);
        let locations = data.query_one_mut("location").unwrap().get_mut();
        assert!(
            matches!(locations[0], Component::Vector2(vector2) if vector2.x == 10.0 && vector2.y == 10.0)
        );
    }

    #[test]
    fn insert_multiple_data() {
        let mut data = Data::new();
        let location1 = Component::create_vector_2(0.0, 0.0);
        let location2 = Component::create_vector_2(10.0, 20.0);
        let component_name = "location";
        data.insert(component_name, location1);
        data.insert(component_name, location2);
        let components = data.data.get_mut(component_name).unwrap().get_mut();
        assert_eq!(components.len(), 2);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_querying_one_component_mutably() {
        let mut data = Data::new();
        let location = Component::create_vector_2(0.0, 0.0);
        data.insert("location", location);
        let locations_after_query = data.query_one_mut("location").unwrap();
        for queried_location in locations_after_query.get_mut() {
            let vector = queried_location.get_vector_2_mut().unwrap();
            vector.x += 1.0;
        }
        let location_after_mutation = data.data.get_mut("location").unwrap().get_mut()[0];
        assert!(
            matches!(location_after_mutation, Component::Vector2(vector) if vector.x == 1.0 && vector.y == 0.0)
        );
    }

    // #[test]
    // #[allow(clippy::float_cmp)]
    // fn test_querying_immutably_for_data() {
    //     let mut data = Data::new();
    //     let x = 0.0;
    //     let y = 10.0;
    //     let location = Component::create_vector_2(x, y);
    //     let component_name = "location";
    //     data.insert(component_name, location);

    //     if let Some(locations) = data.query(component_name) {
    //         assert!(
    //             matches!(locations[0], Component::Vector2(vector2) if vector2.x == x && vector2.y == y)
    //         );
    //     } else {
    //         panic!("We need to find locations");
    //     }
    // }
}
