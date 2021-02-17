use self::point::Point;

pub mod point;

pub enum Component {
    Point(Point),
}

impl Component {
    #[allow(irrefutable_let_patterns)]
    pub fn cast_point(&self) -> &Point {
        if let Self::Point(point) = self {
            point
        } else {
            panic!("Tried to cast Component to something not a vector2");
        }
    }

    #[allow(irrefutable_let_patterns)]
    pub fn cast_point_mut(&mut self) -> &mut Point {
        if let Self::Point(point) = self {
            point
        } else {
            panic!("Tried to cast Component to something not a vector2");
        }
    }
}
