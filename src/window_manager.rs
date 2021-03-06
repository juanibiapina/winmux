extern crate libc;
extern crate x11;

use std::env;
use std::process::exit;
use std::ptr::null;
use std::ffi::CString;
use std::collections::HashMap;

use self::libc::execvp;
use self::x11::xlib;

use crate::key_command::KeyCommand;
use crate::action::Action;
use crate::event::{Event, event_name};
use crate::window_system::WindowSystem;

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

        WindowManager {
            current_exe: current_exe,
            window_system: window_system,
            actions: HashMap::new(),
        }
    }

    pub fn reload(&self) {
        let filename_c = CString::new(self.current_exe.as_bytes()).unwrap();
        let slice : &mut [*const i8; 2] = &mut [
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
                Event::Unknown(event_type) => {
                    warn!("event not handled: {}", event_name(event_type));
                },
            };
        }
    }
}
