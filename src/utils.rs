use core::fmt;

use libtww::game::Console;
use libtww::link::CollisionType;
use libtww::Link;

use crate::{
    cheat_menu, controller, flag_menu, inventory_menu, main_menu, memory, quest_menu, spawn_menu,
    triforce, warp_menu,
};

pub struct ColonWrapper<'a>(pub &'a str, pub &'a str, pub usize);

impl fmt::Display for ColonWrapper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:", self.0)?;
        if self.0.len() + 1 < self.2 {
            for _ in 0..(self.2 - self.0.len()) {
                write!(f, " ")?;
            }
        }
        let _ = write!(f, "{}", self.1);
        Ok(())
    }
}

pub fn clear_menu() {
    let console = Console::get();
    let lines = &mut console.lines;
    for line in lines.into_iter().skip(3) {
        line.clear();
    }
}

pub fn transition(state: MenuState) {
    clear_menu();
    unsafe {
        menu_state = state;
    }
    match state {
        MenuState::MainMenu => main_menu::transition_into(),
        MenuState::WarpMenu => warp_menu::transition_into(),
        MenuState::FlagMenu => flag_menu::transition_into(),
        MenuState::InventoryMenu => inventory_menu::transition_into(),
        MenuState::CheatMenu => cheat_menu::transition_into(),
        MenuState::SpawnMenu => spawn_menu::transition_into(),
        MenuState::QuestMenu => quest_menu::transition_into(),
        MenuState::Triforce => triforce::render(),
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

#[derive(Copy, Clone)]
pub enum MenuState {
    MainMenu,
    WarpMenu,
    FlagMenu,
    InventoryMenu,
    CheatMenu,
    SpawnMenu,
    QuestMenu,
    Triforce,
}

pub static mut menu_state: MenuState = MenuState::MainMenu;
