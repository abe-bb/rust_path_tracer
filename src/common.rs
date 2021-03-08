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

    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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
    // Validates color representation, returning None if passed in color data is invalid
    pub fn new(red: T, green: T, blue: T) -> Option<Color<T>> {
        Color::from_color_vertex(Vec3::new(red, green, blue))
    }

    // Validates color representation, returning None if passed in vertex is invalid
    pub fn from_color_vertex(color: Vec3<T>) -> Option<Color<T>> {
        let upper_threshhold = T::one();
        let lower_threshhold = T::zero();
        if color.x > upper_threshhold || color.y > upper_threshhold || color.z > upper_threshhold {
            return None;
        } else if color.x < lower_threshhold
            || color.y < lower_threshhold
            || color.z < lower_threshhold
        {
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

    // Vector tests
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

    // Color tests
    #[test]
    fn valid_lower_boundary_color() {
        let color_vector = Vec3::new(0.0, 0.0, 0.0);
        let color = Color::from_color_vertex(color_vector.clone()).unwrap();

        assert_eq!(color.color_vector(), &color_vector)
    }

    #[test]
    fn valid_middle_color() {
        let color_vector = Vec3::new(0.5, 0.5, 0.5);
        let color = Color::from_color_vertex(color_vector.clone()).unwrap();

        assert_eq!(color.color_vector(), &color_vector)
    }

    #[test]
    fn valid_upper_boundary_color() {
        let color_vector = Vec3::new(1.0, 1.0, 1.0);
        let color = Color::from_color_vertex(color_vector.clone()).unwrap();

        assert_eq!(color.color_vector(), &color_vector)
    }

    #[test]
    #[should_panic]
    fn invalid_lower_boundary_color() {
        let color_vector = Vec3::new(-0.0000000001, -0.0000000001, -0.0000000001);
        let color = Color::from_color_vertex(color_vector.clone()).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_upper_boundary_color() {
        let color_vector = Vec3::new(1.0000000001, 1.0000000001, 1.0000000001);
        let color = Color::from_color_vertex(color_vector.clone()).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_lower_color() {
        let color_vector = Vec3::new(-2.0, -2.0, -2.0);
        let color = Color::from_color_vertex(color_vector.clone()).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_upper_color() {
        let color_vector = Vec3::new(2.0, 2.0, 2.0);
        let color = Color::from_color_vertex(color_vector.clone()).unwrap();
    }
}
