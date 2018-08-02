use libtww::game::Console;
use libtww::link::CollisionType;
use libtww::Link;

use crate::controller;

use super::*;

macro_rules! define_menu {
    ( $( ($name:ident, $m:ident)),* ) => (
        #[derive(Copy, Clone)]
        pub enum MenuState {
            $(
                $name,
            )*
        }

        pub fn transition(state: MenuState) {
            clear_menu();
            unsafe { menu_state = state; }
            match state {
                $(
                    MenuState::$name => $m::transition_into(),
                )*
            }
        }

        pub fn render() {
            match unsafe { menu_state } {
                $(
                    MenuState::$name => $m::render(),
                )*
            }
        }
    )
}

pub static mut menu_state: MenuState = MenuState::MainMenu;

define_menu!(
    (MainMenu, main_menu),
    (WarpMenu, warp_menu),
    (FlagMenu, flag_menu),
    (InventoryMenu, inventory_menu),
    (CheatMenu, cheat_menu),
    (SpawnMenu, spawn_menu),
    (Memory, memory)
);
pub fn clear_menu() {
    let console = Console::get();
    let lines = &mut console.lines;
    for line in lines.into_iter().skip(3) {
        line.clear();
    }
}

pub fn move_cursor(len: usize, cursor: &mut usize) {
    let console = Console::get();
    let lines = &mut console.lines;

    if controller::DPAD_UP.is_pressed() && *cursor > 0 {
        *cursor -= 1;
        while lines[*cursor + 3].len() < 3 {
            *cursor -= 1;
        }
    } else if controller::DPAD_DOWN.is_pressed() && *cursor + 1 < len {
        *cursor += 1;
        while lines[*cursor + 3].len() < 3 {
            *cursor += 1;
        }
    }
}

pub fn next_collision() {
    let collision = Link::collision();
    let collision = match collision {
        CollisionType::Default => CollisionType::ChestStorage,
        CollisionType::ChestStorage => CollisionType::DoorCancel,
        CollisionType::DoorCancel => CollisionType::Default,
    };
    Link::set_collision(collision);
}

pub fn bool_to_text(b: bool) -> &'static str {
    if b {
        "on"
    } else {
        "off"
    }
}
