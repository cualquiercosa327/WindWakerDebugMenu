#![no_std]
#![allow(non_upper_case_globals)]

extern crate gcn;
extern crate libtww;
#[macro_use]
extern crate lazy_static;

use libtww::game::Console;
use libtww::system;
use libtww::system::gx;
use core::fmt::Write;

pub mod cheat_menu;
pub mod controller;
pub mod flag_menu;
pub mod inventory_menu;
pub mod main_menu;
pub mod memory;
pub mod popups;
pub mod spawn_menu;
pub mod utils;
pub mod warp_menu;

pub static mut visible: bool = false;

struct State {
    square1: Square,
}

static mut STATE: Option<State> = None;

unsafe fn get_state() -> &'static mut State {
    STATE.get_or_insert_with(|| {
        State { square1: Square {
            x: 20.0,
            y: 20.0,
            width: 20.0,
            height: 20.0
        }}
    })
}
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
    let state = unsafe { get_state() };
    cheat_menu::apply_cheats();
    let d_down = controller::DPAD_DOWN.is_pressed();
    let d_up = controller::DPAD_UP.is_pressed();
    let d_left = controller::DPAD_LEFT.is_pressed();
    let d_right = controller::DPAD_RIGHT.is_pressed();
    let rt_down = controller::R.is_down();
    let lt_down = controller::L.is_down();
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
        // popups::check_global_flags();
        let lines = &mut console.lines;
        let _ = write!(lines[0].begin(), "{} {}", state.square1.x, state.square1.y);
        if lt_down {
            if d_down { state.square1.move_down(); }
            if d_up { state.square1.move_up(); }
            if d_left { state.square1.move_left(); }
            if d_right { state.square1.move_right(); }
        }
    }
}

struct Square {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl Square {
    pub fn draw(&self) {
        unsafe {
            gx::clear_vtx_desc();
            gx::set_vtx_desc(gx::VA_POS as u8, gx::DIRECT);
            gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_POS, gx::POS_XY, gx::F32, 0);
            gx::set_num_tex_gens(0);
            gx::set_tev_order(gx::TEVSTAGE0, gx::TEXCOORD_NULL, gx::TEXMAP_NULL, gx::COLOR0A0);
            gx::set_tev_op(gx::TEVSTAGE0, gx::PASSCLR);

            // gx::load_pos_mtx_imm(&mut system::j3d::CAMERA_MATRIX, gx::PNMTX0);
            gx::begin(gx::QUADS, gx::VTXFMT0, 4);
            {
                    gx::submit_f32s(&[self.x, self.y]);
                    gx::submit_f32s(&[self.x + self.width, self.y]);
                    gx::submit_f32s(&[self.x + self.width, self.y + self.height]);
                    gx::submit_f32s(&[self.x, self.y + self.height]);
            }
            gx::end();
        }
    }

    pub fn move_left(&mut self) {
        self.x -= 2.0;
    }

    pub fn move_right(&mut self) {
        self.x += 2.0;
    }

    pub fn move_up(&mut self) {
        self.y -= 2.0;
    }

    pub fn move_down(&mut self) {
        self.y += 2.0;
    }

}

fn gu_ortho(mt: &mut gx::Mtx44 , t: f32, b: f32, l: f32, r: f32, n: f32, f: f32)
{
	let tmp = 1.0/(r-l);
	mt.cells[0][0] = 2.0*tmp;
	mt.cells[0][1] = 0.0;
	mt.cells[0][2] = 0.0;
	mt.cells[0][3] = -(r+l)*tmp;

	let tmp = 1.0/(t-b);
	mt.cells[1][0] = 0.0;
	mt.cells[1][1] = 2.0*tmp;
	mt.cells[1][2] = 0.0;
	mt.cells[1][3] = -(t+b)*tmp;

	let tmp = 1.0/(f-n);
	mt.cells[2][0] = 0.0;
	mt.cells[2][1] = 0.0;
	mt.cells[2][2] = -(1.0)*tmp;
	mt.cells[2][3] = -(f)*tmp;

	mt.cells[3][0] = 0.0;
	mt.cells[3][1] = 0.0;
	mt.cells[3][2] = 0.0;
	mt.cells[3][3] = 1.0;
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    let state = get_state();

    gx::set_z_mode(gx::ENABLE, gx::LEQUAL, gx::TRUE);

    //projection
    let mut perspective: gx::Mtx44 = gx::Mtx44 { cells: [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0]
    ]};
    gu_ortho(&mut perspective, 0.0, 479.0, 0.0, 639.0, 0.0, 300.0);
    gx::set_projection(&mut perspective, gx::ORTHOGRAPHIC);

    gx::set_cull_mode(gx::CULL_BACK);

    gx::set_blend_mode(
        gx::BM_BLEND,
        gx::BL_SRCALPHA,
        gx::BL_INVSRCALPHA,
        gx::LO_SET,
    );

    state.square1.draw();
}
