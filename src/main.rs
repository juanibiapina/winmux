extern crate libc;
extern crate x11;

use std::env;
use std::ptr::null;
use std::ffi::CString;
use std::mem::zeroed;

use libc::{c_int, c_uint, execvp};

use x11::xlib;

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

fn main() {
    println!("{}", "Starting winmux");

    let filename = env::current_exe().unwrap().as_path().to_str().unwrap().to_string();

    let display : *mut xlib::Display = unsafe { xlib::XOpenDisplay(null()) };

    if display.is_null() {
        std::process::exit(1);
    }

    let f1 = CString::new("F1").unwrap();
    let f2 = CString::new("F2").unwrap();
    let f3 = CString::new("F3").unwrap();

    let f1_keysym = unsafe { xlib::XStringToKeysym(f1.as_ptr()) };
    let f2_keysym = unsafe { xlib::XStringToKeysym(f2.as_ptr()) };
    let f3_keysym = unsafe { xlib::XStringToKeysym(f3.as_ptr()) };

    unsafe {
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, f1_keysym) as c_int, xlib::Mod1Mask,
        xlib::XDefaultRootWindow(display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);

        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, f2_keysym) as c_int, xlib::Mod1Mask,
        xlib::XDefaultRootWindow(display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);

        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, f3_keysym) as c_int, xlib::Mod1Mask,
        xlib::XDefaultRootWindow(display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);

        xlib::XGrabButton(display, 1, xlib::Mod1Mask, xlib::XDefaultRootWindow(display), true as c_int,
        (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
        0, 0);
        xlib::XGrabButton(display, 3, xlib::Mod1Mask, xlib::XDefaultRootWindow(display), true as c_int,
        (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
        0, 0);
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
                    if keysym == f1_keysym {
                        if event.subwindow != 0 {
                            xlib::XRaiseWindow(display, event.subwindow);
                        }
                    }

                    if keysym == f2_keysym {
                        std::process::exit(0);
                    }

                    if keysym == f3_keysym {
                        let filename_c = CString::new(filename.as_bytes()).unwrap();
                        let mut slice : &mut [*const i8; 2] = &mut [
                            filename_c.as_ptr(),
                            null(),
                        ];
                        execvp(filename_c.as_ptr(), slice.as_mut_ptr());
                        panic!("failed to reload");
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
                _ => {}
            };
        }
    }
}
