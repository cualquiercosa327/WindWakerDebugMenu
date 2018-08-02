use core::fmt::Write;
use libtww::game::{event, Console};
use libtww::Link;

use crate::controller;
use crate::utils::*;
use crate::visible;

static mut cursor: usize = 0;

pub fn transition_into() {}

pub fn render() {
    const MENU_ITEM_COLLISION: usize = 0;
    const MENU_ITEM_STORAGE: usize = 1;
    const MENU_ITEM_FLAG: usize = 3;
    const MENU_ITEM_WARP: usize = 4;
    const MENU_ITEM_INVENTORY: usize = 5;
    const MENU_ITEM_CHEAT: usize = 6;
    const MENU_ITEM_SPAWN: usize = 7;
    const MENU_ITEM_MEMORY: usize = 8;

    let console = Console::get();

    let lines = &mut console.lines;
    let _ = write!(lines[0].begin(), "Debug Menu");
    let _ = write!(lines[1].begin(), "==========");

    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        console.visible = false;
        unsafe {
            visible = false;
        }
        return;
    }

    let contents = [
        "Collision: ",
        "Storage: ",
        "",
        "Flag Menu",
        "Warp Menu",
        "Inventory Menu",
        "Cheat Menu",
        "Spawn Menu",
        "Memory",
    ];

    move_cursor(contents.len(), unsafe { &mut cursor });

    if pressed_a {
        match unsafe { cursor } {
            MENU_ITEM_COLLISION => next_collision(),
            MENU_ITEM_STORAGE => event::set_event_cancel(!event::event_cancel()),
            MENU_ITEM_FLAG => {
                transition(MenuState::FlagMenu);
                return;
            }
            MENU_ITEM_WARP => {
                transition(MenuState::WarpMenu);
                return;
            }
            MENU_ITEM_INVENTORY => {
                transition(MenuState::InventoryMenu);
                return;
            }
            MENU_ITEM_CHEAT => {
                transition(MenuState::CheatMenu);
                return;
            }
            MENU_ITEM_SPAWN => {
                transition(MenuState::SpawnMenu);
                return;
            }
            MENU_ITEM_MEMORY => {
                transition(MenuState::Memory);
                return;
            }
            _ => {}
        }
    }

    for (index, (line, &content)) in lines.into_iter().skip(3).zip(&contents).enumerate() {
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }
        let _ = write!(line.append(), "{}", content);

        if index == MENU_ITEM_COLLISION {
            let _ = write!(line.append(), "{}", Link::collision());
        } else if index == MENU_ITEM_STORAGE {
            let _ = write!(line.append(), "{}", bool_to_text(event::event_cancel()));
        }
    }
}
