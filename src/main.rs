extern crate winmux;

extern crate libc;
extern crate x11;

use std::env;
use std::ptr::null;
use std::ffi::CString;
use std::mem::zeroed;
use std::collections::HashMap;

use libc::{c_int, c_uint, execvp};
use x11::xlib;

use winmux::key_modifier;
use winmux::key_modifier::KeyModifier;
use winmux::key_command::KeyCommand;
use winmux::actions::Actions;

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

fn main() {
    println!("{}", "Starting winmux");

    let current_exe = env::current_exe().unwrap().as_path().to_str().unwrap().to_string();

    let display : *mut xlib::Display = unsafe { xlib::XOpenDisplay(null()) };

    if display.is_null() {
        panic!("Cannot open display");
    }

    let screen = unsafe { xlib::XDefaultScreenOfDisplay(display) };
    let root_window = unsafe { xlib::XRootWindowOfScreen(screen) };

    let mut actions = HashMap::new();

    actions.insert(KeyCommand::from_str("F1", key_modifier::NONEMASK), Actions::RaiseWindowUnderCursor);
    actions.insert(KeyCommand::from_str("F2", key_modifier::MOD1MASK), Actions::QuitWinmux);
    actions.insert(KeyCommand::from_str("F3", key_modifier::NONEMASK), Actions::ReloadWinmux);

    unsafe {
        // keyboard events
        for key_command in actions.keys() {
            xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, key_command.get_keysym()) as c_int, key_command.get_mask(),
            xlib::XDefaultRootWindow(display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        }

        // mouse events
        xlib::XGrabButton(display, 1, xlib::Mod1Mask, xlib::XDefaultRootWindow(display), true as c_int,
        (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
        0, 0);
        xlib::XGrabButton(display, 3, xlib::Mod1Mask, xlib::XDefaultRootWindow(display), true as c_int,
        (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
        0, 0);

        // window events
        xlib::XSelectInput(display, root_window, xlib::SubstructureRedirectMask);
    };

    let mut attr: xlib::XWindowAttributes = unsafe { zeroed() };

    let mut start: xlib::XButtonEvent = unsafe { zeroed() };
    start.subwindow = 0;

    loop {
        unsafe {
            let mut event: xlib::XEvent = zeroed();

            xlib::XNextEvent(display, &mut event);

            match event.get_type() {
                xlib::KeyPress => {
                    let event: xlib::XKeyEvent = From::from(event);

                    let keysym = xlib::XKeycodeToKeysym(display, event.keycode as u8, 0) as u64;
                    let keymodifier = KeyModifier::from_bits(0xEF & event.state as u32).unwrap();
                    let key_command = KeyCommand::new(keysym, keymodifier);

                    match actions.get(&key_command) {
                        Some(action) => {
                            match *action {
                                Actions::RaiseWindowUnderCursor => {
                                    xlib::XRaiseWindow(display, event.subwindow);
                                },
                                Actions::QuitWinmux => {
                                    std::process::exit(0);
                                },
                                Actions::ReloadWinmux => {
                                    let filename_c = CString::new(current_exe.as_bytes()).unwrap();
                                    let mut slice : &mut [*const i8; 2] = &mut [
                                        filename_c.as_ptr(),
                                        null(),
                                    ];
                                    execvp(filename_c.as_ptr(), slice.as_mut_ptr());
                                    panic!("failed to reload");
                                }
                            }
                        },
                        None => {},
                    }
                },
                xlib::ButtonPress => {
                    let event: xlib::XButtonEvent = From::from(event);
                    if event.subwindow != 0 {
                        xlib::XGetWindowAttributes(display, event.subwindow, &mut attr);
                        start = event;
                    }
                },
                xlib::MotionNotify => {
                    if start.subwindow != 0 {
                        let event: xlib::XButtonEvent = From::from(event);
                        let xdiff : c_int = event.x_root - start.x_root;
                        let ydiff : c_int = event.y_root - start.y_root;
                        xlib::XMoveResizeWindow(display, start.subwindow,
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
                    let event: xlib::XMapRequestEvent = From::from(event);
                    let window = event.window;
                    xlib::XMapWindow(display, window);
                },
                xlib::ConfigureRequest => {
                    let event: xlib::XConfigureRequestEvent = From::from(event);
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
                    xlib::XConfigureWindow(display, window, mask as u32, &mut window_changes);
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
