use core::fmt::Write;
use libtww::game::Console;
use libtww::link::inventory::Inventory;
use libtww::system;
use libtww::Link;

use crate::controller;
use crate::utils::*;

static mut cursor: usize = 0;
static mut scroll_offset: usize = 0;
static mut already_pressed_a: bool = false;

pub fn transition_into() {
    unsafe {
        already_pressed_a = false;
    }
}

pub fn scroll_move_cursor(len: usize) {
    if controller::DPAD_UP.is_pressed() && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            if cursor >= 4 && cursor - 4 < scroll_offset {
                scroll_offset = cursor - 4;
            }
        }
    } else if controller::DPAD_DOWN.is_pressed() && unsafe { cursor + 1 } < len {
        unsafe {
            cursor += 1;
            if cursor + 4 < len && cursor > scroll_offset + 20 {
                scroll_offset = cursor - 20;
            }
        }
    }
}

struct Cheat {
    id: CheatId,
    name: &'static str,
    active: bool,
    togglable: bool,
}

impl Cheat {
    const fn new(id: CheatId, name: &'static str, togglable: bool) -> Self {
        Cheat {
            id: id,
            name: name,
            active: false,
            togglable: togglable,
        }
    }
}

pub fn apply_cheats() {
    let link = Link::get();
    let inventory = Inventory::get();

    for cheat in unsafe { &cheats } {
        if cheat.active {
            match cheat.id {
                MoonJump => {
                    Link::position().y += 175.0;
                }
                Invincible => {
                    link.heart_quarters = link.heart_pieces;
                }
                InfiniteMagic => {
                    link.max_magic = 32;
                    link.magic = link.max_magic;
                }
                InfiniteRupees => {
                    link.rupees = 9999;
                }
                SwiftWind => {
                    let direction = Link::horizontal_movement_direction();
                    let wind = if direction < 0x1000 || direction > 0xF000 {
                        2
                    } else if direction < 0x3000 {
                        1
                    } else if direction < 0x5000 {
                        0
                    } else if direction < 0x7000 {
                        7
                    } else if direction < 0x9000 {
                        6
                    } else if direction < 0xB000 {
                        5
                    } else if direction < 0xD000 {
                        4
                    } else {
                        3
                    };
                    system::set_wind(wind);
                }
                InfiniteAir => {
                    Link::set_air_meter(900);
                }
                FastMovement => {
                    system::memory::write::<u32>(0x8039830C, 0x40000000);
                    system::memory::write::<u32>(0x80398310, 0x41000000);
                }
                HDRolls => {
                    // Credits to Trog and LagoLunatic for HD roll cheat
                    system::memory::write::<f32>(0x80350958, 6.0 / 17.0);
                    system::memory::write::<f32>(0x8035095C, 20.0);
                }
                InfiniteBombs => {
                    inventory.bomb_count = 99;
                }
                InfiniteArrows => {
                    inventory.arrow_count = 99;
                }
            }
        } else {
            match cheat.id {
                HDRolls => {
                    system::memory::write::<f32>(0x80350958, 1.5);
                    system::memory::write::<f32>(0x8035095C, 0.5);
                }
                _ => {}
            }
        }
    }
}

static mut cheats: [Cheat; 10] = [
    Cheat::new(Invincible, "Invincible", true),
    Cheat::new(InfiniteMagic, "Infinite Magic", true),
    Cheat::new(InfiniteAir, "Infinite Air", true),
    Cheat::new(InfiniteRupees, "Infinite Rupees", true),
    Cheat::new(InfiniteBombs, "Infinite Bombs", true),
    Cheat::new(InfiniteArrows, "Infinite Arrows", true),
    Cheat::new(SwiftWind, "Swift Wind", true),
    Cheat::new(MoonJump, "Moon Jump", false),
    Cheat::new(FastMovement, "Fast Movement", false),
    Cheat::new(HDRolls, "HD Style Rolls", true),
];

#[derive(Copy, Clone)]
enum CheatId {
    Invincible,
    InfiniteMagic,
    InfiniteAir,
    InfiniteRupees,
    InfiniteBombs,
    InfiniteArrows,
    SwiftWind,
    MoonJump,
    FastMovement,
    HDRolls,
}

use self::CheatId::*;

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Cheat Menu");
    let _ = write!(lines[1].begin(), "==========");

    let down_a = controller::A.is_down();
    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    scroll_move_cursor(unsafe { cheats.len() });

    let cheat_id = unsafe { cursor };
    let cheat = unsafe { &mut cheats[cheat_id] };

    unsafe {
        already_pressed_a |= pressed_a;
    }

    if cheat.togglable {
        cheat.active ^= pressed_a;
    } else if unsafe { already_pressed_a } {
        cheat.active = down_a;
    }

    for (index, (line, cheat)) in lines
        .into_iter()
        .skip(3)
        .zip(unsafe { cheats.iter().skip(scroll_offset) })
        .enumerate()
        .take(25)
    {
        let index = index + unsafe { scroll_offset };
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let checkbox = if cheat.active { "[x] " } else { "[ ] " };

        let text = cheat.name;
        let text = if text.len() > 45 { &text[..45] } else { text };

        let _ = write!(line.append(), "{}{}", checkbox, text);
    }
}
