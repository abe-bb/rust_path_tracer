use crate::common::{Color, Intersection, Vec3, VertexFormat};
use crate::scene::light::LightSource;

pub struct Material<T: VertexFormat> {
    diffuse_coefficient: T,
    diffuse_color: Color<T>,
    ambient_coefficient: T,
    ambient_color: Color<T>,
    specular_coefficient: T,
    phong_exponent: T,
    specular_color: Color<T>,
    reflective_coefficient: T,
}

impl<T: VertexFormat> Material<T> {
    pub fn new(
        diffuse_coefficient: T,
        diffuse_color: Color<T>,
        specular_coefficient: T,
        specular_color: Color<T>,
        phong_exponent: T,
        ambient_coefficient: T,
        ambient_color: Color<T>,
        reflective_coefficient: T,
    ) -> Self {
        Material {
            diffuse_coefficient,
            diffuse_color,
            ambient_coefficient,
            ambient_color,
            specular_coefficient,
            phong_exponent,
            specular_color,
            reflective_coefficient,
        }
    }

    fn ambient(&self) -> Vec3<T> {
        self.ambient_color
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
        let angle = T::zero().max(intersection.normal.dot(&l));

        println!("light vector: {:?}", l);
        println!("angle: {:?}", angle);

        // diffuse color calculation
        light_source
            .color()
            .color_vector()
            .scalar_mul(self.diffuse_color.color_vector())
            .mul(self.diffuse_coefficient)
            .mul(angle)
    }

    fn specular(
        &self,
        intersection: &Intersection<T>,
        light_source: Box<dyn LightSource<T>>,
        viewpoint: &Vec3<T>,
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
            .mul(T::zero().max(v.dot(&r)).powf(self.phong_exponent))
    }

    pub fn is_reflective(&self) -> bool {
        self.reflective_coefficient > T::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::light::PointLight;

    #[test]
    fn correct_ambient_light_calculation() {
        let material = Material::new(
            0.3,
            Color::new(1.0, 0.0, 0.0).unwrap(),
            0.3,
            Color::new(1.0, 0.1, 0.1).unwrap(),
            32.0,
            0.1,
            Color::new(0.2, 0.2, 0.2).unwrap(),
            0.0,
        );

        let expected_diffuse = Color::new(0.2 * 0.1, 0.0, 0.0).unwrap();

        assert_eq!(expected_diffuse.color_vector(), &material.ambient());
    }

    #[test]
    fn correct_diffuse_light_calculation() {
        let material = Material::new(
            0.3,
            Color::new(1.0, 0.0, 0.0).unwrap(),
            0.3,
            Color::new(1.0, 0.1, 0.1).unwrap(),
            32.0,
            0.1,
            Color::new(0.2, 0.2, 0.2).unwrap(),
            0.0,
        );

        let intersection = Intersection {
            point: Vec3::new(0.0, 0.0, 1.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
        };

        let light = Box::new(PointLight::new(
            Color::new(0.5, 0.5, 0.5).unwrap(),
            Vec3::new(0.0, 0.0, 0.0),
        ));

        // l dot n = 1
        // diffuse color * light color: (1.0, 0.0, 0.0) * (0.5, 0.5, 0.5) = (0.5, 0.0, 0.0)
        // (0.5, 0.0, 0.0) * 1 * 0.3 = (0.5 * 0.3, 0.0, 0.0)

        let expected_diffuse = Color::new(0.5 * 0.3, 0.0, 0.0).unwrap();

        assert_eq!(
            expected_diffuse.color_vector(),
            &material.diffuse(&intersection, light)
        );
    }

    #[test]
    fn correct_specular_light_calculation() {
        let material = Material::new(
            0.3,
            Color::new(1.0, 0.0, 0.0).unwrap(),
            0.3,
            Color::new(1.0, 0.1, 0.1).unwrap(),
            32.0,
            0.1,
            Color::new(0.2, 0.2, 0.2).unwrap(),
            0.0,
        );

        let intersection = Intersection {
            point: Vec3::new(0.0, 0.0, 1.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
        };

        let light = Box::new(PointLight::new(
            Color::new(0.5, 0.5, 0.5).unwrap(),
            Vec3::new(0.0, 0.0, 0.0),
        ));

        let viewpoint = Vec3::new(0.0, 0.0, 0.0);

        // L dot N = 1
        // R = (0.0, 0.0, -2.0)(1) - (0.0, 0.0, -1.0) = (0.0, 0.0, -1.0)
        // V = (0.0, 0.0, -1.0)
        // max(0.0, V dot R) = 1.0
        // 1.0 ^ phong_exponent = 1.0
        // 0.3 (0.5, 0.5, 0.5) (1.0, 0.1, 0.1)

        let expected_specular =
            Color::new(0.3 * 0.5 * 1.0, 0.3 * 0.5 * 0.1, 0.3 * 0.5 * 0.1).unwrap();

        assert_eq!(
            expected_specular.color_vector(),
            &material.specular(&intersection, light, &viewpoint)
        )
    }
}
