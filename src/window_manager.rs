extern crate libc;
extern crate x11;

use std::env;
use std::process::exit;
use std::ptr::null;
use std::ffi::CString;
use std::collections::HashMap;

use self::libc::{c_int, c_uint, execvp};
use self::x11::xlib;

use key_command::KeyCommand;
use mouse_command::MouseCommand;
use action::Action;
use event::{Event, event_name};
use window_system::WindowSystem;

fn max(a : c_int, b : c_int) -> c_uint {
    if a > b {
        a as c_uint
    } else {
        b as c_uint
    }
}

pub struct WindowManager<'a> {
    current_exe: String,
    actions: HashMap<KeyCommand, Action>,
    window_system: &'a WindowSystem,
}

impl<'a> WindowManager<'a> {
    pub fn new(window_system: &'a WindowSystem) -> WindowManager<'a> {
        let current_exe = env::current_exe().unwrap().as_path().to_str().unwrap().to_string();

        // window events
        window_system.select_input(xlib::SubstructureRedirectMask);

        // mouse events
        window_system.grab_button(&MouseCommand::new(1, xlib::Mod1Mask));
        window_system.grab_button(&MouseCommand::new(3, xlib::Mod1Mask));

        WindowManager {
            current_exe: current_exe,
            window_system: window_system,
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
        let mut window_attributes = None;
        let mut last_press_event = None;

        loop {
            let event = self.window_system.next_event();

            match event {
                Event::KeyPress(window, key_command) => {
                    match self.actions.get(&key_command) {
                        Some(action) => {
                            match *action {
                                Action::RaiseWindowUnderCursor => {
                                    self.window_system.raise_window(&window);
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
                Event::MapRequest(window) => {
                    self.window_system.map_window(&window);
                },
                Event::ConfigureRequest(window, window_changes) => {
                    self.window_system.configure_window(&window, &window_changes);
                },
                Event::ButtonPress(ref window, _, _, _) => {
                    window_attributes = Some(self.window_system.get_window_attributes(&window));
                    last_press_event = Some(event.clone());
                },
                Event::ButtonRelease(_, _, _, _) => {
                    window_attributes = None;
                    last_press_event = None;
                },
                Event::MotionNotify(x_root, y_root) => {
                    match last_press_event {
                        Some(ref last_press_event) => {
                            match last_press_event {
                                &Event::ButtonPress(ref window, ref mouse_command, last_x_root, last_y_root) => {
                                    let xdiff = x_root - last_x_root;
                                    let ydiff = y_root - last_y_root;
                                    let attr = match window_attributes {
                                        Some(ref window_attributes) => window_attributes,
                                        None => panic!("Inconsistent program state"),
                                    };

                                    self.window_system.move_resize_window(window,
                                                                          attr.x + (if mouse_command.button_number == 1 { xdiff } else { 0 }),
                                                                          attr.y + (if mouse_command.button_number == 1 { ydiff } else { 0 }),
                                                                          max(1, attr.width + (if mouse_command.button_number == 3 { xdiff } else { 0 })),
                                                                          max(1, attr.height + (if mouse_command.button_number == 3 { ydiff } else { 0 })));
                                },
                                _ => {},
                            };
                        },
                        None => {},
                    };
                },
                Event::Unknown(event_type) => {
                    warn!("event not handled: {}", event_name(event_type));
                },
            };
        }
    }
}
