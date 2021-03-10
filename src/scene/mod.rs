use crate::common::{Color, Vec3, VertexFormat};
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
    lights: Vec<Box<dyn LightSource<T>>>,
}

impl<T: VertexFormat> Scene<T> {
    pub fn new(camera: Camera<T>, ambient_color: Color<T>) -> Scene<T> {
        Scene {
            camera,
            visibles: Vec::new(),
            ambient_color,
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
}
