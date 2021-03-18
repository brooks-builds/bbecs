use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::data_types::point::Point;
use crate::errors::BbEcsError;

pub trait CastComponents<T> {
    fn cast_mut(&mut self) -> Result<&mut Vec<T>>;
    fn cast(&self) -> Result<&Vec<T>>;
}

/// These components are used to store data into the world. Each of the components contains
/// a vector of the appropriate data. Generally consumers of this library will not need to
/// call Components directly. However the methods attached to components will be used to
/// extract the data.
#[derive(Debug, Clone)]
pub enum Components {
    Point(Vec<Point>),
    F32(Vec<f32>),
    Color(Vec<Color>),
    Mesh(Vec<Mesh>),
    U32(Vec<u32>),
    Usize(Vec<usize>),
    Bool(Vec<bool>),
    GgezKeyCode(Vec<KeyCode>),
    Marker(Vec<String>),
    GgezText(Vec<Text>),
}

impl Components {
    pub fn delete_by_index(&mut self, index: usize) -> Result<()> {
        match self {
            Components::Point(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::F32(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::Color(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::Mesh(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::U32(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::Usize(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::Bool(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::GgezKeyCode(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::Marker(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
            Components::GgezText(data) => {
                if index < data.len() {
                    data.remove(index);
                }
            }
        };
        Ok(())
    }
}

impl CastComponents<Point> for Components {
    /// Cast the components to it's contained data as long as it is really a point.
    /// ```
    /// use bbecs::components::{Components, CastComponents};
    /// use bbecs::data_types::point::Point;
    /// let mut wrapped_locations = Components::Point(vec![]);
    /// let locations: &mut Vec<Point> = wrapped_locations.cast_mut().unwrap();
    /// ```
    fn cast_mut(&mut self) -> Result<&mut Vec<Point>> {
        if let Components::Point(points) = self {
            Ok(points)
        } else {
            Err(BbEcsError::CastingComponents("Point").into())
        }
    }

    fn cast(&self) -> Result<&Vec<Point>> {
        if let Components::Point(points) = self {
            Ok(points)
        } else {
            Err(BbEcsError::CastingComponents("Point").into())
        }
    }
}

impl CastComponents<f32> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<f32>> {
        if let Components::F32(numbers) = self {
            Ok(numbers)
        } else {
            Err(BbEcsError::CastingComponents("F32").into())
        }
    }

    fn cast(&self) -> Result<&Vec<f32>> {
        if let Components::F32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("F32").into())
        }
    }
}

impl CastComponents<Color> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<Color>> {
        if let Components::Color(color) = self {
            Ok(color)
        } else {
            Err(BbEcsError::CastingComponents("Color").into())
        }
    }

    fn cast(&self) -> Result<&Vec<Color>> {
        if let Components::Color(color) = self {
            Ok(color)
        } else {
            Err(BbEcsError::CastingComponents("Color").into())
        }
    }
}

impl CastComponents<Mesh> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<Mesh>> {
        if let Components::Mesh(mesh) = self {
            Ok(mesh)
        } else {
            Err(BbEcsError::CastingComponents("Mesh").into())
        }
    }

    fn cast(&self) -> Result<&Vec<Mesh>> {
        if let Components::Mesh(mesh) = self {
            Ok(mesh)
        } else {
            Err(BbEcsError::CastingComponents("Mesh").into())
        }
    }
}

impl CastComponents<u32> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<u32>> {
        if let Components::U32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("U32").into())
        }
    }

    fn cast(&self) -> Result<&Vec<u32>> {
        if let Components::U32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("U32").into())
        }
    }
}

impl CastComponents<usize> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<usize>> {
        if let Components::Usize(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("Usize").into())
        }
    }

    fn cast(&self) -> Result<&Vec<usize>> {
        if let Components::Usize(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("Usize").into())
        }
    }
}

impl CastComponents<bool> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<bool>> {
        if let Components::Bool(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("Bool").into())
        }
    }

    fn cast(&self) -> Result<&Vec<bool>> {
        if let Components::Bool(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("Bool").into())
        }
    }
}

impl CastComponents<KeyCode> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<KeyCode>> {
        if let Components::GgezKeyCode(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("GgezKeyCode").into())
        }
    }

    fn cast(&self) -> Result<&Vec<KeyCode>> {
        if let Components::GgezKeyCode(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("GgezKeyCode").into())
        }
    }
}

impl CastComponents<String> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<String>> {
        if let Components::Marker(string) = self {
            Ok(string)
        } else {
            Err(BbEcsError::CastingComponents("Marker").into())
        }
    }

    fn cast(&self) -> Result<&Vec<String>> {
        if let Components::Marker(string) = self {
            Ok(string)
        } else {
            Err(BbEcsError::CastingComponents("Marker").into())
        }
    }
}

impl CastComponents<Text> for Components {
    fn cast_mut(&mut self) -> Result<&mut Vec<Text>> {
        if let Components::GgezText(text) = self {
            Ok(text)
        } else {
            Err(BbEcsError::CastingComponents("GgezText").into())
        }
    }

    fn cast(&self) -> Result<&Vec<Text>> {
        if let Components::GgezText(text) = self {
            Ok(text)
        } else {
            Err(BbEcsError::CastingComponents("GgezText").into())
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
    Bool,
    GgezKeyCode,
    Marker,
    GgezText,
}
