#![warn(dead_code)]

use std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe extern "C" {
    pub fn free(ptr: *const c_void);
}
