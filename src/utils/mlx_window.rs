use super::MlxError;
use crate::{mlx_destroy_window, mlx_new_window, VoidPtr};

#[derive(Eq, PartialEq)]
pub struct MlxWindow {
    pub width: i32,
    pub height: i32,
    win_ptr: VoidPtr,
}

impl MlxWindow {
    pub fn new(mlx_ptr: VoidPtr, width: i32, height: i32, title: &str) -> Result<Self, MlxError> {
        let win_ptr = mlx_new_window(mlx_ptr, width, height, title.to_string().as_mut_str());
        if win_ptr.is_null() {
            return Err(MlxError::WindowCreationError);
        }
        Ok(Self {
            width,
            height,
            win_ptr,
        })
    }
    pub fn destroy(&mut self, mlx_ptr: VoidPtr) {
        if self.win_ptr.is_null() || mlx_ptr.is_null() {
            return;
        }
        mlx_destroy_window(mlx_ptr, self.win_ptr);
        self.win_ptr = std::ptr::null_mut();
    }
    pub fn ptr(&self) -> VoidPtr {
        self.win_ptr
    }
}
