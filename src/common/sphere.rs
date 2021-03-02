use crate::common::Intersection;
use crate::common::Ray;
use crate::common::Vec3;
use crate::common::VertexComp;
use crate::common::Visible;

pub struct Sphere<T: VertexComp> {
    center: Vec3<T>,
    radius: T,
}

impl<T: VertexComp> Sphere<T> {
    fn new(center: Vec3<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }
}

impl<T: VertexComp> Visible<T> for Sphere<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        let oc = self.center.sub(&ray.origin);
        let tca = ray.direction.dot(&oc);
        let d2 = oc.dist_sqrd();
        let inside_sphere = d2 < self.radius;

        // Ray does not intersect sphere
        if tca < T::zero() && !inside_sphere {
            return None;
        }

        let thc2 = (self.radius * self.radius) - (d2 * d2) + (tca * tca);
        // Sphere is behind the ray. Ray does not intersect sphere
        if thc2 < T::zero() {
            return None;
        }

        let t;
        if inside_sphere {
            t = tca + thc2.sqrt();
        } else {
            t = tca - thc2.sqrt();
        }

        Some(Intersection {
            point: ray.origin.add(&ray.direction.mul(t)),
            normal: ray.origin.sub(&self.center).div(self.radius),
        })
    }
}
