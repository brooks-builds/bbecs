use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh};

use crate::data_types::point::Point;
use crate::errors::BbEcsError;

pub trait ResourceCast<T> {
    fn cast(&self) -> Result<&T>;
    fn cast_mut(&mut self) -> Result<&mut T>;
}

#[derive(Debug, Clone)]
pub enum Resource {
    Color(Color),
    Mesh(Mesh),
    Point(Point),
    U32(u32),
    F32(f32),
    Usize(usize),
    Bool(bool),
    GgezKeyCode(KeyCode),
}

impl ResourceCast<u32> for Resource {
    fn cast(&self) -> Result<&u32> {
        if let Resource::U32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&u32",
            }
            .into())
        }
    }

    fn cast_mut(&mut self) -> Result<&mut u32> {
        if let Resource::U32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&mut u32",
            }
            .into())
        }
    }
}

impl ResourceCast<Point> for Resource {
    fn cast(&self) -> Result<&Point> {
        if let Resource::Point(point) = self {
            Ok(point)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&point",
            }
            .into())
        }
    }

    fn cast_mut(&mut self) -> Result<&mut Point> {
        if let Resource::Point(point) = self {
            Ok(point)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&mut point",
            }
            .into())
        }
    }
}

impl ResourceCast<Color> for Resource {
    fn cast(&self) -> Result<&Color> {
        if let Resource::Color(color) = self {
            Ok(color)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&color",
            }
            .into())
        }
    }

    fn cast_mut(&mut self) -> Result<&mut Color> {
        if let Resource::Color(color) = self {
            Ok(color)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&mut color",
            }
            .into())
        }
    }
}

impl ResourceCast<Mesh> for Resource {
    fn cast(&self) -> Result<&Mesh> {
        if let Resource::Mesh(mesh) = self {
            Ok(mesh)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&mesh",
            }
            .into())
        }
    }

    fn cast_mut(&mut self) -> Result<&mut Mesh> {
        if let Resource::Mesh(mesh) = self {
            Ok(mesh)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&mut mesh",
            }
            .into())
        }
    }
}

impl ResourceCast<f32> for Resource {
    fn cast(&self) -> Result<&f32> {
        if let Resource::F32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&f32",
            }
            .into())
        }
    }

    fn cast_mut(&mut self) -> Result<&mut f32> {
        if let Resource::F32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&mut f32",
            }
            .into())
        }
    }
}

impl ResourceCast<usize> for Resource {
    fn cast(&self) -> Result<&usize> {
        if let Resource::Usize(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&usize",
            }
            .into())
        }
    }

    fn cast_mut(&mut self) -> Result<&mut usize> {
        if let Resource::Usize(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "&mut usize",
            }
            .into())
        }
    }
}

impl ResourceCast<bool> for Resource {
    fn cast(&self) -> Result<&bool> {
        if let Resource::Bool(boolean) = self {
            Ok(boolean)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "boolean",
            }
            .into())
        }
    }

    fn cast_mut(&mut self) -> Result<&mut bool> {
        if let Resource::Bool(boolean) = self {
            Ok(boolean)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "boolean",
            }
            .into())
        }
    }
}

impl ResourceCast<KeyCode> for Resource {
    fn cast(&self) -> Result<&KeyCode> {
        if let Resource::GgezKeyCode(key_code) = self {
            Ok(key_code)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "key_code",
            }
            .into())
        }
    }

    fn cast_mut(&mut self) -> Result<&mut KeyCode> {
        if let Resource::GgezKeyCode(key_code) = self {
            Ok(key_code)
        } else {
            Err(BbEcsError::CastingResource {
                from: self.clone(),
                to: "key_code",
            }
            .into())
        }
    }
}
