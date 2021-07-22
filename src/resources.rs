use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

#[derive(Debug, Default)]
pub struct Resources {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, resource: impl Any) {
        self.data.insert(resource.type_id(), Box::new(resource));
    }

    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.data.get(&type_id)?.downcast_ref()
    }

    pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.data.get_mut(&type_id)?.downcast_mut()
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::float_cmp)]
    use eyre::Result;

    use super::*;

    #[test]
    fn resources() -> Result<()> {
        let mut resources = Resources::new();
        resources.add(Speed(42.0));
        let speed = resources.get::<Speed>().unwrap();
        assert_eq!(42.0, speed.0);
        Ok(())
    }

    struct Speed(f32);
}
