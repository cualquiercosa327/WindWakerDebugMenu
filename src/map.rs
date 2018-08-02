use core::fmt::Write;
use libtww::game::Console;

use controller;
use utils::{move_cursor, transition, MenuState};

use super::{inv_menu_state, InventoryMenu};

static mut cursor: usize = 0;

pub fn transition_into() {}

const ITEMS: [&str; 1] = [""];

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Map Menu");
    let _ = write!(lines[1].begin(), "========");

    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Main;
        }
        transition(MenuState::InventoryMenu);
        return;
    }

    move_cursor(ITEMS.len(), unsafe { &mut cursor });

    for (index, (line, item)) in lines.into_iter().skip(3).zip(ITEMS.iter()).enumerate() {
        let index = index;
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }
        let _ = write!(line.append(), "{}", item);
    }
}
