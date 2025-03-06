//! Core math implemenatation for angles between vectors

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point(pub f32, pub f32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn mag(&self) -> f32 {
        let Vector(x, y) = self;
        f32::sqrt((x * x) + (y * y))
    }

    pub fn angle_between(&self, other: &Self) -> f32 {
        let inner = self.dot(other) / (self.mag() * other.mag());
        f32::acos(inner)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LineSegment(pub Point, pub Point);

impl LineSegment {
    pub fn vector(self) -> Vector {
        let LineSegment(Point(x1, y1), Point(x2, y2)) = self;
        Vector(x2 - x1, y2 - y1)
    }
}
