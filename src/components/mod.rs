pub mod helpers;

use std::cell::RefCell;
use std::rc::Rc;

use eyre::{bail, Result};
use ggez::audio::SoundData;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::data_types::point::Point;

macro_rules! impl_component_data_cast {
    ($new_type:ty, $arm:ident) => {
        impl CastComponents<$new_type> for ComponentData {
            fn cast(&self) -> Result<&Rc<RefCell<$new_type>>> {
                if let ComponentData::$arm(value) = self {
                    Ok(value)
                } else {
                    bail!(
                        "Error casting component from {:?} to {}",
                        self,
                        stringify!($new_type)
                    )
                }
            }
        }
    };
}

pub trait CastComponents<T> {
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

impl_component_data_cast!(Point, Point);
impl_component_data_cast!(f32, F32);
impl_component_data_cast!(Color, Color);
impl_component_data_cast!(Mesh, Mesh);
impl_component_data_cast!(u32, U32);
impl_component_data_cast!(usize, Usize);
impl_component_data_cast!(bool, Bool);
impl_component_data_cast!(KeyCode, GgezKeyCode);
impl_component_data_cast!(String, Marker);
impl_component_data_cast!(Text, GgezText);
impl_component_data_cast!(SoundData, GgezSound);

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
