use core::fmt::Write;
use libtww::game::Console;

use crate::controller;
use crate::utils::{move_cursor, transition, MenuState};

use super::{inv_menu_state, InventoryMenu};

static mut cursor: usize = 0;

pub fn transition_into() {}

// Bomb Bag
pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Item Amounts");
    let _ = write!(lines[1].begin(), "============");

    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Main;
        }
        transition(MenuState::InventoryMenu);
        return;
    }

    let items = [""];

    move_cursor(items.len(), unsafe { &mut cursor });

    for (index, (line, item)) in lines.into_iter().skip(3).zip(items.iter()).enumerate() {
        let index = index;
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }
        let _ = write!(line.append(), "{}", item);
    }
}
