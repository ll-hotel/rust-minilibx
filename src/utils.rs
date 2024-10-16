use crate::mlx::*;

pub enum MlxError {
    MlxCreationError,
    WindowCreationError,
    ImageCreationError,
}

pub struct Mlx {
    pub mlx_ptr: VoidPtr,
    pub win_ptr: VoidPtr,
    image_vec: Vec<Box<MlxImage>>,
}

impl Mlx {
    pub fn new() -> Self {
        Self {
            mlx_ptr: std::ptr::null_mut(),
            win_ptr: std::ptr::null_mut(),
            image_vec: vec![],
        }
    }
    pub fn init(&mut self) -> Result<(), MlxError> {
        self.mlx_ptr = mlx_init();
        if self.mlx_ptr.is_null() {
            return Err(MlxError::MlxCreationError);
        }
        self.win_ptr = mlx_init();
        if self.win_ptr.is_null() {
            return Err(MlxError::WindowCreationError);
        }
        return Ok(());
    }
    pub fn new_image(&mut self, width: i32, height: i32) -> Result<&mut MlxImage, MlxError> {
        let image = Box::new(MlxImage::new(self.mlx_ptr, width, height)?);
        self.image_vec.push(image);
        return Ok(self
            .image_vec
            .last_mut()
            .expect("Vector should be non-empty"));
    }
}
impl Drop for Mlx {
    fn drop(&mut self) {
        for image in &self.image_vec {
            mlx_destroy_image(self.mlx_ptr, image.img_ptr);
        }
        mlx_destroy_window(self.mlx_ptr, self.win_ptr);
        mlx_destroy_display(self.mlx_ptr);
    }
}

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
    pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        let offset = ((x + y * self.size_line) * self.bits_per_pixel / 8) as usize;
        if offset < self.pixel_count {
            unsafe {
                *self.data_addr.add(offset) = color;
            }
        }
    }
}
