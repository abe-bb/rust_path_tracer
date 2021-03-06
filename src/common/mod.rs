use num;
use std::fmt::Debug;

// Traits defining needed operations for vectors
pub trait VertexComp: num::Float + Debug {}
impl<T> VertexComp for T where T: num::Float + Debug {}

#[derive(PartialEq, Debug)]
pub struct Vec3<T: VertexComp> {
    x: T,
    y: T,
    z: T,
}

impl<T: VertexComp> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    // magnitude squared of the vector
    pub fn mag_sqrd(&self) -> T {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn mul(&self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    pub fn sub(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn div(&self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }

    pub fn add(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(mut self) -> Self {
        let dist = self.mag_sqrd().sqrt();
        self.x = self.x / dist;
        self.y = self.y / dist;
        self.z = self.z / dist;

        self
    }
}

impl<T: VertexComp> std::ops::Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
pub struct Ray<T: VertexComp> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T: VertexComp> Ray<T> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Ray<T> {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Intersection<T: VertexComp> {
    pub point: Vec3<T>,
    pub normal: Vec3<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_unit_vectors() {
        let unit_x = Vec3::new(-1.0, 0.0, 0.0);
        let expected = Vec3::new(-1.0, 0.0, 0.0);

        assert_eq!(unit_x.normalize(), expected);
    }

    #[test]
    fn normalize() {
        let vector = Vec3::new(1.0, 1.0, 0.0);

        let expected = Vec3::new(1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt(), 0.0);

        assert_eq!(expected, vector.normalize());
    }
}
