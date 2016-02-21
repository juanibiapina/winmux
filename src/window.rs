extern crate x11;

use self::x11::xlib;

#[derive(Clone)]
pub struct Window {
    xwindow: xlib::Window,
}

#[derive(Clone)]
pub struct WindowChanges {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub sibling: Window,
    pub stack_mode: i32,
    pub mask: u32,
}

pub struct WindowAttributes {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Window {
    pub fn from(xwindow: xlib::Window) -> Window {
        Window {
            xwindow: xwindow,
        }
    }

    pub fn get_x_window(&self) -> xlib::Window {
        self.xwindow
    }
}
