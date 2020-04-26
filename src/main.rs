extern crate winmux;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::collections::HashMap;

use winmux::key_modifier::KeyModifier;
use winmux::key_command::KeyCommand;
use winmux::action::Action;
use winmux::window_manager::WindowManager;
use winmux::window_system::WindowSystem;

fn main() {
    env_logger::init();

    info!("starting winmux");

    let window_system = WindowSystem::new();

    let mut window_manager = WindowManager::new(&window_system);

    let mut actions = HashMap::new();

    actions.insert(KeyCommand::from_str("F1", KeyModifier::NONEMASK), Action::RaiseWindowUnderCursor);
    actions.insert(KeyCommand::from_str("F2", KeyModifier::MOD1MASK), Action::QuitWinmux);
    actions.insert(KeyCommand::from_str("F3", KeyModifier::NONEMASK), Action::ReloadWinmux);

    window_manager.set_actions(actions);

    window_manager.run();
}
