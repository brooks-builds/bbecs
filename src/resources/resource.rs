use ggez::graphics::{Color, Mesh};

use crate::components::point::Point;

pub enum Resource {
    Color(Color),
    Mesh(Mesh),
    Point(Point),
}

impl Resource {
    pub fn cast_color(&self) -> &Color {
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

    pub fn cast_point(&self) -> &Point {
        if let Self::Point(point) = self {
            point
        } else {
            panic!(self.create_error_message("point"));
        }
    }

    fn create_error_message(&self, type_name: &str) -> String {
        format!("Cannot cast to type {} as it is another type", type_name)
    }
}
