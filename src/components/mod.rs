use ggez::graphics::{Color, Mesh};

use crate::data_types::point::Point;

pub trait CastComponents<T> {
    fn cast_mut(&mut self) -> &mut Vec<T>;
    fn cast(&self) -> &Vec<T>;
}

pub enum Components {
    Point(Vec<Point>),
    F32(Vec<f32>),
    Color(Vec<Color>),
    Mesh(Vec<Mesh>),
    U32(Vec<u32>),
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

impl CastComponents<Color> for Components {
    fn cast_mut(&mut self) -> &mut Vec<Color> {
        if let Components::Color(color) = self {
            color
        } else {
            panic!("No color here, it was something else");
        }
    }

    fn cast(&self) -> &Vec<Color> {
        if let Components::Color(color) = self {
            color
        } else {
            panic!("No color here, it was something else");
        }
    }
}

impl CastComponents<Mesh> for Components {
    fn cast_mut(&mut self) -> &mut Vec<Mesh> {
        if let Components::Mesh(mesh) = self {
            mesh
        } else {
            panic!("I am not a mesh, but you tried to cast me into one anyways");
        }
    }

    fn cast(&self) -> &Vec<Mesh> {
        if let Components::Mesh(mesh) = self {
            mesh
        } else {
            panic!("I am not a mesh, but you tried to cast me into one anyways");
        }
    }
}

impl CastComponents<u32> for Components {
    fn cast_mut(&mut self) -> &mut Vec<u32> {
        if let Components::U32(number) = self {
            number
        } else {
            panic!("Tried to cast to a u32");
        }
    }

    fn cast(&self) -> &Vec<u32> {
        if let Components::U32(number) = self {
            number
        } else {
            panic!("Tried to cast to a u32");
        }
    }
}

pub enum Component {
    Point,
    F32,
    Color,
    Mesh,
    U32,
}
