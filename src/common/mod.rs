use num;
use std::fmt::Debug;

// Traits defining needed operations for vectors
pub trait VertexFormat: num::Float + Debug {}

impl<T> VertexFormat for T where T: num::Float + Debug {}

#[derive(PartialEq, Debug, Clone)]
pub struct Vec3<T: VertexFormat> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VertexFormat> Vec3<T> {
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

    pub fn scalar_mul(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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

impl<T: VertexFormat> std::ops::Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Ray<T: VertexFormat> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T: VertexFormat> Ray<T> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Ray<T> {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
}

// Color struct. Stores colors using values from RGB from 0.0 to 1.0
pub struct Color<T: VertexFormat> {
    color: Vec3<T>,
}

impl<T: VertexFormat> Color<T> {
    // Validates color representation, returning None if passed in vertex is invalid
    pub fn new(color: Vec3<T>) -> Option<Color<T>> {
        let threshhold = T::one();
        if color.x > threshhold || color.y > threshhold || color.z > threshhold {
            return None;
        }

        Some(Color { color })
    }

    pub fn color_vector(&self) -> &Vec3<T> {
        &self.color
    }
}

#[derive(PartialEq, Debug)]
pub struct Intersection<T: VertexFormat> {
    pub point: Vec3<T>,
    pub normal: Vec3<T>,
}

pub fn max<T: VertexFormat>(v1: T, v2: T) -> T {
    if v1 > v2 {
        return v1;
    }

    v2
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn normalize_unit_vectors() {
        let unit_x = Vec3::new(-1.0, 0.0, 0.0);
        let expected = Vec3::new(-1.0, 0.0, 0.0);

        assert_eq!(unit_x.normalize(), expected);
    }

    #[test]
    fn normalize_vector() {
        let vector = Vec3::new(1.0, 1.0, 0.0);

        let expected = Vec3::new(1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt(), 0.0);

        let vector = vector.normalize();

        assert!(approx_eq!(f64, vector.x, expected.x, ulps = 1));
        assert!(approx_eq!(f64, vector.y, expected.y, ulps = 1));
        assert!(approx_eq!(f64, vector.z, expected.z, ulps = 1));
    }

    #[test]
    fn valid_lower_color() {
        let color_vector = Vec3::new(0.0, 0.0, 0.0);
        let color = Color::new(color_vector.clone()).unwrap();

        assert_eq!(color.color_vector(), &color_vector)
    }

    #[test]
    fn valid_middle_color() {
        let color_vector = Vec3::new(0.5, 0.5, 0.5);
        let color = Color::new(color_vector.clone()).unwrap();

        assert_eq!(color.color_vector(), &color_vector)
    }

    #[test]
    fn valid_upper_color() {
        let color_vector = Vec3::new(1.0, 1.0, 1.0);
        let color = Color::new(color_vector.clone()).unwrap();

        assert_eq!(color.color_vector(), &color_vector)
    }
}
