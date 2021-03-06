use crate::common::{Intersection, Ray, VertexComp};

mod sphere;

pub trait Visible<T: VertexComp> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>>;
}
