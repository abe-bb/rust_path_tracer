use crate::common::{Intersection, Ray, Vec3, VertexFormat};
use crate::scene::visible::Visible;

pub struct Mesh<T: VertexFormat> {
    vertices: Vec<Vec3<T>>,
}

impl<T: VertexFormat> Mesh<T> {
    pub fn new_triangle(v1: Vec3<T>, v2: Vec3<T>, v3: Vec3<T>) -> Mesh<T> {
        Mesh {
            vertices: vec![v1, v2, v3],
        }
    }
}

impl<T: VertexFormat> Visible<T> for Mesh<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        None
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

        let mesh = Mesh::new_triangle(v1, v2, v3);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

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

        let mesh = Mesh::new_triangle(v1, v2, v3);

        let ray = Ray::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let exptected_intersection = Intersection {
            point: Vec3::new(0.0, 0.0, -1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };

        assert_eq!(None, mesh.intersect(&ray));
    }
}
