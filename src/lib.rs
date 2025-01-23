use std::ffi::c_void;

use mlx::{
    free, mlx_destroy_display, mlx_destroy_image, mlx_destroy_window, mlx_do_key_autorepeatoff,
    mlx_do_key_autorepeaton, mlx_do_sync, mlx_get_data_addr, mlx_hook, mlx_init, mlx_key_hook,
    mlx_loop, mlx_loop_end, mlx_loop_hook, mlx_mouse_get_pos, mlx_mouse_hide, mlx_mouse_hook,
    mlx_mouse_move, mlx_mouse_show, mlx_new_image, mlx_new_window, mlx_put_image_to_window,
};

pub struct Display {
    raw: *mut c_void,
}
impl Display {
    pub fn new() -> Result<Self, &'static str> {
        let Some(raw) = (unsafe { mlx_init().as_mut() }) else {
            return Err("mlx: can not create display");
        };
        Ok(Self { raw })
    }
    pub fn do_sync(&self) {
        unsafe { mlx_do_sync(self.raw) };
    }
    pub fn do_key_autorepeaton(&self) {
        unsafe { mlx_do_key_autorepeaton(self.raw) };
    }
    pub fn do_key_autorepeatoff(&self) {
        unsafe { mlx_do_key_autorepeatoff(self.raw) };
    }
    pub fn loop_start(&self) {
        unsafe { mlx_loop(self.raw) };
    }
    pub fn loop_end(&self) {
        unsafe { mlx_loop_end(self.raw) };
    }
    pub fn loop_hook<T>(&self, funct: fn(*mut T), param: *mut T) {
        unsafe {
            mlx_loop_hook(
                self.raw,
                Some(std::mem::transmute(funct)),
                param as *mut c_void,
            )
        };
    }
}
impl Drop for Display {
    fn drop(&mut self) {
        unsafe { mlx_destroy_display(self.raw) };
        unsafe { free(self.raw) };
    }
}

pub struct Window<'a> {
    display: &'a Display,
    raw: *mut c_void,
    width: u32,
    height: u32,
}
impl<'a> Window<'a> {
    pub fn new(
        display: &'a Display,
        width: u32,
        height: u32,
        title: &str,
    ) -> Result<Self, &'static str> {
        let Some(raw) = (unsafe {
            mlx_new_window(
                display.raw,
                width as i32,
                height as i32,
                title.as_ptr() as *mut i8,
            )
            .as_mut()
        }) else {
            return Err("mlx: can not create window");
        };
        unsafe {
            let funct_ptr: unsafe extern "C" fn() -> i32 = std::mem::transmute(mlx_loop_end as *const c_void);
            mlx_hook(raw, 17, 0, Some(funct_ptr), display.raw)
        };
        Ok(Self {
            display,
            raw,
            width,
            height,
        })
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn put_image(&self, image: &Image, x: u32, y: u32) {
        unsafe {
            mlx_put_image_to_window(self.display.raw, self.raw, image.raw, x as i32, y as i32)
        };
    }
    pub fn mouse_hide(&self) {
        unsafe { mlx_mouse_hide(self.display.raw, self.raw) };
    }
    pub fn mouse_show(&self) {
        unsafe { mlx_mouse_show(self.display.raw, self.raw) };
    }
    pub fn mouse_move(&self, x: u32, y: u32) {
        unsafe { mlx_mouse_move(self.display.raw, self.raw, x as i32, y as i32) };
    }
    pub fn mouse_get_pos(&self, x: &mut u32, y: &mut u32) {
        unsafe {
            mlx_mouse_get_pos(
                self.display.raw,
                self.raw,
                x as *mut u32 as *mut i32,
                y as *mut u32 as *mut i32,
            )
        };
    }
    pub fn mouse_hook<T>(&self, funct: fn(i32, i32, i32, *mut T), param: *mut T) {
        unsafe { mlx_mouse_hook(self.raw, std::mem::transmute(funct), param as *mut c_void) };
    }
    pub fn key_hook<T>(&self, funct: fn(i32, *mut T), param: *mut T) {
        unsafe { mlx_key_hook(self.raw, std::mem::transmute(funct), param as *mut c_void) };
    }
    pub fn hook<T>(&self, x_event: i32, funct: fn(i32, *mut T), param: *mut T) {
        unsafe {
            mlx_hook(
                self.raw,
                x_event,
                0,
                std::mem::transmute(funct),
                param as *mut c_void,
            )
        };
    }
}
impl<'a> Drop for Window<'a> {
    fn drop(&mut self) {
        unsafe { mlx_destroy_window(self.display.raw, self.raw) };
    }
}

pub struct Image<'a> {
    display: &'a Display,
    raw: *mut c_void,
    data_addr: *mut i8,
    width: u32,
    height: u32,
    bpp: u32,
    endian: u32,
}
impl<'a> Image<'a> {
    pub fn new(display: &'a Display, width: u32, height: u32) -> Result<Self, &'static str> {
        let Some(raw) =
            (unsafe { mlx_new_image(display.raw, width as i32, height as i32).as_mut() })
        else {
            return Err("mlx: can not create image");
        };
        let mut bpp = 0;
        let mut endian = 0;
        let mut size_line = 0;
        let data_addr = unsafe { mlx_get_data_addr(raw, &mut bpp, &mut size_line, &mut endian) };
        Ok(Self {
            display,
            raw,
            data_addr,
            width,
            height,
            bpp: bpp as u32,
            endian: endian as u32,
        })
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn put_pixel(&self, x: u32, y: u32, pixel: i32) {
        unsafe {
            *(self.data_addr.byte_add((x + y * self.width) as usize * 4) as *mut i32) = pixel
        };
    }
}
impl<'a> Drop for Image<'a> {
    fn drop(&mut self) {
        unsafe { mlx_destroy_image(self.display.raw, self.raw) };
    }
}
