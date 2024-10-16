extern crate x11;
use crate::cmlx;
pub use x11::xlib::xEvent;

pub type VoidPtr = *mut ::std::os::raw::c_void;
type CFunPtr = unsafe extern "C" fn() -> i32;

pub fn mlx_init() -> VoidPtr {
    unsafe { cmlx::mlx_init() }
}

pub fn mlx_new_window(mlx_ptr: VoidPtr, size_x: i32, size_y: i32, title: &mut str) -> VoidPtr {
    unsafe { cmlx::mlx_new_window(mlx_ptr, size_x, size_y, title.as_mut_ptr() as *mut i8) }
}

pub fn mlx_clear_window(mlx_ptr: VoidPtr, win_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_clear_window(mlx_ptr, win_ptr);
    }
}

pub fn mlx_pixel_put(mlx_ptr: VoidPtr, win_ptr: VoidPtr, x: i32, y: i32, color: u32) {
    unsafe {
        cmlx::mlx_pixel_put(mlx_ptr, win_ptr, x, y, color as i32);
    }
}

pub fn mlx_new_image(mlx_ptr: VoidPtr, width: i32, height: i32) -> VoidPtr {
    unsafe { cmlx::mlx_new_image(mlx_ptr, width, height) }
}

pub fn mlx_get_data_addr(
    img_ptr: VoidPtr,
    bits_per_pixel: &mut i32,
    size_line: &mut i32,
    endian: &mut i32,
) -> *mut i8 {
    unsafe { cmlx::mlx_get_data_addr(img_ptr, bits_per_pixel, size_line, endian) }
}

pub fn mlx_put_image_to_window(
    mlx_ptr: VoidPtr,
    win_ptr: VoidPtr,
    img_ptr: VoidPtr,
    x: i32,
    y: i32,
) {
    unsafe {
        cmlx::mlx_put_image_to_window(mlx_ptr, win_ptr, img_ptr, x, y);
    }
}

pub fn mlx_get_color_value(mlx_ptr: VoidPtr, color: i32) -> i32 {
    unsafe { cmlx::mlx_get_color_value(mlx_ptr, color) }
}

pub fn mlx_mouse_hook(win_ptr: VoidPtr, funct_ptr: Option<CFunPtr>, param: VoidPtr) {
    unsafe {
        cmlx::mlx_mouse_hook(win_ptr, funct_ptr, param);
    }
}

pub fn mlx_key_hook(win_ptr: VoidPtr, funct_ptr: Option<CFunPtr>, param: VoidPtr) {
    unsafe {
        cmlx::mlx_key_hook(win_ptr, funct_ptr, param);
    }
}

pub fn mlx_expose_hook(win_ptr: VoidPtr, funct_ptr: Option<CFunPtr>, param: VoidPtr) {
    unsafe {
        cmlx::mlx_expose_hook(win_ptr, funct_ptr, param);
    }
}

pub fn mlx_loop_hook(win_ptr: VoidPtr, funct_ptr: Option<CFunPtr>, param: VoidPtr) {
    unsafe {
        cmlx::mlx_loop_hook(win_ptr, funct_ptr, param);
    }
}

pub fn mlx_loop(mlx_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_loop(mlx_ptr);
    }
}

pub fn mlx_loop_end(mlx_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_loop_end(mlx_ptr);
    }
}

pub fn mlx_string_put(
    mlx_ptr: VoidPtr,
    win_ptr: VoidPtr,
    x: i32,
    y: i32,
    color: u32,
    mut string: String,
) {
    unsafe {
        cmlx::mlx_string_put(
            mlx_ptr,
            win_ptr,
            x,
            y,
            color as i32,
            string.as_mut_ptr() as *mut i8,
        );
    }
}

pub fn mlx_set_font(mlx_ptr: VoidPtr, win_ptr: VoidPtr, mut name: String) {
    unsafe {
        cmlx::mlx_set_font(mlx_ptr, win_ptr, name.as_mut_ptr() as *mut i8);
    }
}

pub fn mlx_xpm_to_image(
    mlx_ptr: VoidPtr,
    mut xpm_data: Vec<*mut i8>,
    width: &mut i32,
    height: &mut i32,
) -> VoidPtr {
    unsafe { cmlx::mlx_xpm_to_image(mlx_ptr, xpm_data.as_mut_ptr(), width, height) }
}

pub fn mlx_xpm_file_to_image(
    mlx_ptr: VoidPtr,
    mut filename: String,
    width: &mut i32,
    height: &mut i32,
) -> VoidPtr {
    unsafe { cmlx::mlx_xpm_file_to_image(mlx_ptr, filename.as_mut_ptr() as *mut i8, width, height) }
}

pub fn mlx_destroy_window(mlx_ptr: VoidPtr, win_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_destroy_window(mlx_ptr, win_ptr);
    }
}

pub fn mlx_destroy_image(mlx_ptr: VoidPtr, img_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_destroy_image(mlx_ptr, img_ptr);
    }
}

pub fn mlx_destroy_display(mlx_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_destroy_display(mlx_ptr);
    }
}

pub fn mlx_hook(
    win_ptr: VoidPtr,
    x_event: xEvent,
    x_mask: u32,
    funct_ptr: Option<CFunPtr>,
    param: VoidPtr,
) {
    unsafe {
        cmlx::mlx_hook(win_ptr, x_event as i32, x_mask as i32, funct_ptr, param);
    }
}

pub fn mlx_do_key_autoreapeatoff(mlx_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_do_key_autorepeatoff(mlx_ptr);
    }
}

pub fn mlx_do_key_autoreapeaton(mlx_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_do_key_autorepeaton(mlx_ptr);
    }
}

pub fn mlx_do_sync(mlx_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_do_sync(mlx_ptr);
    }
}

pub fn mlx_mouse_get_pos(mlx_ptr: VoidPtr, win_ptr: VoidPtr, x: &mut i32, y: &mut i32) {
    unsafe {
        cmlx::mlx_mouse_get_pos(mlx_ptr, win_ptr, x, y);
    }
}

pub fn mlx_mouse_move(mlx_ptr: VoidPtr, win_ptr: VoidPtr, x: i32, y: i32) {
    unsafe {
        cmlx::mlx_mouse_move(mlx_ptr, win_ptr, x, y);
    }
}

pub fn mlx_mouse_hide(mlx_ptr: VoidPtr, win_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_mouse_hide(mlx_ptr, win_ptr);
    }
}

pub fn mlx_mouse_show(mlx_ptr: VoidPtr, win_ptr: VoidPtr) {
    unsafe {
        cmlx::mlx_mouse_show(mlx_ptr, win_ptr);
    }
}

pub fn mlx_get_screen_size(mlx_ptr: VoidPtr, size_x: &mut i32, size_y: &mut i32) {
    unsafe {
        cmlx::mlx_get_screen_size(mlx_ptr, size_x, size_y);
    }
}
