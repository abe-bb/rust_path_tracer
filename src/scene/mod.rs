use crate::common::VertexFormat;
use crate::scene::camera::Camera;

mod camera;
mod visible;

struct Scene<T: VertexFormat> {
    camera: Camera<T>,
}
