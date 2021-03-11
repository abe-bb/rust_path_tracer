use crate::common::{Color, VertexFormat};

pub struct Image<T: VertexFormat> {
    width: u32,
    height: u32,
    buffer: Vec<Color<T>>,
}

impl<T: VertexFormat> Image<T> {
    pub fn new(width: u32, height: u32) -> Image<T> {
        Image {
            width,
            height,
            buffer: Vec::with_capacity((width * height) as usize),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color<T>) {
        if x >= self.width || y >= self.height {
            panic!("Index out of bounds");
        }

        let index = (x * self.width + y) as usize;

        self.buffer[index] = color;
    }
}
