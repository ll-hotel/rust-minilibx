use crate::mlx::*;

#[derive(Clone, Copy, Debug)]
pub enum MlxError {
    MlxCreationError,
    WindowCreationError,
    ImageCreationError,
}
impl std::fmt::Display for MlxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MlxCreationError => write!(f, "Failed to create mlx"),
            Self::WindowCreationError => write!(f, "Faild to create new mlx window"),
            Self::ImageCreationError => write!(f, "Failed to create new mlx image"),
        }
    }
}

type MlxWindowHandle<'a> = &'a Option<MlxWindow>;
type MlxImageHandle<'a> = &'a Option<MlxImage>;

pub struct Mlx {
    mlx_ptr: VoidPtr,
    win_vec: Vec<Option<MlxWindow>>,
    image_vec: Vec<Option<MlxImage>>,
}

impl Mlx {
    pub fn new() -> Result<Self, MlxError> {
        let mlx_ptr = mlx_init();
        if mlx_ptr.is_null() {
            return Err(MlxError::MlxCreationError);
        }
        Ok(Self {
            mlx_ptr,
            win_vec: vec![],
            image_vec: vec![],
        })
    }
    pub fn new_window(
        &mut self,
        width: i32,
        height: i32,
        title: &mut str,
    ) -> Result<MlxWindowHandle, MlxError> {
        let win_ptr = mlx_new_window(self.mlx_ptr, width, height, title);
        if win_ptr.is_null() {
            return Err(MlxError::WindowCreationError);
        }
        self.win_vec.push(Some(MlxWindow { win_ptr }));
        return Ok(self.win_vec.last().unwrap());
    }
    pub fn destroy_window(&mut self, window_handle: MlxWindowHandle) {
        if window_handle.is_none() {
            return;
        }
        for i in 0..self.win_vec.len() {
            if &self.win_vec[i] == window_handle {
                mlx_destroy_window(self.mlx_ptr, self.win_vec[i].take().unwrap().win_ptr);
                break;
            }
        }
    }
    pub fn new_image(&mut self, width: i32, height: i32) -> Result<MlxImageHandle, MlxError> {
        let image = Some(MlxImage::new(self.mlx_ptr, width, height)?);

        self.image_vec.push(image);
        return Ok(self.image_vec.last().unwrap());
    }
    pub fn destroy_image(&mut self, image_handle: MlxImageHandle) {
        if image_handle.is_none() {
            return;
        }
        for i in 0..self.image_vec.len() {
            if &self.image_vec[i] == image_handle {
                mlx_destroy_window(self.mlx_ptr, self.win_vec[i].take().unwrap().win_ptr);
                break;
            }
        }
    }
}
impl Drop for Mlx {
    fn drop(&mut self) {
        for image_h in &self.image_vec {
            if let Some(image) = image_h {
                mlx_destroy_image(self.mlx_ptr, image.img_ptr);
            }
        }
        for window_h in &self.win_vec {
            if let Some(window) = window_h {
                mlx_destroy_window(self.mlx_ptr, window.win_ptr);
            }
        }
        mlx_destroy_display(self.mlx_ptr);
    }
}

#[derive(Eq, PartialEq)]
pub struct MlxWindow {
    win_ptr: VoidPtr,
}

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
    pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        let offset = ((x + y * self.size_line) * self.bits_per_pixel / 8) as usize;
        if offset < self.pixel_count {
            unsafe {
                *self.data_addr.add(offset) = color;
            }
        }
    }
}
