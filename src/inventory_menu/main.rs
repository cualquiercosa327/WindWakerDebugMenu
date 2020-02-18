use core::fmt::Write;
use libtww::game::Console;

use crate::controller;
use crate::utils::{move_cursor, transition, MenuState};

use super::{inv_menu_state, InventoryMenu};

static mut cursor: usize = 0;

pub fn transition_into() {}

const MENU_ITEM_INVENTORY: usize = 0;
const MENU_ITEM_QUEST: usize = 1;
const MENU_ITEM_AMOUNTS: usize = 2;

const ITEMS: [&str; 3] = ["Inventory Menu", "Quest Menu", "Amounts"];

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Inventory Menu");
    let _ = write!(lines[1].begin(), "==============");

    let pressed_b = controller::B.is_pressed();
    let pressed_a = controller::A.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Main;
        }
        transition(MenuState::MainMenu);
        return;
    }

    if pressed_a {
        unsafe {
            if cursor == MENU_ITEM_INVENTORY {
                inv_menu_state = InventoryMenu::Equipment;
                transition(MenuState::InventoryMenu);
                return;
            } else if cursor == MENU_ITEM_QUEST {
                inv_menu_state = InventoryMenu::Quest;
                transition(MenuState::InventoryMenu);
            } else if cursor == MENU_ITEM_AMOUNTS {
                inv_menu_state = InventoryMenu::Amounts;
                transition(MenuState::InventoryMenu);
                return;
            }
        }
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
