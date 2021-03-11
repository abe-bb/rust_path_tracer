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

        match nearest {
            Some((intersection, visible)) => {
                return visible.calculate_lighting(
                    &intersection,
                    &self.lights,
                    self.camera.location(),
                    &self.ambient_color,
                );
            }
            None => self.background_color.clone(),
        }
    }
}
