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
}
