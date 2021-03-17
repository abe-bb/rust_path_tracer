use crate::common::{Intersection, Ray, Spacial, Vec3, VertexFormat};
use crate::scene::visible::{Intersectable, Visible};

pub struct Triangle<T: VertexFormat> {
    vertices: Vec<Vec3<T>>,
    // Triangle normal
    normal: Vec3<T>,
    // distance to origin
    d: T,
}

impl<T: VertexFormat> Triangle<T> {
    pub fn new(v1: Vec3<T>, v2: Vec3<T>, v3: Vec3<T>) -> Triangle<T> {
        let vector1 = v2.sub(&v1);
        let vector2 = v3.sub(&v1);

        let normal = vector1.cross(&vector2).normalize();
        let d = Vec3::new(T::zero(), T::zero(), T::zero())
            .sub(&v1)
            .dot(&normal)
            .abs();

        Triangle {
            vertices: vec![v1, v2, v3],
            normal,
            d,
        }
    }

    fn axis_to_drop(&self) -> u8 {
        if self.normal.x >= self.normal.y && self.normal.x >= self.normal.z {
            0
        } else if self.normal.y >= self.normal.x && self.normal.y >= self.normal.z {
            1
        } else {
            2
        }
    }

    fn project(&self, vector: &Vec3<T>, axis_to_drop: u8) -> (T, T) {
        if axis_to_drop == 0 {
            (vector.y, vector.z)
        } else if axis_to_drop == 1 {
            (vector.x, vector.z)
        } else {
            (vector.x, vector.y)
        }
    }

    fn projection_intersection(&self, plane_intersection: &Vec3<T>) -> bool {
        let axis_to_drop = self.axis_to_drop();
        let v1 = self.project(&self.vertices[0].sub(plane_intersection), axis_to_drop);
        let v2 = self.project(&self.vertices[1].sub(plane_intersection), axis_to_drop);
        let v3 = self.project(&self.vertices[2].sub(plane_intersection), axis_to_drop);

        let mut count = 0_u32;

        if v1.0 > T::zero() && v2.0 >= T::zero() {
            count += 1;
        }
        if v2.0 > T::zero() && v3.0 >= T::zero() {
            count += 1;
        }
        if v3.0 > T::zero() && v1.0 >= T::zero() {
            count += 1;
        }

        count % 2 == 1
    }
}

impl<T: VertexFormat> Intersectable<T> for Triangle<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        let dot = self.normal.dot(&ray.direction);

        if dot == T::zero() {
            return None;
        } else if dot > T::zero() {
            return None;
        }
        let t = -(self.normal.dot(&ray.origin) + self.d) / dot;
        if t <= T::zero() {
            return None;
        }

        let intersection_point = ray.origin.add(&ray.direction.mul(t));

        if self.projection_intersection(&intersection_point) {
            Some(Intersection {
                point: intersection_point,
                normal: self.normal.clone(),
            })
        } else {
            None
        }
    }
}

impl<T: VertexFormat> Spacial<T> for Triangle<T> {
    fn location(&self) -> &Vec3<T> {
        //     TODO: maybe change this to be the something else? like an object origin or something.
        &self.vertices.first().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_plane_intersection() {
        let v1 = Vec3::new(-1.0, -1.0, -1.0);
        let v2 = Vec3::new(1.0, -1.0, -1.0);
        let v3 = Vec3::new(0.0, 1.0, -1.0);

        let mesh = Triangle::new(v1, v2, v3);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));

        let exptected_intersection = Intersection {
            point: Vec3::new(0.0, 0.0, -1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };

        assert_eq!(exptected_intersection, mesh.intersect(&ray).unwrap());
    }

    #[test]
    fn ray_plane_miss() {
        let v1 = Vec3::new(-1.0, -1.0, -1.0);
        let v2 = Vec3::new(1.0, -1.0, -1.0);
        let v3 = Vec3::new(0.0, 1.0, -1.0);

        let mesh = Triangle::new(v1, v2, v3);

        let ray = Ray::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        assert_eq!(None, mesh.intersect(&ray));
    }
}
