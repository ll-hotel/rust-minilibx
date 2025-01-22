use std::ptr::null_mut;

use gethostname::gethostname;
use x11::{
    xlib::{
        AllocNone, Atom, Colormap, Display, False, TrueColor, Visual, VisualClassMask, VisualDepthMask, Window, XCloseDisplay, XCreateColormap, XDefaultColormap, XDefaultDepth, XDefaultRootWindow, XDefaultScreen, XDefaultVisual, XGetVisualInfo, XInternAtom, XOpenDisplay, XVisualInfo
    },
    xshm::{XShmPixmapFormat, XShmQueryVersion},
};

pub struct MlxVars {
    pub display: *mut Display,
    pub root: Window,
    screen: i32,
    depth: i32,
    pub visual: *mut Visual,
    pub cmap: Colormap,
    private_cmap: bool,

    use_xshm: bool,
    pshm_format: i32,
    pub do_flush: bool,
    decrgb: [i32; 6],
    pub wm_delete_window: Atom,
    wm_protocols: Atom,
    end_loop: bool,
    //win_list: *mut c_void,
    //int			(*loop_hook)();
    //void		*loop_param;
}
impl MlxVars {
    pub fn new() -> Option<Box<Self>> {
        let Some(display) = (unsafe { XOpenDisplay("\0".as_ptr() as *const i8).as_mut() }) else {
            return None;
        };
        let screen = unsafe { XDefaultScreen(display) };
        let root = unsafe { XDefaultRootWindow(display) };
        let mut cmap = unsafe { XDefaultColormap(display, screen) };
        let depth = unsafe { XDefaultDepth(display, screen) };

        let mut private_cmap = false;
        let Some(visual) = mlx_int_get_visual(display, screen, depth, &mut private_cmap) else {
            eprintln!("MinilibX Error : No TrueColor Visual available.");
            return None;
        };

        let do_flush = true;
        let wm_delete_window =
            unsafe { XInternAtom(display, "WM_DELETE_WINDOW\0".as_ptr() as *const i8, False) };
        let wm_protocols =
            unsafe { XInternAtom(display, "WM_PROTOCOLS\0".as_ptr() as *const i8, False) };

        let mut pshm_format = 0;
        let use_xshm = mlx_int_deal_shm(display, &mut pshm_format);

        if private_cmap {
            cmap = unsafe { XCreateColormap(display, root, visual, AllocNone) };
        }
        let decrgb = mlx_int_rgb_conversion(unsafe { visual.as_mut() }.unwrap());
        let end_loop = false;
        Some(Box::new(MlxVars {
            display,
            root,
            screen,
            depth,
            visual,
            cmap,
            private_cmap,
            use_xshm,
            pshm_format,
            do_flush,
            decrgb,
            wm_delete_window,
            wm_protocols,
            end_loop,
        }))
    }
}
impl Drop for MlxVars {
    fn drop(&mut self) {
        unsafe { XCloseDisplay(self.display) };
    }
}

fn mlx_int_get_visual(
    display: *mut Display,
    screen: i32,
    depth: i32,
    private_cmap: &mut bool,
) -> Option<*mut Visual> {
    let mut vi: *mut XVisualInfo = null_mut();
    let mut template: XVisualInfo = unsafe { std::mem::zeroed() };
    let mut visual = unsafe { XDefaultVisual(display, screen) };
    if (unsafe { *visual }).class == TrueColor {
        return Some(visual);
    }
    template.class = TrueColor;
    template.depth = depth;
    let mut nb_item: i32 = 0;
    vi = unsafe {
        XGetVisualInfo(
            display,
            VisualDepthMask | VisualClassMask,
            &mut template,
            &mut nb_item,
        )
    };
    if vi.is_null() {
        return None;
    }
    visual = (unsafe { *vi }).visual;
    *private_cmap = true;
    Some(visual)
}

fn mlx_int_deal_shm(display: *mut Display, pshm_format: &mut i32) -> bool {
    let mut bidon = 0;
    let mut use_pshm = 0;
    let mut use_xshm =
        unsafe { XShmQueryVersion(display, &mut bidon, &mut bidon, &mut use_pshm) } != 0;
    *pshm_format = if use_xshm && use_pshm != 0 {
        unsafe { XShmPixmapFormat(display) }
    } else {
        -1
    };
    let hostname = gethostname().to_string_lossy().to_string();
    let dpy = std::env::var("DISPLAY");
    if let Ok(dpy) = dpy {
        if dpy != hostname && dpy != "localhost" {
            *pshm_format = -1;
            use_xshm = false;
        }
    }
    use_xshm
}

fn mlx_int_rgb_conversion(visual: &mut Visual) -> [i32; 6] {
    let mut decrgb = [0; 6];

    while !(visual.red_mask % 2 == 1) {
        visual.red_mask >>= 1;
        decrgb[0] += 1;
    }
    while visual.red_mask % 2 == 1 {
        visual.red_mask >>= 1;
        decrgb[1] += 1;
    }
    while !(visual.green_mask % 2 == 1) {
        visual.green_mask >>= 1;
        decrgb[2] += 1;
    }
    while visual.green_mask % 2 == 1 {
        visual.green_mask >>= 1;
        decrgb[3] += 1;
    }
    while !(visual.blue_mask % 2 == 1) {
        visual.blue_mask >>= 1;
        decrgb[4] += 1;
    }
    while visual.blue_mask % 2 == 1 {
        visual.blue_mask >>= 1;
        decrgb[5] += 1;
    }
    decrgb
}
