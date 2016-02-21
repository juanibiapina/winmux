extern crate libc;
extern crate x11;

use std::ptr::null;

use self::libc::{c_int, c_uint, c_long};
use self::x11::xlib;

use key_command::KeyCommand;

pub struct WindowSystem {
    pub display: *mut xlib::Display,
}

impl WindowSystem {
    pub fn new() -> WindowSystem {
        let display: *mut xlib::Display = unsafe { xlib::XOpenDisplay(null()) };

        if display.is_null() {
            panic!("Cannot open display");
        }

        WindowSystem {
            display: display,
        }
    }

    pub fn select_input(&self, event_mask: c_long) {
        unsafe {
            xlib::XSelectInput(self.display, xlib::XDefaultRootWindow(self.display), event_mask);
        }
    }

    pub fn grab_button(&self, button_number: u32, modifiers: c_uint) {
        unsafe {
            xlib::XGrabButton(self.display, button_number, modifiers as c_uint, xlib::XDefaultRootWindow(self.display), true as c_int, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
        }
    }

    pub fn grab_key(&self, key_command: &KeyCommand) {
        unsafe {
            xlib::XGrabKey(self.display, xlib::XKeysymToKeycode(self.display, key_command.get_keysym()) as c_int, key_command.get_mask(),
            xlib::XDefaultRootWindow(self.display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        }
    }
}
