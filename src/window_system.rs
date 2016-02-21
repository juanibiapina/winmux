extern crate libc;
extern crate x11;

use std::ptr::null;
use std::mem::zeroed;

use self::libc::{c_int, c_uint, c_long};
use self::x11::xlib;

use key_command::KeyCommand;
use mouse_command::MouseCommand;
use key_modifier::KeyModifier;
use event::Event;
use window::{Window, WindowChanges, WindowAttributes};

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

    pub fn grab_button(&self, mouse_command: &MouseCommand) {
        unsafe {
            xlib::XGrabButton(self.display, mouse_command.button_number, mouse_command.modifiers as c_uint, xlib::XDefaultRootWindow(self.display), true as c_int, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
        }
    }

    pub fn grab_key(&self, key_command: &KeyCommand) {
        unsafe {
            xlib::XGrabKey(self.display, xlib::XKeysymToKeycode(self.display, key_command.get_keysym()) as c_int, key_command.get_mask(),
            xlib::XDefaultRootWindow(self.display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        }
    }

    pub fn next_event(&self) -> Event {
        let mut xevent: xlib::XEvent = unsafe { zeroed() };

        unsafe {
            xlib::XNextEvent(self.display, &mut xevent);
        }

        match xevent.get_type() {
            xlib::KeyPress => {
                unsafe {
                    let xevent = xlib::XKeyEvent::from(xevent);

                    let keysym = xlib::XKeycodeToKeysym(self.display, xevent.keycode as u8, 0) as u64;
                    let keymodifier = KeyModifier::from_bits(0xEF & xevent.state as u32).unwrap();
                    let key_command = KeyCommand::new(keysym, keymodifier);

                    let subwindow = Window::from(xevent.subwindow);

                    Event::KeyPress(subwindow, key_command)
                }
            },
            xlib::MapRequest => {
                let xevent = xlib::XMapRequestEvent::from(xevent);
                Event::MapRequest(Window::from(xevent.window))
            },
            xlib::ConfigureRequest => {
                let xevent = xlib::XConfigureRequestEvent::from(xevent);
                let window = Window::from(xevent.window);
                let mask = xevent.value_mask as u32;
                let window_changes = WindowChanges {
                    x: xevent.x,
                    y: xevent.y,
                    width: xevent.width,
                    height: xevent.height,
                    border_width: xevent.border_width,
                    stack_mode: xevent.detail,
                    sibling: Window::from(xevent.above),
                    mask: mask,
                };

                Event::ConfigureRequest(window, window_changes)
            },
            xlib::ButtonPress => {
                let xevent = xlib::XButtonEvent::from(xevent);
                let mouse_command = MouseCommand::new(xevent.button, xevent.state);
                Event::ButtonPress(Window::from(xevent.subwindow), mouse_command, xevent.x_root, xevent.y_root)
            },
            xlib::ButtonRelease => {
                let xevent = xlib::XButtonEvent::from(xevent);
                let mouse_command = MouseCommand::new(xevent.button, xevent.state);
                Event::ButtonRelease(Window::from(xevent.subwindow), mouse_command, xevent.x_root, xevent.y_root)
            },
            xlib::MotionNotify => {
                let xevent = xlib::XButtonEvent::from(xevent);
                Event::MotionNotify(xevent.x_root, xevent.y_root)
            },
            _ => {
                Event::Unknown(xevent.get_type())
            }
        }
    }

    pub fn raise_window(&self, window: &Window) {
        unsafe {
            xlib::XRaiseWindow(self.display, window.get_x_window());
        }
    }

    pub fn map_window(&self, window: &Window) {
        unsafe {
            xlib::XMapWindow(self.display, window.get_x_window());
        }
    }

    pub fn configure_window(&self, window: &Window, window_changes: &WindowChanges) {
        let mut x_window_changes = xlib::XWindowChanges {
            x: window_changes.x,
            y: window_changes.y,
            width: window_changes.width,
            height: window_changes.height,
            border_width: window_changes.border_width,
            stack_mode: window_changes.stack_mode,
            sibling: window_changes.sibling.get_x_window(),
        };

        unsafe {
            xlib::XConfigureWindow(self.display, window.get_x_window(), window_changes.mask, &mut x_window_changes);
        }
    }

    pub fn get_window_attributes(&self, window: &Window) -> WindowAttributes {
        let mut attr: xlib::XWindowAttributes = unsafe { zeroed() };

        unsafe {
            xlib::XGetWindowAttributes(self.display, window.get_x_window(), &mut attr);
        }

        WindowAttributes {
            x: attr.x,
            y: attr.y,
            width: attr.width,
            height: attr.height,
        }
    }

    pub fn move_resize_window(&self, window: &Window, x: i32, y: i32, width: u32, height: u32) {
        unsafe {
            xlib::XMoveResizeWindow(self.display, window.get_x_window(), x, y, width, height);
        }
    }
}
