use super::{MlxError, MlxImage, MlxWindow};
use crate::{mlx_init, mlx_destroy_display, mlx_put_image_to_window, VoidPtr};

pub type MlxWindowHandle<'a> = &'a Option<MlxWindow>;
pub type MlxImageHandle<'a> = &'a Option<MlxImage>;

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
        title: &str,
    ) -> Result<MlxWindowHandle, MlxError> {
        self.win_vec
            .push(Some(MlxWindow::new(self.mlx_ptr, width, height, title)?));
        return Ok(self.win_vec.last().unwrap());
    }
    pub fn destroy_window(&mut self, window_handle: MlxWindowHandle) {
        if window_handle.is_none() {
            return;
        }
        for i in 0..self.win_vec.len() {
            if &self.win_vec[i] == window_handle {
                self.win_vec[i].take().unwrap().destroy(self.mlx_ptr);
                break;
            }
        }
    }
    pub fn new_image(&mut self, width: i32, height: i32) -> Result<MlxImageHandle, MlxError> {
        self.image_vec
            .push(Some(MlxImage::new(self.mlx_ptr, width, height)?));
        return Ok(self.image_vec.last().unwrap());
    }
    pub fn destroy_image(&mut self, image_handle: MlxImageHandle) {
        if image_handle.is_none() {
            return;
        }
        for i in 0..self.image_vec.len() {
            if &self.image_vec[i] == image_handle {
                self.image_vec[i].take().unwrap().destroy(self.mlx_ptr);
                break;
            }
        }
    }
    pub fn put_image_to_window(&mut self, window: &MlxWindow, image: &MlxImage, x: i32, y: i32) {
        mlx_put_image_to_window(self.mlx_ptr, window.ptr(), image.ptr(), x, y);
    }
}

impl Drop for Mlx {
    fn drop(&mut self) {
        for image_h in &mut self.image_vec {
            if let Some(ref mut image) = image_h {
                image.destroy(self.mlx_ptr);
            }
        }
        for window_h in &mut self.win_vec {
            if let Some(ref mut window) = window_h {
                window.destroy(self.mlx_ptr);
            }
        }
        mlx_destroy_display(self.mlx_ptr);
        unsafe {
            crate::cmlx::free(self.mlx_ptr);
        };
    }
}
