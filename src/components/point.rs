#[derive(Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    pub fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}
