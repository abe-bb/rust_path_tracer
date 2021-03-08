use crate::common::{Color, Vec3, VertexFormat};

pub trait LightSource<T: VertexFormat> {
    fn set_color(&mut self, color: Color<T>);
    fn color(&self) -> &Color<T>;
    fn light_vector(&self, point: &Vec3<T>) -> Vec3<T>;
}

pub struct PointLight<T: VertexFormat> {
    color: Color<T>,
    position: Vec3<T>,
}

impl<T: VertexFormat> PointLight<T> {
    pub fn new(color: Color<T>, position: Vec3<T>) -> Self {
        PointLight { color, position }
    }

    pub fn set_position(&mut self, position: Vec3<T>) {
        self.position = position;
    }
}

impl<T: VertexFormat> LightSource<T> for PointLight<T> {
    fn set_color(&mut self, color: Color<T>) {
        self.color = color;
    }

    fn color(&self) -> &Color<T> {
        &self.color
    }

    fn light_vector(&self, point: &Vec3<T>) -> Vec3<T> {
        self.position.sub(point).normalize()
    }
}
