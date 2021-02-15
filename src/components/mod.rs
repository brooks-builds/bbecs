use nalgebra::Vector2;

#[derive(Debug, Clone, Copy)]
pub enum Component {
    Vector2(Vector2<f32>),
}

impl Component {
    pub fn create_vector_2(x: f32, y: f32) -> Self {
        Self::Vector2(Vector2::new(x, y))
    }

    #[allow(irrefutable_let_patterns)]
    pub fn get_vector_2_mut(&mut self) -> Option<&mut Vector2<f32>> {
        if let Self::Vector2(vector2) = self {
            Some(vector2)
        } else {
            None
        }
    }

    #[allow(irrefutable_let_patterns)]
    pub fn get_vector_2(&self) -> Option<&Vector2<f32>> {
        if let Self::Vector2(vector2) = self {
            Some(vector2)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_creating_a_vector2_component() {
        let mut location = Component::create_vector_2(50.0, 25.0);

        let vector2 = location.get_vector_2().unwrap();
        assert!(vector2.x == 50.0 && vector2.y == 25.0);
    }
}
