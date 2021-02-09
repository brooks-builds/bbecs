use data::Data;

mod data;

pub struct World {
    data: Data,
}

impl World {
    pub fn new() -> Self {
        let data = Data::new();
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::{data::Data, World};

    #[test]
    fn create_new_world() {
        let world = World::new();
        let expected_data = Data::new();
        assert_eq!(world.data, expected_data);
    }
}
