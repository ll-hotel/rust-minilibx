use x11::xlib::{
    CWBackPixel, CWBorderPixel, CWColormap, CWEventMask, CopyFromParent, Display, ExposureMask,
    GCForeground, GCFunction, GXcopy, InputOutput, Window, XCreateGC, XCreateWindow, XEvent,
    XGCValues, XGetWMNormalHints, XMapRaised, XPutBackEvent, XSetWMNormalHints, XSetWMProtocols,
    XSetWindowAttributes, XSizeHints, XStoreName, XWindowEvent, GC,
};
use x11::xlib::{XDestroyWindow, XFlush, XFreeGC};

use crate::MlxGeneralHook;
use crate::MlxMouseHook;
use crate::MlxVars;

pub struct MlxWindow<'a> {
    mlx_vars: &'a MlxVars,
    window: Window,
    gc: GC,
    mouse_hook: Option<MlxMouseHook>,
    key_hook: Option<MlxGeneralHook>,
    expose_hook: Option<MlxGeneralHook>,
    hooks: Vec<Option<MlxGeneralHook>>,
}

impl<'a> MlxWindow<'a> {
    pub fn new(xvar: &'a MlxVars, width: u32, height: u32, title: &str) -> Option<Box<Self>> {
        let mut xswa: XSetWindowAttributes = unsafe { std::mem::zeroed() };
        xswa.background_pixel = 0;
        xswa.border_pixel = u64::MAX;
        xswa.colormap = xvar.cmap;
        xswa.event_mask = 0xffffff; // all events.

        let window: Window = unsafe {
            XCreateWindow(
                xvar.display,
                xvar.root,
                0,
                0,
                width,
                height,
                0,
                CopyFromParent,
                InputOutput as u32,
                xvar.visual,
                CWEventMask | CWBackPixel | CWBorderPixel | CWColormap,
                &mut xswa,
            )
        };
        mlx_int_anti_resize_win(xvar.display, window, width as i32, height as i32);
        unsafe { XStoreName(xvar.display, window, title.as_ptr() as *const i8) };
        let mut wm_delete_window = xvar.wm_delete_window;
        unsafe { XSetWMProtocols(xvar.display, window, &mut wm_delete_window, 1) };

        let mut xgcv: XGCValues = unsafe { std::mem::zeroed() };
        xgcv.foreground = u64::MAX;
        xgcv.function = GXcopy;
        xgcv.plane_mask = u64::MAX;
        let gc = unsafe {
            XCreateGC(
                xvar.display,
                window,
                (GCFunction | GCForeground) as u64,
                &mut xgcv,
            )
        };
        let mut hooks = vec![];
        for _ in 0..36 {
            hooks.push(None);
        }
        unsafe { XMapRaised(xvar.display, window) };
        mlx_int_wait_first_expose(xvar.display, window);
        Some(Box::new(Self {
            mlx_vars: xvar,
            window,
            gc,
            mouse_hook: None,
            key_hook: None,
            expose_hook: None,
            hooks,
        }))
    }
}
impl Drop for MlxWindow<'_> {
    fn drop(&mut self) {
        unsafe { XDestroyWindow(self.mlx_vars.display, self.window) };
        unsafe { XFreeGC(self.mlx_vars.display, self.gc) };
        if self.mlx_vars.do_flush {
            unsafe { XFlush(self.mlx_vars.display) };
        }
    }
}

fn mlx_int_anti_resize_win(display: *mut Display, window: Window, width: i32, height: i32) {
    let mut hints: XSizeHints = unsafe { std::mem::zeroed() };
    let mut tmp = 0;
    unsafe { XGetWMNormalHints(display, window, &mut hints, &mut tmp) };
    hints.width = width;
    hints.height = height;
    hints.min_width = width;
    hints.min_height = height;
    hints.max_width = width;
    hints.max_height = height;
    unsafe { XSetWMNormalHints(display, window, &mut hints) };
}

fn mlx_int_wait_first_expose(display: *mut Display, window: Window) {
    let mut ev: XEvent = unsafe { std::mem::zeroed() };
    unsafe { XWindowEvent(display, window, ExposureMask, &mut ev) };
    unsafe { XPutBackEvent(display, &mut ev) };
}
