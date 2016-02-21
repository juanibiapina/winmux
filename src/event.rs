extern crate libc;

use self::libc::c_int;

use key_command::KeyCommand;
use mouse_command::MouseCommand;
use window::{Window, WindowChanges};

#[derive(Clone)]
pub enum Event {
    KeyPress(Window, KeyCommand),

    ButtonPress(Window, MouseCommand, i32, i32),
    ButtonRelease(Window, MouseCommand, i32, i32),

    MotionNotify(i32, i32),

    MapRequest(Window),
    ConfigureRequest(Window, WindowChanges),

    Unknown(c_int),
}
