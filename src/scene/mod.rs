use crate::common::{Color, Intersection, Ray, Spacial, Vec3, VertexFormat};
use crate::image::Image;
use crate::scene::camera::Camera;
use crate::scene::light::LightSource;
use crate::scene::visible::Visible;

mod camera;
mod light;
mod visible;

struct Scene<T: VertexFormat> {
    camera: Camera<T>,
    visibles: Vec<Box<dyn Visible<T>>>,
    ambient_color: Color<T>,
    background_color: Color<T>,
    lights: Vec<Box<dyn LightSource<T>>>,
}

impl<T: VertexFormat> Scene<T> {
    pub fn new(camera: Camera<T>, ambient_color: Color<T>, background_color: Color<T>) -> Scene<T> {
        Scene {
            camera,
            visibles: Vec::new(),
            ambient_color,
            background_color,
            lights: Vec::new(),
        }
    }

    // Mutable self borrows
    pub fn add_visible(&mut self, visible: Box<dyn Visible<T>>) {
        self.visibles.push(visible);
    }

    pub fn add_light(&mut self, light: Box<dyn LightSource<T>>) {
        self.lights.push(light);
    }

    pub fn set_ambient_color(&mut self, color: Color<T>) {
        self.ambient_color = color;
    }

    // immutable self borrows
    pub fn render(&self) -> Image<T> {
        Image::new(self.camera.width(), self.camera.height())
    }

    fn trace_ray(&self, ray: Ray<T>) -> Color<T> {
        let nearest = self.intersect(&ray);

        match nearest {
            Some((intersection, visible)) => {
                let visible_lights = self.visible_lights(&intersection);

                return visible.calculate_lighting(
                    &intersection,
                    &visible_lights,
                    self.camera.location(),
                );
            }
            None => self.background_color.clone(),
        }
    }

    // intersects a ray with every visible in the scene, returning the nearest intersection (and a
    // reference to the visible it belongs to)
    fn intersect(&self, ray: &Ray<T>) -> Option<(Intersection<T>, &Box<dyn Visible<T>>)> {
        let mut dist = T::zero();
        let mut nearest = None;

        for visible in &self.visibles {
            if let Some(i) = visible.intersect(&ray) {
                let distance = i.point.sub(&self.camera.location()).mag_sqrd();
                if distance < dist {
                    dist = distance;
                    nearest = Some((i, visible));
                }
            }
        }

        nearest
    }

    fn visible_lights(&self, intersection: &Intersection<T>) -> Vec<&Box<dyn LightSource<T>>> {
        let mut lights = Vec::new();
        for light in &self.lights {
            let light_vector = light.light_vector(&intersection.point);
            let ray = Ray::new(intersection.point.clone(), light_vector);

            let nearest_intersection = self.intersect(&ray);

            if let Some((intrsct, vis)) = nearest_intersection {
                let dist_to_light = light.location().sub(&intersection.point).mag_sqrd();
                let dist_to_vis = vis.location().sub(&intersection.point).mag_sqrd();

                if dist_to_light < dist_to_vis {
                    lights.push(light);
                }
            }
        }

        lights
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::light::PointLight;
    use crate::scene::visible::material::Material;
    use crate::scene::visible::sphere::Sphere;
    use crate::scene::visible::Body;

    #[test]
    fn test_single_traced_ray() {
        let ambiant = Color::new(0.1, 0.1, 0.1).unwrap();

        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0);
        let material = Material::new(
            0.6,
            Color::new(0.5, 0.5, 0.5).unwrap(),
            0.4,
            Color::new(0.6, 0.0, 0.0).unwrap(),
            32.0,
            0.3,
            ambiant.clone(),
            0.0,
        );
        let body = Body::new(Box::new(sphere), material);

        let visible = Box::new(body);
        let light_source = Box::new(PointLight::new(
            Color::new(1.0, 1.0, 1.0).unwrap(),
            Vec3::new(0.0, 3.0, 1.0),
        ));

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 8.0),
            Vec3::new(0.0, 1.0, 0.0),
            1920,
            1080,
            70.0_f64.to_radians(),
        );
        let background_color = Color::new(0.2, 0.2, 0.2).unwrap();

        let mut scene = Scene::new(camera, ambiant, background_color);

        scene.add_light(light_source);
        scene.add_visible(visible);

        let color = scene.trace_ray(scene.camera.ray(960.0, 540.0));

        println!("{:?}", color.color_vector());
    }
}
