#![no_std]
#![allow(non_upper_case_globals)]

extern crate libtww;
#[macro_use]
extern crate lazy_static;

use libtww::game::Console;
use libtww::system;

use gcn_fonts::prelude::*;

pub mod cheat_menu;
pub mod controller;
pub mod flag_menu;
pub mod inventory_menu;
pub mod main_menu;
pub mod memory;
pub mod popups;
pub mod print;
pub mod settings;
pub mod spawn_menu;
pub mod statics;
pub mod utils;
pub mod warp_menu;

pub static mut visible: bool = false;

struct State {
    font: UploadedFont,
    settings: settings::Settings,
}

static mut STATE: Option<State> = None;

static FONT: Font = gcn_fonts::include_font! { path: "res/Calamity-Bold.ttf", size: 18.0 };

unsafe fn get_state() -> &'static mut State {
    STATE.get_or_insert_with(|| State {
        font: FONT.upload(),
        settings: settings::Settings { drop_shadow: true },
    })
}

#[no_mangle]
pub extern "C" fn init() {
    // Call overridden instruction
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
        utils::render();
    } else if d_down && rt_down && unsafe { !popups::visible } {
        console.visible = true;
        unsafe {
            visible = true;
        }
    } else {
        memory::render_watches();
        // Only check popups if the Debug Menu is not open
        popups::check_global_flags();
    }
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    print::setup_draw();
    memory::render_watches();
}
