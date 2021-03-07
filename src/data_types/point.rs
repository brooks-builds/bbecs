use std::iter::Sum;
use std::ops::{AddAssign, Div, Sub};

/// Point that stores a f32 x and y with methods for manipulating the point. Uses Vector math
/// for the methods
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates an array of the point with x being the first element, and y being the second.
    /// This is mostly useful for working with ggez points as they can cast from arrays.
    /// ```
    /// use bbecs::data_types::point::Point;
    /// let location = Point::new(15.0, 7.0);
    /// assert_eq!(location.to_array(), [15.0, 7.0]);
    /// ```
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    /// Adds another point to the one being called, mutating itself
    /// ```
    /// use bbecs::data_types::point::Point;
    /// let mut location = Point::new(0.0, 0.0);
    /// let velocity = Point::new(1.0, 2.0);
    /// location.add(&velocity);
    /// assert_eq!(location, Point::new(1.0, 2.0));
    /// ```
    pub fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    /// Create a new Point that is perpendicular to the self pointing towards the left
    /// ```
    /// use bbecs::data_types::point::Point;
    /// let velocity = Point::new(10.0, 25.0);
    /// let left_velocity = velocity.to_perpendicular_left();
    /// assert_eq!(left_velocity, Point::new(25.0, -10.0));
    /// ```
    pub fn to_perpendicular_left(&self) -> Self {
        Self::new(self.y, -self.x)
    }
    /// Create a new Point that is perpendicular to the self pointing towards the right
    /// ```
    /// use bbecs::data_types::point::Point;
    /// let velocity = Point::new(10.0, 25.0);
    /// let right_velocity = velocity.to_perpendicular_right();
    /// assert_eq!(right_velocity, Point::new(-25.0, 10.0));
    /// ```
    pub fn to_perpendicular_right(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Multiplies the x and y by a scalar (single f32) and mutates itself
    /// ```
    /// use bbecs::data_types::point::Point;
    /// let mut point = Point::new(1.0, 2.0);
    /// point.multiply_scalar(10.0);
    /// assert_eq!(point, Point::new(10.0, 20.0));
    /// ```
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

    pub fn distance_to(&self, other: &Self) -> f32 {
        let distance = *other - *self;
        distance.length()
    }

    pub fn rotation(&self) -> f32 {
        self.y.atan2(self.x)
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

impl Div<usize> for Point {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
        }
    }
}

impl Sum<Point> for Point {
    fn sum<I: Iterator<Item = Point>>(points: I) -> Self {
        points.fold(Self::new(0.0, 0.0), |mut summed_point, point| {
            summed_point.add(&point);
            summed_point
        })
    }
}
