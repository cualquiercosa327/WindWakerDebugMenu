use core::fmt::Write;
use libtww::game::Console;
use libtww::link::{pearl, triforce};

use crate::controller;
use crate::utils::*;

use super::{inv_menu_state, InventoryMenu};

static mut cursor: usize = 0;

#[derive(Copy, Clone)]
enum Item {
    Triforce(triforce::TriforcePiece),
    Pearl(pearl::Pearl),
}

impl Item {
    pub fn is_unlocked(self) -> bool {
        match self {
            Item::Triforce(t) => t.is_unlocked(),
            Item::Pearl(t) => t.is_unlocked(),
        }
    }

    pub fn lock(self) {
        match self {
            Item::Triforce(t) => t.lock(),
            Item::Pearl(t) => t.lock(),
        }
    }

    pub fn unlock(self) {
        match self {
            Item::Triforce(t) => t.unlock(),
            Item::Pearl(t) => t.unlock(),
        }
    }
}

const ITEMS: [(&str, Item); 11] = [
    ("Din's Pearl    :", Item::Pearl(pearl::DINS_PEARL)),
    ("Farore's Pearl :", Item::Pearl(pearl::FARORES_PEARL)),
    ("Nayru's Pearl  :", Item::Pearl(pearl::NAYRUS_PEARL)),
    (
        "Piece 1        :",
        Item::Triforce(triforce::TRIFORCE_PIECE_1),
    ),
    (
        "Piece 2        :",
        Item::Triforce(triforce::TRIFORCE_PIECE_2),
    ),
    (
        "Piece 3        :",
        Item::Triforce(triforce::TRIFORCE_PIECE_3),
    ),
    (
        "Piece 4        :",
        Item::Triforce(triforce::TRIFORCE_PIECE_4),
    ),
    (
        "Piece 5        :",
        Item::Triforce(triforce::TRIFORCE_PIECE_5),
    ),
    (
        "Piece 6        :",
        Item::Triforce(triforce::TRIFORCE_PIECE_6),
    ),
    (
        "Piece 7        :",
        Item::Triforce(triforce::TRIFORCE_PIECE_7),
    ),
    (
        "Piece 8        :",
        Item::Triforce(triforce::TRIFORCE_PIECE_8),
    ),
];

pub fn transition_into() {}

pub fn handle_input() {
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();
    let item_slot = unsafe { cursor };
    let (_, item) = ITEMS[item_slot];
    if dpad_right {
        item.unlock();
    } else if dpad_left {
        item.lock();
    }
}

pub fn render() {
    let console = Console::get();

    let lines = &mut console.lines;
    let _ = write!(lines[0].begin(), "Progress Items");
    let _ = write!(lines[1].begin(), "==============");

    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Quest;
        }
        transition(MenuState::InventoryMenu);
        return;
    }

    handle_input();
    move_cursor(ITEMS.len(), unsafe { &mut cursor });

    for (index, (line, &(content, item))) in lines.into_iter().skip(3).zip(ITEMS.iter()).enumerate()
    {
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }
        let _ = write!(
            line.append(),
            "{} {}",
            content,
            if item.is_unlocked() { "X" } else { "" }
        );
    }
}
