use eyre::Result;
use ggez::graphics::{Color, Mesh};

use crate::data_types::point::Point;
use crate::errors::BbEcsError;

pub trait CastComponents<T> {
    fn cast_mut(&mut self) -> Result<&mut Vec<T>>;
    fn cast(&self) -> Result<&Vec<T>>;
}

#[derive(Debug)]
pub enum Components {
    Point(Vec<Point>),
    F32(Vec<f32>),
    Color(Vec<Color>),
    Mesh(Vec<Mesh>),
    U32(Vec<u32>),
    Usize(Vec<usize>),
}

impl CastComponents<Point> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<Point>> {
        if let Components::Point(points) = self {
            Ok(points)
        } else {
            Err(BbEcsError::CastingComponents {
                from: "Components".to_owned(),
                to: "&mut Vec<Point>".to_owned(),
            }
            .into())
        }
    }

    fn cast(&self) -> Result<&Vec<Point>> {
        if let Components::Point(points) = self {
            Ok(points)
        } else {
            Err(BbEcsError::CastingComponents {
                from: "Components".to_owned(),
                to: "&mut Vec<Point>".to_owned(),
            }
            .into())
        }
    }
}

impl CastComponents<f32> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<f32>> {
        if let Components::F32(numbers) = self {
            numbers
        } else {
            panic!("These are not the f32s that you are looking for");
        }
    }

    fn cast(&self) -> Result<&Vec<f32>> {
        if let Components::F32(number) = self {
            number
        } else {
            panic!("Tried to cast a {:?} component to a &Vec<f32>", self);
        }
    }
}

impl CastComponents<Color> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<Color>> {
        if let Components::Color(color) = self {
            color
        } else {
            panic!("No color here, it was something else");
        }
    }

    fn cast(&self) -> Result<&Vec<Color>> {
        if let Components::Color(color) = self {
            color
        } else {
            panic!("No color here, it was something else");
        }
    }
}

impl CastComponents<Mesh> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<Mesh>> {
        if let Components::Mesh(mesh) = self {
            mesh
        } else {
            panic!("I am not a mesh, but you tried to cast me into one anyways");
        }
    }

    fn cast(&self) -> Result<&Vec<Mesh>> {
        if let Components::Mesh(mesh) = self {
            mesh
        } else {
            panic!("I am not a mesh, but you tried to cast me into one anyways");
        }
    }
}

impl CastComponents<u32> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<u32>> {
        if let Components::U32(number) = self {
            number
        } else {
            panic!("Tried to cast to a u32");
        }
    }

    fn cast(&self) -> Result<&Vec<u32>> {
        if let Components::U32(number) = self {
            number
        } else {
            panic!("Tried to cast to a u32");
        }
    }
}

impl CastComponents<usize> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<usize>> {
        if let Components::Usize(number) = self {
            number
        } else {
            panic!("tried to cast to a usize when not a usize");
        }
    }

    fn cast(&self) -> Result<&Vec<usize>> {
        if let Components::Usize(number) = self {
            number
        } else {
            panic!("tried to cast to a usize when not a usize");
        }
    }
}

pub enum Component {
    Point,
    F32,
    Color,
    Mesh,
    U32,
    Usize,
}
