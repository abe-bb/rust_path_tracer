use crate::common::{Color, Intersection, Ray, Spacial, Vec3, VertexFormat};
use crate::scene::light::LightSource;
use crate::scene::visible::material::Material;

pub mod material;
pub mod mesh;
pub mod sphere;

pub trait Visible<T: VertexFormat>: Intersectable<T> {
    fn calculate_lighting(
        &self,
        intersection: &Intersection<T>,
        lights: &Vec<&Box<dyn LightSource<T>>>,
        viewpoint: &Vec3<T>,
    ) -> Color<T>;

    fn reflection_coefficient(&self) -> T;

    fn is_reflective(&self) -> bool;
}

pub trait Intersectable<T: VertexFormat>: Spacial<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>>;
}

pub struct Body<T: VertexFormat> {
    shape: Box<dyn Intersectable<T>>,
    material: Material<T>,
}

impl<T: VertexFormat> Body<T> {
    pub fn new(shape: Box<dyn Intersectable<T>>, material: Material<T>) -> Body<T> {
        Body { shape, material }
    }
}

impl<T: VertexFormat> Spacial<T> for Body<T> {
    fn location(&self) -> &Vec3<T> {
        self.shape.location()
    }
}

impl<T: VertexFormat> Intersectable<T> for Body<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        self.shape.intersect(ray)
    }
}

impl<T: VertexFormat> Visible<T> for Body<T> {
    fn calculate_lighting(
        &self,
        intersection: &Intersection<T>,
        lights: &Vec<&Box<dyn LightSource<T>>>,
        viewpoint: &Vec3<T>,
    ) -> Color<T> {
        let mut color = self.material.ambient();
        let reflective = self.material.is_reflective();

        for light in lights {
            let diffuse = self.material.diffuse(intersection, light);
            let specular = self.material.specular(intersection, light, viewpoint);

            color.mut_add(&diffuse);
            color.mut_add(&specular)
        }

        Color::clipped(color)
    }

    fn reflection_coefficient(&self) -> T {
        *self.material.reflective_coefficient()
    }

    fn is_reflective(&self) -> bool {
        self.material.is_reflective()
    }
}
