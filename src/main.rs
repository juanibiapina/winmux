extern crate x11;

use x11::xlib;
use std::ptr::{
    null
};

fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(null());
    }
}
