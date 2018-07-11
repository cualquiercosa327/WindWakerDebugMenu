#![no_std]
#![allow(non_upper_case_globals)]

extern crate gcn;
extern crate libtww;
#[macro_use]
extern crate lazy_static;

use libtww::game::Console;
use libtww::system;

pub mod cheat_menu;
pub mod controller;
pub mod flag_menu;
pub mod inventory_menu;
pub mod main_menu;
pub mod mutex;
pub mod popups;
pub mod quest_menu;
pub mod spawn_menu;
pub mod triforce;
pub mod utils;
pub mod warp_menu;

// use mutex::*;
use utils::*;

pub static mut visible: bool = false;

#[no_mangle]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    let console = Console::get();
    console.line_count = 32;
    console.x = 0;
    console.y = 16;
    console.font_scale_x *= 1.2;
    console.font_scale_y *= 1.2;
    console.background_color.a = 150;
    console.clear();
}

#[no_mangle]
pub extern "C" fn game_loop() {
    cheat_menu::apply_cheats();
    let d_down = controller::DPAD_DOWN.is_pressed();
    let rt_down = controller::R.is_down();
    let console = Console::get();

    if unsafe { visible } {
        console.background_color.a = 150;
        match unsafe { menu_state } {
            MenuState::MainMenu => main_menu::render(),
            MenuState::WarpMenu => warp_menu::render(),
            MenuState::FlagMenu => flag_menu::render(),
            MenuState::InventoryMenu => inventory_menu::render(),
            MenuState::CheatMenu => cheat_menu::render(),
            MenuState::SpawnMenu => spawn_menu::render(),
            MenuState::QuestMenu => quest_menu::render(),
            MenuState::Triforce => triforce::render(),
        }
    } else if d_down && rt_down && unsafe { !popups::visible } {
        console.visible = true;
        unsafe {
            visible = true;
        }
    } else {
        // Only check popups if the Debug Menu is not open
        // popups::check_global_flags();
        memory::render_watches();
    }
}
