use std::{
    ffi::{c_char, c_void},
    mem::MaybeUninit,
    ptr::null_mut,
    rc::Rc,
};

use gethostname::gethostname;
use shm::ffi::{shmat, shmctl, shmdt, shmget, Ipc};
use x11::{
    xlib::{
        AllocNone, Atom, CWBackPixel, CWBorderPixel, CWColormap, CWEventMask, Colormap,
        CopyFromParent, Display, ExposureMask, False, GCForeground, GCFunction, GCPlaneMask,
        GXcopy, InputOutput, KeySym, LASTEvent, Pixmap, TrueColor, Visual, VisualClassMask,
        VisualDepthMask, Window, XCreateColormap, XCreateGC, XCreatePixmap, XCreateWindow,
        XDefaultColormap, XDefaultDepth, XDefaultRootWindow, XDefaultScreen, XDefaultVisual,
        XDestroyImage, XErrorEvent, XEvent, XFlush, XGCValues, XGetVisualInfo, XGetWMNormalHints,
        XImage, XInternAtom, XMapRaised, XOpenDisplay, XPutBackEvent, XSetErrorHandler,
        XSetWMNormalHints, XSetWMProtocols, XSetWindowAttributes, XSizeHints, XStoreName, XSync,
        XVisualInfo, XWindowEvent, ZPixmap, GC,
    },
    xshm::{
        XShmAttach, XShmCreateImage, XShmCreatePixmap, XShmPixmapFormat, XShmQueryVersion,
        XShmSegmentInfo,
    },
};

pub const MLX_TYPE_SHM_PIXMAP: i32 = 3;
pub const MLX_TYPE_SHM: i32 = 2;
pub const MLX_TYPE_XIMAGE: i32 = 1;
pub const MLX_MAX_EVENT: usize = LASTEvent as usize;

pub const ENV_DISPLAY: &str = "DISPLAY\0";
pub const LOCALHOST: &str = "localhost\0";
pub const ERR_NO_TRUECOLOR: &str = "Minilibx Error : No TrueColor Visual available.\0";
pub const WARN_SHM_ATTACH: &str = "Minilibx Warning : X server cannot attach shared memory.\0";

pub static mut MLX_X_ERROR: i32 = 0;

pub struct XpmCol {
    name: i32,
    col: i32,
}

pub struct ColName {
    name: &'static mut str,
    color: i32,
}

#[derive(Clone, Copy)]
pub struct EventList {
    mask: i32,
    hook: Option<fn(*mut c_void)>,
    param: *mut c_void,
}

pub struct WinList {
    window: Window,
    gc: GC,
    next: Option<Rc<WinList>>,
    mouse_hook: Option<fn(i32, i32, *mut c_void)>,
    mouse_param: *mut c_void,
    key_hook: Option<fn(KeySym, *mut c_void)>,
    key_param: *mut c_void,
    expose_hook: Option<fn()>,
    expose_param: (),
    hooks: [EventList; MLX_MAX_EVENT],
}

pub struct Img<'a> {
    image: &'a mut XImage,
    pix: Pixmap,
    gc: GC,
    size_line: u32,
    bpp: u32,
    width: u32,
    height: u32,
    r#type: i32,
    format: i32,
    data: *mut char,
    shm: XShmSegmentInfo,
}

pub struct XVar {
    display: *mut Display,
    root: Window,
    screen: i32,
    depth: u32,
    visual: *mut Visual,
    cmap: Colormap,
    private_cmap: bool,
    win_list: Option<Rc<WinList>>,
    loop_hook: Option<fn(*mut c_void)>,
    loop_param: *mut c_void,
    use_xshm: bool,
    pshm_format: i32,
    do_flush: bool,
    decrgb: [i32; 6],
    wm_delete_window: Atom,
    wm_protocols: Atom,
    end_loop: bool,
}

pub fn mlx_int_do_nothing() {
    todo!()
}

pub fn mlx_get_color_value() {
    todo!()
}

pub fn mlx_int_get_good_color() {
    todo!()
}

pub fn mlx_int_find_in_pcm() {
    todo!()
}

pub fn mlx_int_anti_resize_win(xvar: &mut XVar, window: u64, width: i32, height: i32) {
    let mut hints: XSizeHints = unsafe { std::mem::zeroed() };
    let mut tmp: i64 = 0;
    unsafe {
        XGetWMNormalHints(xvar.display, window, &mut hints, &mut tmp);
    }
    hints.width = width;
    hints.min_width = width;
    hints.max_width = width;
    hints.height = height;
    hints.min_height = height;
    hints.max_height = height;
    unsafe {
        XSetWMNormalHints(xvar.display, window, &mut hints);
    }
}

pub fn mlx_int_wait_first_expose(xvar: &mut XVar, window: u64) {
    let mut event: XEvent = unsafe { std::mem::zeroed() };
    unsafe {
        XWindowEvent(xvar.display, window, ExposureMask, &mut event);
        XPutBackEvent(xvar.display, &mut event);
    }
}

