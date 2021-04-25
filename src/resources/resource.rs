use eyre::{bail, Result};
use ggez::audio::SoundData;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::data_types::point::Point;

macro_rules! impl_resource_cast {
    ($new_type:ty, $arm:ident) => {
        impl ResourceCast<$new_type> for Resource {
            fn cast(&self) -> Result<&$new_type> {
                if let Resource::$arm(value) = self {
                    Ok(value)
                } else {
                    bail!(
                        "Error casting {} to resource of type {}",
                        format!("{:?}", self),
                        stringify!($new_type)
                    )
                }
            }

            fn cast_mut(&mut self) -> Result<&mut $new_type> {
                if let Resource::$arm(value) = self {
                    Ok(value)
                } else {
                    bail!(
                        "Error casting {} to resource of type {}",
                        format!("{:?}", self),
                        stringify!($new_type)
                    )
                }
            }
        }
    };
}

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
    Marker(String),
    GgezText(Text),
    GgezSound(ggez::audio::SoundData),
}

impl_resource_cast!(u32, U32);
impl_resource_cast!(Point, Point);
impl_resource_cast!(Color, Color);
impl_resource_cast!(Mesh, Mesh);
impl_resource_cast!(f32, F32);
impl_resource_cast!(usize, Usize);
impl_resource_cast!(bool, Bool);
impl_resource_cast!(KeyCode, GgezKeyCode);
impl_resource_cast!(String, Marker);
impl_resource_cast!(Text, GgezText);
impl_resource_cast!(SoundData, GgezSound);
