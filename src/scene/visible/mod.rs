use crate::common::{Color, Intersection, Ray, Spacial, Vec3, VertexFormat};
use crate::scene::light::LightSource;
use crate::scene::visible::material::Material;

mod material;
mod mesh;
mod sphere;

pub trait Visible<T: VertexFormat>: Intersectable<T> {
    fn calculate_lighting(
        &self,
        intersection: &Intersection<T>,
        lights: &Vec<Box<dyn LightSource<T>>>,
        viewpoint: &Vec3<T>,
        ambient_color: &Color<T>,
    ) -> Color<T>;

    fn is_reflective(&self) -> bool;
}

pub trait Intersectable<T: VertexFormat>: Spacial<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>>;
}

struct Body<T: VertexFormat> {
    shape: Box<dyn Intersectable<T>>,
    material: Material<T>,
}

impl<T: VertexFormat> Spacial<T> for Body<T> {
    fn location(&self) -> &Vec3<T> {
        self.shape.location()
    }
}

impl<T: VertexFormat> Intersectable<T> for Body<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        self.intersect(ray)
    }
}

impl<T: VertexFormat> Visible<T> for Body<T> {
    fn calculate_lighting(
        &self,
        intersection: &Intersection<T>,
        lights: &Vec<Box<dyn LightSource<T>>>,
        viewpoint: &Vec3<T>,
        ambient_color: &Color<T>,
    ) -> Color<T> {
        unimplemented!()
    }

    fn is_reflective(&self) -> bool {
        self.material.is_reflective()
    }
}