pub fn mlx_int_rgb_conversion(visual: &mut Visual, decrgb: &mut [i32; 6]) {
    while (visual.red_mask & 1) == 0 {
        visual.red_mask >>= 1;
        decrgb[0] += 1;
    }
    while (visual.red_mask & 1) == 1 {
        visual.red_mask >>= 1;
        decrgb[1] += 1;
    }
    while (visual.green_mask & 1) == 0 {
        visual.green_mask >>= 1;
        decrgb[2] += 1;
    }
    while (visual.green_mask & 1) == 1 {
        visual.green_mask >>= 1;
        decrgb[3] += 1;
    }
    while (visual.blue_mask & 1) == 0 {
        visual.blue_mask >>= 1;
        decrgb[4] += 1;
    }
    while (visual.blue_mask & 1) == 1 {
        visual.blue_mask >>= 1;
        decrgb[5] += 1;
    }
}

pub fn mlx_int_deal_shm(
    display: *mut Display,
    use_xshm: &mut bool,
    use_pshm: &mut bool,
    pshm_format: &mut i32,
) {
    let mut tmp0 = 0;
    let mut tmp1 = 0;
    let mut pshm = 0;
    *use_xshm = unsafe { XShmQueryVersion(display, &mut tmp0, &mut tmp1, &mut pshm) != 0 };
    *use_pshm = pshm != 0;
    *pshm_format = if *use_xshm && *use_pshm {
        unsafe { XShmPixmapFormat(display) }
    } else {
        -1
    };
    let hostname = gethostname();
    if let Ok(dpy) = std::env::var("DISPLAY") {
        if *dpy != *hostname && dpy != LOCALHOST {
            *pshm_format = -1;
            *use_xshm = false;
        }
    }
}

pub fn mlx_int_new_xshmimage(
    xvar: &mut XVar,
    width: u32,
    height: u32,
    format: i32,
) -> Option<Box<Img>> {
    let mut img = Box::<Img>::new(unsafe { std::mem::zeroed() });
    let save_handler: Option<unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> i32>;

    img.data = null_mut();
    let Some(image) = (unsafe {
        XShmCreateImage(
            xvar.display,
            xvar.visual,
            xvar.depth as u32,
            format,
            img.data as *mut i8,
            &mut img.shm,
            width,
            height,
        )
        .as_mut()
    }) else {
        return None;
    };
    img.image = image;
    img.width = width;
    img.height = height;
    img.size_line = img.image.bytes_per_line as u32;
    img.bpp = img.image.bits_per_pixel as u32;
    img.format = format;
    img.shm.shmid = unsafe {
        shmget(
            0,
            ((width + 32) * height * 4) as u64,
            Ipc::CREAT as i32 | 0777,
        )
    };
    if img.shm.shmid == -1 {
        unsafe { XDestroyImage(img.image) };
        return None;
    }
    img.image.data = unsafe { shmat(img.shm.shmid, null_mut(), 0) } as *mut i8;
    img.shm.shmaddr = img.image.data;
    img.data = img.shm.shmaddr as *mut char;
    if img.data as usize == usize::MAX {
        unsafe {
            shmctl(img.shm.shmid, Ipc::RMID as i32, null_mut());
            XDestroyImage(img.image);
        }
        return None;
    }
    img.shm.readOnly = False;
    unsafe {
        MLX_X_ERROR = 0;
        save_handler = XSetErrorHandler(Some(std::mem::transmute(shm_att_pb as usize)));
        if XShmAttach(xvar.display, &mut img.shm) == 0
            || (0 & XSync(xvar.display, False)) != 0
            || MLX_X_ERROR != 0
        {
            XSetErrorHandler(save_handler);
            shmdt(img.data as *mut i32);
            shmctl(img.shm.shmid, Ipc::RMID as i32, null_mut());
            XDestroyImage(img.image);
            return None;
        }
        XSetErrorHandler(save_handler);
        shmctl(img.shm.shmid, Ipc::RMID as i32, null_mut());
        if xvar.pshm_format == format {
            img.pix = XShmCreatePixmap(
                xvar.display,
                xvar.root,
                img.shm.shmaddr,
                &mut img.shm,
                width,
                height,
                xvar.depth,
            );
            img.r#type = MLX_TYPE_SHM_PIXMAP;
        } else {
            img.pix = XCreatePixmap(xvar.display, xvar.root, width, height, xvar.depth);
            img.r#type = MLX_TYPE_XIMAGE;
        }
        if xvar.do_flush {
            XFlush(xvar.display);
        }
    }
    Some(img)
}

pub fn mlx_int_str_to_wordtab() -> Vec<String> {
    todo!()
}

pub fn mlx_new_image(xvar: &mut XVar, width: u32, height: u32) -> Option<Box<Img>> {
    if xvar.use_xshm {
        mlx_int_new_xshmimage(xvar, width, height, ZPixmap)
    } else {
        mlx_int_new_image(xvar, width, height, ZPixmap)
    }
}

pub fn mlx_int_new_image(xvar: &mut XVar, width: u32, height: u32, format: i32) -> Option<Box<Img>> {
    None
}

pub fn shm_att_pb(display: *mut Display, event: *mut XErrorEvent) {
    todo!()
}

