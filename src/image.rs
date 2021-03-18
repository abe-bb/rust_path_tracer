use crate::common::{Color, VertexFormat};
use std::slice::Iter;

pub struct Image<T: VertexFormat> {
    width: u32,
    height: u32,
    buffer: Vec<Color<T>>,
}

pub struct ImageIterator<'a, T: VertexFormat> {
    buffer_iter: Iter<'a, Color<T>>,
}

impl<'a, T: VertexFormat> Iterator for ImageIterator<'a, T> {
    type Item = &'a Color<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer_iter.next()
    }
}

impl<T: VertexFormat> Image<T> {
    pub fn new(width: u32, height: u32) -> Image<T> {
        Image {
            width,
            height,
            buffer: vec![
                Color::new(T::zero(), T::zero(), T::zero()).unwrap();
                (width * height) as usize
            ],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color<T>) {
        if x >= self.width || y >= self.height {
            panic!("Index out of bounds");
        }

        let y = self.height - y - 1;

        let index = (y * self.width + x) as usize;

        self.buffer[index] = color;
    }

    pub fn iter(&self) -> ImageIterator<T> {
        ImageIterator {
            buffer_iter: self.buffer.iter(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl<T: VertexFormat> Iterator for Image<T> {
    type Item = Color<T>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
