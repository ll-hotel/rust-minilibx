include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" {
    pub fn free(ptr: *mut std::ffi::c_void);
}

mod src;
