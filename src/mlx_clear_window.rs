use x11::xlib::{XClearWindow, XFlush};

pub fn mlx_clear_window(xvar: *mut t_xvar, win: *mut t_win_list) {
    XClearWindow(xvar.display, win.window);
    if xvar.do_flush {
        XFlush(xvar.display);
    }
}
