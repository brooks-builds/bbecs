use self::point::Point;

pub mod point;

pub enum Component {
    Point(Point),
    F32(f32),
}

impl Component {
    pub fn cast_point(&self) -> &Point {
        if let Self::Point(point) = self {
            point
        } else {
            panic!("Tried to cast Component to something not a vector2");
        }
    }

    pub fn cast_point_mut(&mut self) -> &mut Point {
        if let Self::Point(point) = self {
            point
        } else {
            panic!("Tried to cast Component to something not a vector2");
        }
    }

    pub fn cast_f32(&self) -> &f32 {
        if let Self::F32(number) = self {
            number
        } else {
            panic!("Tried to cast Component to something not a f32");
        }
    }

    pub fn cast_f32_mut(&mut self) -> &mut f32 {
        if let Self::F32(number) = self {
            number
        } else {
            panic!("Tried to cast Component to something not a f32");
        }
    }
}
