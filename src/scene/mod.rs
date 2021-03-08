use crate::common::{Vec3, VertexFormat};
use crate::scene::camera::Camera;
use crate::scene::visible::Visible;

mod camera;
mod light;
mod visible;

struct Scene<T: VertexFormat> {
    camera: Camera<T>,
    visibles: Vec<Box<dyn Visible<T>>>,
    ambient_color: Vec3<T>,
}

impl<T: VertexFormat> Scene<T> {
    pub fn new(camera: Camera<T>) -> Scene<T> {
        Scene {
            camera,
            visibles: Vec::new(),
            ambient_color: Vec3::new(T::zero(), T::zero(), T::zero()),
        }
    }

    pub fn add_visible(&mut self, visible: Box<dyn Visible<T>>) {
        self.visibles.push(visible);
    }

    pub fn set_ambient_color(&mut self, ambient_color: Vec3<T>) {
        self.ambient_color = ambient_color;
    }
}
