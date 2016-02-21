extern crate libc;
extern crate x11;

use std::env;
use std::process::exit;
use std::mem::zeroed;
use std::ptr::null;
use std::ffi::CString;
use std::collections::HashMap;

use self::libc::{c_int, c_uint, execvp};
use self::x11::xlib;

use key_command::KeyCommand;
use key_modifier::KeyModifier;
use action::Action;
use window_system::WindowSystem;

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

pub struct WindowManager<'a> {
    current_exe: String,
    display: *mut xlib::Display,
    actions: HashMap<KeyCommand, Action>,
    window_system: &'a WindowSystem,
}

impl<'a> WindowManager<'a> {
    pub fn new(window_system: &'a WindowSystem) -> WindowManager<'a> {
        println!("Starting winmux");

        let current_exe = env::current_exe().unwrap().as_path().to_str().unwrap().to_string();

        let display = window_system.display;

        // window events
        window_system.select_input(xlib::SubstructureRedirectMask);

        // mouse events
        window_system.grab_button(1, xlib::Mod1Mask);
        window_system.grab_button(3, xlib::Mod1Mask);

        WindowManager {
            current_exe: current_exe,
            window_system: window_system,
            display: display,
            actions: HashMap::new(),
        }
    }

    pub fn reload(&self) {
        let filename_c = CString::new(self.current_exe.as_bytes()).unwrap();
        let mut slice : &mut [*const i8; 2] = &mut [
            filename_c.as_ptr(),
            null(),
        ];
        unsafe {
            execvp(filename_c.as_ptr(), slice.as_mut_ptr());
        }
        panic!("winmux: failed to reload");
    }

    pub fn set_actions(&mut self, actions: HashMap<KeyCommand, Action>) {
        self.actions = actions;

        for key_command in self.actions.keys() {
            self.window_system.grab_key(&key_command);
        }
    }

    pub fn run(&self) {
        let mut attr: xlib::XWindowAttributes = unsafe { zeroed() };

        let mut start: xlib::XButtonEvent = unsafe { zeroed() };
        start.subwindow = 0;

        loop {
            unsafe {
                let mut event: xlib::XEvent = zeroed();

                xlib::XNextEvent(self.display, &mut event);

                match event.get_type() {
                    xlib::KeyPress => {
                        let event = xlib::XKeyEvent::from(event);

                        let keysym = xlib::XKeycodeToKeysym(self.display, event.keycode as u8, 0) as u64;
                        let keymodifier = KeyModifier::from_bits(0xEF & event.state as u32).unwrap();
                        let key_command = KeyCommand::new(keysym, keymodifier);

                        match self.actions.get(&key_command) {
                            Some(action) => {
                                match *action {
                                    Action::RaiseWindowUnderCursor => {
                                        xlib::XRaiseWindow(self.display, event.subwindow);
                                    },
                                    Action::QuitWinmux => {
                                        exit(0);
                                    },
                                    Action::ReloadWinmux => {
                                        self.reload();
                                    }
                                }
                            },
                            None => {},
                        }
                    },
                    xlib::ButtonPress => {
                        let event = xlib::XButtonEvent::from(event);
                        if event.subwindow != 0 {
                            xlib::XGetWindowAttributes(self.display, event.subwindow, &mut attr);
                            start = event;
                        }
                    },
                    xlib::MotionNotify => {
                        if start.subwindow != 0 {
                            let event = xlib::XButtonEvent::from(event);
                            let xdiff : c_int = event.x_root - start.x_root;
                            let ydiff : c_int = event.y_root - start.y_root;
                            xlib::XMoveResizeWindow(self.display, start.subwindow,
                                                    attr.x + (if start.button==1 { xdiff } else { 0 }),
                                                    attr.y + (if start.button==1 { ydiff } else { 0 }),
                                                    max(1, attr.width + (if start.button==3 { xdiff } else { 0 })),
                                                    max(1, attr.height + (if start.button==3 { ydiff } else { 0 })));
                        }
                    },
                    xlib::ButtonRelease => {
                        start.subwindow = 0;
                    },
                    xlib::MapRequest => {
                        let event = xlib::XMapRequestEvent::from(event);
                        let window = event.window;
                        xlib::XMapWindow(self.display, window);
                    },
                    xlib::ConfigureRequest => {
                        let event = xlib::XConfigureRequestEvent::from(event);
                        let window = event.window;
                        let mask = event.value_mask;
                        let mut window_changes = xlib::XWindowChanges {
                            x: event.x,
                            y: event.y,
                            width: event.width,
                            height: event.height,
                            border_width: event.border_width,
                            stack_mode: event.detail,
                            sibling: event.above,
                        };
                        xlib::XConfigureWindow(self.display, window, mask as u32, &mut window_changes);
                    },
                    xlib::CirculateRequest => {
                        println!("Event circulate request");
                    },
                    _ => {
                        println!("Event {} not handled", event.get_type());
                    }
                };
            }
        }
    }
}
