extern crate x11;

use std::ffi::CString;

use self::x11::xlib;

use key_modifier::KeyModifier;

#[derive(Eq, PartialEq, Hash)]
pub struct KeyCommand {
    key: u64,
    modifier: KeyModifier,
}

impl KeyCommand {
    pub fn new(keysym: u64, modifier: KeyModifier) -> KeyCommand {
        KeyCommand {
            key: keysym,
            modifier: modifier,
        }
    }

    pub fn from_str(name: &str, modifier: KeyModifier) -> KeyCommand {
        let c_name = match CString::new(name) {
            Ok(v) => v,
            Err(_) => panic!("Invalid key name"),
        };

        let keysym = unsafe {
            xlib::XStringToKeysym(c_name.as_ptr())
        };

        KeyCommand::new(keysym, modifier)
    }

    pub fn get_keysym(&self) -> u64 {
        self.key
    }

    pub fn get_mask(&self) -> u32 {
        self.modifier.get_mask()
    }
}
