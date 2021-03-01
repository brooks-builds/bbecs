use crate::data_types::point::Point;

pub trait CastComponents<T> {
    fn cast_mut(&mut self) -> &mut Vec<T>;
}

pub enum Components {
    Point(Vec<Point>),
    F32(Vec<f32>),
}

// for each type of component, implement
// cast
// cast mut

impl CastComponents<Point> for Components {
    fn cast_mut(&mut self) -> &mut Vec<Point> {
        if let Components::Point(points) = self {
            points
        } else {
            panic!("These are not the points that you are looking for");
        }
    }
}

pub enum Component {
    Point,
    F32,
}
