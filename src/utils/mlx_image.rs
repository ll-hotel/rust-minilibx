use super::MlxError;
use crate::{mlx_destroy_image, mlx_get_data_addr, mlx_new_image, VoidPtr};

#[derive(Eq, PartialEq)]
pub struct MlxImage {
    pub width: i32,
    pub height: i32,
    img_ptr: VoidPtr,
    data_addr: *mut u32,
    bits_per_pixel: i32,
    size_line: i32,
    endian: i32,
    pixel_count: usize,
}

impl MlxImage {
    pub fn new(mlx_ptr: VoidPtr, width: i32, height: i32) -> Result<Self, MlxError> {
        let img_ptr = mlx_new_image(mlx_ptr, width, height);
        if img_ptr.is_null() {
            return Err(MlxError::ImageCreationError);
        }
        let mut size_line = 0;
        let mut bits_per_pixel = 0;
        let mut endian = 0;
        let data_addr = mlx_get_data_addr(img_ptr, &mut bits_per_pixel, &mut size_line, &mut endian)
            as *mut u32;
        let pixel_count = (size_line * height) as usize;
        Ok(Self {
            width,
            height,
            img_ptr,
            data_addr,
            bits_per_pixel,
            size_line,
            endian,
            pixel_count,
        })
    }
    pub fn destroy(&mut self, mlx_ptr: VoidPtr) {
        if self.img_ptr.is_null() || mlx_ptr.is_null() {
            return;
        }
        mlx_destroy_image(mlx_ptr, self.img_ptr);
        self.img_ptr = std::ptr::null_mut();
    }
    pub fn ptr(&self) -> VoidPtr {
        self.img_ptr
    }
    pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        let offset = ((x + y * self.size_line) * self.bits_per_pixel / 8) as usize;
        if offset < self.pixel_count {
            unsafe {
                *self.data_addr.add(offset) = color;
            }
        }
    }
    pub fn put_rect(&mut self, top_left: (i32, i32), bot_right: (i32, i32), color: u32) {
        for x in top_left.0..bot_right.0 {
            for y in top_left.1..bot_right.1 {
                self.put_pixel(x, y, color);
            }
        }
    }
}
