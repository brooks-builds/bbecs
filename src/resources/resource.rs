use ggez::graphics::{Color, Mesh};

pub enum Resource {
    Color(Color),
    Mesh(Mesh),
}

impl Resource {
    /// Cast to type Color. If the cast fails then we are going to panic
    pub fn cast_color(&self) -> &Color {
        if let Self::Color(color) = self {
            color
        } else {
            panic!("Cannot cast to type color as it is another type");
        }
    }

    /// Cast to type Mesh, if the cast fails then we are going to panic.
    pub fn cast_mesh(&self) -> &Mesh {
        if let Self::Mesh(mesh) = self {
            mesh
        } else {
            panic!("Cannot cast to type mesh as it is another type");
        }
    }
}
