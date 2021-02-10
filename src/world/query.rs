use std::any::TypeId;

pub struct Query {
    pub data_keys: Vec<TypeId>,
}

impl Query {
    pub fn new() -> Self {
        Self { data_keys: vec![] }
    }

    pub fn with_type<T: 'static>(mut self) -> Self {
        let type_id = TypeId::of::<T>();
        self.data_keys.push(type_id);
        self
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;

    #[test]
    fn create_a_query() {
        let query = Query::new().with_type::<f32>();
        let expected_result = vec![42.0_f32.type_id()];
        assert_eq!(query.data_keys, expected_result);
    }
}
