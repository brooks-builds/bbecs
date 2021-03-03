use crate::data_types::point::Point;

pub trait CastComponents<T> {
    fn cast_mut(&mut self) -> &mut Vec<T>;
    fn cast(&self) -> &Vec<T>;
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

    fn cast(&self) -> &Vec<Point> {
        if let Components::Point(points) = self {
            points
        } else {
            panic!("No points to be found here, there is something else");
        }
    }
}

impl CastComponents<f32> for Components {
    fn cast_mut(&mut self) -> &mut Vec<f32> {
        if let Components::F32(numbers) = self {
            numbers
        } else {
            panic!("These are not the f32s that you are looking for");
        }
    }

    fn cast(&self) -> &Vec<f32> {
        todo!()
    }
}

pub enum Component {
    Point,
    F32,
}
