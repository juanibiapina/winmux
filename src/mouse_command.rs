extern crate libc;

use self::libc::c_uint;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct MouseCommand {
    pub button_number: u32,
    pub modifiers: c_uint,
}

impl MouseCommand {
    pub fn new(button_number: u32, modifiers: c_uint) -> MouseCommand {
        MouseCommand {
            button_number: button_number,
            modifiers: modifiers,
        }
    }
}