pub fn mlx_int_get_visual(
    display: *mut Display,
    screen: i32,
    private_cmap: &mut bool,
    depth: i32,
) -> *mut Visual {
    *private_cmap = false;
    let Some(visual) = (unsafe { XDefaultVisual(display, screen).as_mut() }) else {
        return null_mut();
    };
    if visual.class == TrueColor {
        return visual;
    }
    let Some(vi) = (unsafe {
        let mut template: MaybeUninit<XVisualInfo> = MaybeUninit::zeroed();
        template.assume_init_mut().class = TrueColor;
        template.assume_init_mut().depth = depth;
        let mut nb_item: i32 = 0;
        XGetVisualInfo(
            display,
            VisualDepthMask | VisualClassMask,
            template.assume_init_mut(),
            &mut nb_item,
        )
        .as_mut()
    }) else {
        return null_mut();
    };
    *private_cmap = true;
    return vi.visual;
}

pub fn mlx_int_set_win_event_mask(xvar: &mut XVar) {
    todo!()
}

pub fn mlx_int_str_str_cote(str: String, find: String, len: usize) {
    todo!()
}

pub fn mlx_int_str_str(str: String, find: String, len: usize) {
    todo!()
}

pub fn mlx_init() -> Option<Rc<XVar>> {
    let Some(display) = (unsafe { XOpenDisplay("\0".as_ptr() as *const i8).as_mut() }) else {
        return None;
    };
    let screen = unsafe { XDefaultScreen(display) };
    let root = unsafe { XDefaultRootWindow(display) };
    let mut cmap = unsafe { XDefaultColormap(display, screen) };
    let depth = unsafe { XDefaultDepth(display, screen) };
    let mut private_cmap = false;
    let Some(visual) =
        (unsafe { mlx_int_get_visual(display, screen, &mut private_cmap, depth).as_mut() })
    else {
        eprintln!("{ERR_NO_TRUECOLOR}");
        return None;
    };
    let win_list = None;
    let loop_hook = None;
    let loop_param = null_mut();
    let do_flush = true;
    let wm_delete_window = unsafe {
        XInternAtom(
            display,
            "WM_DELETE_WINDOW\0".as_ptr() as *const c_char,
            false as i32,
        )
    };
    let wm_protocols = unsafe {
        XInternAtom(
            display,
            "WM_PROTOCOLS\0".as_ptr() as *const c_char,
            false as i32,
        )
    };
    let mut use_xshm = false;
    let mut use_pshm = false;
    let mut pshm_format = 0;
    mlx_int_deal_shm(display, &mut use_xshm, &mut use_pshm, &mut pshm_format);
    if private_cmap {
        cmap = unsafe { XCreateColormap(display, root, visual, AllocNone) };
    }
    let mut decrgb = [0; 6];
    mlx_int_rgb_conversion(visual, &mut decrgb);
    let end_loop = false;

    Some(Rc::new(XVar {
        display,
        screen,
        root,
        cmap,
        depth: depth as u32,
        private_cmap,
        visual,
        win_list,
        loop_hook,
        loop_param,
        do_flush,
        wm_delete_window,
        wm_protocols,
        use_xshm,
        //use_pshm,
        pshm_format,
        decrgb,
        end_loop,
    }))
}

pub fn mlx_new_window(xvar: &mut XVar, width: u32, height: u32, title: &str) -> Rc<WinList> {
    let mut xswa = XSetWindowAttributes {
        background_pixel: 0,
        border_pixel: u64::MAX,
        colormap: xvar.cmap,
        event_mask: 0xffffff, // All events.
        background_pixmap: 0,
        backing_pixel: 0,
        backing_planes: 0,
        backing_store: 0,
        bit_gravity: 0,
        border_pixmap: 0,
        cursor: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        save_under: 0,
        win_gravity: 0,
    };
    let window = unsafe {
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
    mlx_int_anti_resize_win(xvar, window, width as i32, height as i32);
    unsafe {
        XStoreName(xvar.display, window, title.as_ptr() as *const c_char);
        XSetWMProtocols(xvar.display, window, &mut xvar.wm_delete_window, 1);
    }
    let mut xgcv: XGCValues = unsafe { std::mem::zeroed() };
    xgcv.foreground = u64::MAX;
    xgcv.function = GXcopy;
    xgcv.plane_mask = u64::MAX;
    let new_win = Rc::<WinList>::new(WinList {
        window,
        gc: unsafe {
            XCreateGC(
                xvar.display,
                window,
                (GCFunction | GCPlaneMask | GCForeground) as u64,
                &mut xgcv,
            )
        },
        next: xvar.win_list.clone(),
        mouse_hook: None,
        mouse_param: null_mut(),
        key_hook: None,
        key_param: null_mut(),
        expose_hook: None,
        expose_param: (),
        hooks: [EventList {
            mask: 0,
            hook: None,
            param: null_mut(),
        }; MLX_MAX_EVENT],
    });
    xvar.win_list = Some(new_win.clone());
    unsafe {
        XMapRaised(xvar.display, window);
    }
    mlx_int_wait_first_expose(xvar, window);
    new_win
}
