use crate::common::VertexFormat;
use crate::scene::camera::Camera;
use crate::scene::visible::Visible;

mod camera;
mod visible;

struct Scene<T: VertexFormat> {
    camera: Camera<T>,
    visibles: Vec<Box<dyn Visible<T>>>,
}

impl<T: VertexFormat> Scene<T> {
    pub fn new(camera: Camera<T>) -> Scene<T> {
        Scene {
            camera,
            visibles: Vec::new(),
        }
    }

    pub fn add_visible(&mut self, visible: Box<dyn Visible<T>>) {
        self.visibles.push(visible);
    }
}
