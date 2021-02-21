use std::ops::{AddAssign, Sub};

#[derive(Default, Debug, Clone, Copy)]
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

    /// Create a new Point that is perpendicular to the self pointing towards the left
    pub fn to_perpendicular_left(&self) -> Self {
        Self::new(self.y, -self.x)
    }

    pub fn to_perpendicular_right(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Multiply a scalar (single number) by the x and y mutably.
    pub fn multiply_scalar(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    /// Normalize the point, which makes the hypotenuse equal to 1.0
    pub fn normalize(&mut self) {
        let length = self.length();
        if length != 0.0 {
            self.x /= length;
            self.y /= length;
        }
    }

    /// Get the length of the Point
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}