use num;

mod common;
mod image;
mod scene;

// Splits and stores an image into renderable chunks
pub struct Image<T: num::Integer> {
    rows: usize,
    cols: usize,
    num_pixels: usize,
    sub_sections: Vec<SubImage<T>>,
    index: usize,
    ordered_indices: Vec<usize>,
}

impl<T: num::Integer> Image<T> {
    pub fn new(rows: usize, cols: usize) -> Image<T> {
        Image {
            rows,
            cols,
            num_pixels: rows * cols,
            sub_sections: Vec::new(),
            index: 0,
            ordered_indices: Vec::new(),
        }
    }
}

struct SubImage<T: num::Integer> {
    pos_row: usize,
    pos_col: usize,
    rows: usize,
    cols: usize,
    length: usize,
    pixel_width: u8,
    bitmap: Vec<T>,
}

impl<T: num::Integer> SubImage<T> {
    fn new(
        pos_row: usize,
        pos_col: usize,
        rows: usize,
        cols: usize,
        length: usize,
        pixel_width: u8,
    ) -> SubImage<T> {
        SubImage {
            pos_row,
            pos_col,
            rows,
            cols,
            length,
            pixel_width,
            bitmap: Vec::with_capacity(length),
        }
    }
}
