use ggez::graphics::{Color, Mesh};

use crate::data_types::point::Point;

pub enum Resource {
    Color(Color),
    Mesh(Mesh),
    Point(Point),
    U32(u32),
    F32(f32),
    Usize(usize),
}

impl Resource {
    pub fn cast_color(&self) -> &Color {
        if let Self::Color(color) = self {
            color
        } else {
            panic!(self.create_error_message("color"));
        }
    }

    pub fn cast_color_mut(&mut self) -> &mut Color {
        if let Self::Color(color) = self {
            color
        } else {
            panic!(self.create_error_message("color"));
        }
    }

    pub fn cast_mesh(&self) -> &Mesh {
        if let Self::Mesh(mesh) = self {
            mesh
        } else {
            panic!(self.create_error_message("mesh"));
        }
    }

    pub fn cast_mesh_mut(&mut self) -> &mut Mesh {
        if let Self::Mesh(mesh) = self {
            mesh
        } else {
            panic!(self.create_error_message("mesh"));
        }
    }

    pub fn cast_point(&self) -> &Point {
        if let Self::Point(point) = self {
            point
        } else {
            panic!(self.create_error_message("point"));
        }
    }

    pub fn cast_point_mut(&mut self) -> &mut Point {
        if let Self::Point(point) = self {
            point
        } else {
            panic!(self.create_error_message("point"));
        }
    }

    fn create_error_message(&self, type_name: &str) -> String {
        format!("Cannot cast to type {} as it is another type", type_name)
    }

    pub fn cast_u32(&self) -> u32 {
        if let Self::U32(number) = self {
            *number
        } else {
            panic!(self.create_error_message("u32"))
        }
    }

    pub fn cast_f32(&self) -> f32 {
        if let Self::F32(number) = self {
            *number
        } else {
            panic!(self.create_error_message("f32"))
        }
    }

    pub fn cast_usize(&self) -> usize {
        if let Self::Usize(number) = self {
            *number
        } else {
            panic!(self.create_error_message("usize"))
        }
    }

    pub fn cast_usize_mut(&mut self) -> &mut usize {
        if let Self::Usize(number) = self {
            number
        } else {
            panic!(self.create_error_message("usize"))
        }
    }
}
