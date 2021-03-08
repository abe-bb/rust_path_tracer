use crate::common::{self, Color, Intersection, Vec3, VertexFormat};
use crate::scene::visible::light::LightSource;
use ::std::cmp;

pub struct Material<T: VertexFormat> {
    diffuse_coefficient: T,
    diffuse_color: Color<T>,
    ambient_coefficient: T,
    specular_coefficient: T,
    phong_exponent: T,
    specular_color: Color<T>,
}

impl<T: VertexFormat> Material<T> {
    pub fn new(
        diffuse_coefficient: T,
        diffuse_color: Color<T>,
        ambient_coefficient: T,
        specular_coefficient: T,
        phong_exponent: T,
        specular_color: Color<T>,
    ) -> Self {
        Material {
            diffuse_coefficient,
            diffuse_color,
            ambient_coefficient,
            specular_coefficient,
            phong_exponent,
            specular_color,
        }
    }

    fn ambient(&self, ambient_color: &Color<T>) -> Vec3<T> {
        ambient_color
            .color_vector()
            .scalar_mul(&self.diffuse_color.color_vector())
            .mul(self.ambient_coefficient)
    }

    fn diffuse(
        &self,
        intersection: &Intersection<T>,
        light_source: Box<dyn LightSource<T>>,
    ) -> Vec3<T> {
        // normalized vector from intersection point to light source
        let l = light_source.light_vector(&intersection.point);

        // diffuse color calculation
        light_source
            .color()
            .color_vector()
            .scalar_mul(&self.diffuse_color.color_vector())
            .mul(self.diffuse_coefficient)
            .mul(common::max(T::zero(), l.dot(&intersection.normal)))
    }

    fn specular(
        &self,
        intersection: &Intersection<T>,
        light_source: Box<dyn LightSource<T>>,
        viewpoint: Vec3<T>,
    ) -> Vec3<T> {
        let l = light_source.light_vector(&intersection.point);
        let r = intersection
            .normal
            .mul(T::from(2.0).unwrap())
            .mul(intersection.normal.dot(&l))
            .sub(&l);

        let v = viewpoint.sub(&intersection.point).normalize();

        self.specular_color
            .color_vector()
            .scalar_mul(light_source.color().color_vector())
            .mul(self.specular_coefficient)
            .mul(common::max(T::zero(), v.dot(&r).powf(self.phong_exponent)))
    }
}
