use crate::common::{Intersection, Ray, VertexFormat};

mod light;
mod sphere;

pub trait Visible<T: VertexFormat> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>>;
}
