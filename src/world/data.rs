use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

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
        self.data
            .insert(new_data.type_id(), vec![Rc::new(new_data)]);
    }

    pub fn query<T: Send + Sync + 'static, S: Send + Sync + 'static>(&self) -> () {
        dbg!("**********************");
        dbg!(std::any::type_name::<T>());
        dbg!("************************");
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

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
    fn query_for_data() {
        let mut data = Data::new();
        let entity = 32_i32;
        let type_id = entity.type_id();
        data.insert(entity);
        // We are going to need to create a query struct that uses the builder pattern to create queries for types one at a time
        let query = Query::new()
            .with_type::<i32>() // takes in the type and stores the type id
            .with_type::<f32>()
            .build();
        let result = data.query(query);
    }
}
