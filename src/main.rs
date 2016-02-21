extern crate winmux;

extern crate libc;

use std::collections::HashMap;

use winmux::key_modifier;
use winmux::key_command::KeyCommand;
use winmux::action::Action;
use winmux::window_manager::WindowManager;

fn main() {
    let mut window_manager = WindowManager::new();

    let mut actions = HashMap::new();

    actions.insert(KeyCommand::from_str("F1", key_modifier::NONEMASK), Action::RaiseWindowUnderCursor);
    actions.insert(KeyCommand::from_str("F2", key_modifier::MOD1MASK), Action::QuitWinmux);
    actions.insert(KeyCommand::from_str("F3", key_modifier::NONEMASK), Action::ReloadWinmux);

    window_manager.set_actions(actions);

    window_manager.run();
}
