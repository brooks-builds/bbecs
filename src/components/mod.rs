use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::data_types::point::Point;
use crate::errors::BbEcsError;

pub trait CastComponents<T> {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<T>>>;
    fn cast(&self) -> Result<&Rc<RefCell<T>>>;
}

/// These components are used to store data into the world. Each of the components contains
/// a vector of the appropriate data. Generally consumers of this library will not need to
/// call Components directly. However the methods attached to components will be used to
/// extract the data.
#[derive(Debug, Clone)]
pub enum ComponentData {
    Point(Rc<RefCell<Point>>),
    F32(Rc<RefCell<f32>>),
    Color(Rc<RefCell<Color>>),
    Mesh(Rc<RefCell<Mesh>>),
    U32(Rc<RefCell<u32>>),
    Usize(Rc<RefCell<usize>>),
    Bool(Rc<RefCell<bool>>),
    GgezKeyCode(Rc<RefCell<KeyCode>>),
    Marker(Rc<RefCell<String>>),
    GgezText(Rc<RefCell<Text>>),
    GgezSound(Rc<RefCell<ggez::audio::SoundData>>),
}

impl CastComponents<Point> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<Point>>> {
        if let Self::Point(points) = self {
            Ok(points)
        } else {
            Err(BbEcsError::CastingComponents("Point").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<Point>>> {
        if let Self::Point(points) = self {
            Ok(points)
        } else {
            Err(BbEcsError::CastingComponents("Point").into())
        }
    }
}

impl CastComponents<f32> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<f32>>> {
        if let Self::F32(numbers) = self {
            Ok(numbers)
        } else {
            Err(BbEcsError::CastingComponents("F32").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<f32>>> {
        if let Self::F32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("F32").into())
        }
    }
}

impl CastComponents<Color> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<Color>>> {
        if let Self::Color(color) = self {
            Ok(color)
        } else {
            Err(BbEcsError::CastingComponents("Color").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<Color>>> {
        if let Self::Color(color) = self {
            Ok(color)
        } else {
            Err(BbEcsError::CastingComponents("Color").into())
        }
    }
}

impl CastComponents<Mesh> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<Mesh>>> {
        if let Self::Mesh(mesh) = self {
            Ok(mesh)
        } else {
            Err(BbEcsError::CastingComponents("Mesh").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<Mesh>>> {
        if let Self::Mesh(mesh) = self {
            Ok(mesh)
        } else {
            Err(BbEcsError::CastingComponents("Mesh").into())
        }
    }
}

impl CastComponents<u32> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<u32>>> {
        if let Self::U32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("U32").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<u32>>> {
        if let Self::U32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("U32").into())
        }
    }
}

impl CastComponents<usize> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<usize>>> {
        if let Self::Usize(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("Usize").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<usize>>> {
        if let Self::Usize(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("Usize").into())
        }
    }
}

impl CastComponents<bool> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<bool>>> {
        if let Self::Bool(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("Bool").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<bool>>> {
        if let Self::Bool(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("Bool").into())
        }
    }
}

impl CastComponents<KeyCode> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<KeyCode>>> {
        if let Self::GgezKeyCode(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("GgezKeyCode").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<KeyCode>>> {
        if let Self::GgezKeyCode(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("GgezKeyCode").into())
        }
    }
}

impl CastComponents<String> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<String>>> {
        if let Self::Marker(string) = self {
            Ok(string)
        } else {
            Err(BbEcsError::CastingComponents("Marker").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<String>>> {
        if let Self::Marker(string) = self {
            Ok(string)
        } else {
            Err(BbEcsError::CastingComponents("Marker").into())
        }
    }
}

impl CastComponents<Text> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<Text>>> {
        if let Self::GgezText(text) = self {
            Ok(text)
        } else {
            Err(BbEcsError::CastingComponents("GgezText").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<Text>>> {
        if let Self::GgezText(text) = self {
            Ok(text)
        } else {
            Err(BbEcsError::CastingComponents("GgezText").into())
        }
    }
}

impl CastComponents<ggez::audio::SoundData> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<ggez::audio::SoundData>>> {
        if let Self::GgezSound(data) = self {
            Ok(data)
        } else {
            Err(BbEcsError::CastingComponents("GgezSound").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<ggez::audio::SoundData>>> {
        if let Self::GgezSound(data) = self {
            Ok(data)
        } else {
            Err(BbEcsError::CastingComponents("GgezSound").into())
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
