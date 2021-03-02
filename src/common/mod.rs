mod sphere;

use num;

pub trait VertexComp: num::Float {}

pub trait Visible<T: VertexComp> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>>;
}

pub struct Vec3<T: VertexComp> {
    x: T,
    y: T,
    z: T,
}

impl<T: VertexComp> Vec3<T> {
    fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    fn dist_sqrd(&self) -> T {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    fn mul(&self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    fn sub(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn div(&self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }

    fn add(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn normalize(&mut self) {
        let dist = self.dist_sqrd();
        self.x = self.x / dist;
        self.y = self.y / dist;
        self.z = self.z / dist;
    }
}

pub struct Ray<T: VertexComp> {
    origin: Vec3<T>,
    direction: Vec3<T>,
}

impl<T: VertexComp> Ray<T> {
    fn new(origin: Vec3<T>, direction: Vec3<T>) -> Ray<T> {
        Ray { origin, direction }
    }
}

pub struct Intersection<T: VertexComp> {
    point: Vec3<T>,
    normal: Vec3<T>,
}
