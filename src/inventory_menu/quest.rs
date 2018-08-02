use super::{inv_menu_state, InventoryMenu};
use core::fmt::Write;
use libtww::game::Console;
use libtww::link::item::*;
use libtww::link::quest_items::*;
use libtww::link::song;
use libtww::Link;
/////

// TODO: Power Bracelet

/////
use crate::controller;
use crate::utils::*;

static mut cursor: usize = 0;
static mut scroll_offset: usize = 0;

pub fn transition_into() {}

const MENU_ITEM_SWORD: usize = 0;
const MENU_ITEM_SHIELD: usize = 1;
const MENU_ITEM_PIRATES_CHARM: usize = 2;
const MENU_ITEM_POWER_BRACELETS: usize = 3;
const MENU_ITEM_HEROS_CHARM: usize = 4;
const MENU_ITEM_WINDS_REQUIEM: usize = 5;
const MENU_ITEM_BALLAD_OF_GALES: usize = 6;
const MENU_ITEM_COMMAND_MELODY: usize = 7;
const MENU_ITEM_EARTH_GODS_LYRIC: usize = 8;
const MENU_ITEM_WIND_GODS_ARIA: usize = 9;
const MENU_ITEM_SONG_OF_PASSING: usize = 10;
const MENU_ITEM_SPACE_0: usize = 11;
const MENU_ITEM_PROGRESSION: usize = 12;

const ITEMS: [&str; 13] = [
    "Sword            ",
    "Shield           ",
    "Pirates's Charm  ",
    "Power Bracelets  ",
    "Hero's Charm     ",
    "Wind's Requiem   ",
    "Ballad of Gales  ",
    "Command Melody   ",
    "Earth God's Lyric",
    "Wind God's Aria  ",
    "Song of Passing  ",
    "",
    "Progression Items",
];

// Bomb Bag
// Quiver
// Wallet Upgrade

fn handle_item_switch() {
    let link = Link::get();
    let quest_items = QuestItems::get();
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();
    let index = unsafe { cursor };
    if dpad_left {
        match index {
            MENU_ITEM_SWORD => match quest_items.sword {
                Sword::None => link.set_sword(Sword::FullyChargedMasterSword),
                Sword::HerosSword => link.set_sword(Sword::None),
                Sword::UnchargedMasterSword => link.set_sword(Sword::HerosSword),
                Sword::HalfChargedMasterSword => link.set_sword(Sword::UnchargedMasterSword),
                Sword::FullyChargedMasterSword => link.set_sword(Sword::HalfChargedMasterSword),
            },
            MENU_ITEM_SHIELD => match quest_items.shield {
                Shield::None => link.set_shield(Shield::MirrorShield),
                Shield::HerosShield => link.set_shield(Shield::None),
                Shield::MirrorShield => link.set_shield(Shield::HerosShield),
            },
            MENU_ITEM_HEROS_CHARM => {
                quest_items.heros_charm = match quest_items.heros_charm {
                    HerosCharm::None => HerosCharm::Enabled,
                    HerosCharm::Disabled => HerosCharm::None,
                    HerosCharm::Enabled => HerosCharm::Disabled,
                };
            }
            MENU_ITEM_POWER_BRACELETS => {
                quest_items.has_power_bracelets = if quest_items.has_power_bracelets {
                    false
                } else {
                    true
                };
            }
            MENU_ITEM_PIRATES_CHARM => {
                quest_items.has_pirates_charm = if quest_items.has_pirates_charm {
                    false
                } else {
                    true
                }
            }
            MENU_ITEM_WINDS_REQUIEM => song::WINDS_REQUIEM.lock(),
            MENU_ITEM_BALLAD_OF_GALES => song::BALLAD_OF_GALES.lock(),
            MENU_ITEM_COMMAND_MELODY => song::COMMAND_MELODY.lock(),
            MENU_ITEM_EARTH_GODS_LYRIC => song::EARTH_GODS_LYRIC.lock(),
            MENU_ITEM_WIND_GODS_ARIA => song::WIND_GODS_ARIA.lock(),
            MENU_ITEM_SONG_OF_PASSING => song::SONG_OF_PASSING.lock(),
            _ => {}
        }
    } else if dpad_right {
        match index {
            MENU_ITEM_SWORD => match quest_items.sword {
                Sword::None => link.set_sword(Sword::HerosSword),
                Sword::HerosSword => link.set_sword(Sword::UnchargedMasterSword),
                Sword::UnchargedMasterSword => link.set_sword(Sword::HalfChargedMasterSword),
                Sword::HalfChargedMasterSword => link.set_sword(Sword::FullyChargedMasterSword),
                Sword::FullyChargedMasterSword => link.set_sword(Sword::None),
            },
            MENU_ITEM_SHIELD => match quest_items.shield {
                Shield::None => link.set_shield(Shield::HerosShield),
                Shield::HerosShield => link.set_shield(Shield::MirrorShield),
                Shield::MirrorShield => link.set_shield(Shield::None),
            },
            MENU_ITEM_HEROS_CHARM => {
                quest_items.heros_charm = match quest_items.heros_charm {
                    HerosCharm::None => HerosCharm::Disabled,
                    HerosCharm::Disabled => HerosCharm::Enabled,
                    HerosCharm::Enabled => HerosCharm::None,
                };
            }
            MENU_ITEM_POWER_BRACELETS => {
                quest_items.has_power_bracelets = if quest_items.has_power_bracelets {
                    false
                } else {
                    true
                };
            }
            MENU_ITEM_PIRATES_CHARM => {
                quest_items.has_pirates_charm = if quest_items.has_pirates_charm {
                    false
                } else {
                    true
                }
            }
            MENU_ITEM_WINDS_REQUIEM => song::WINDS_REQUIEM.unlock(),
            MENU_ITEM_BALLAD_OF_GALES => song::BALLAD_OF_GALES.unlock(),
            MENU_ITEM_COMMAND_MELODY => song::COMMAND_MELODY.unlock(),
            MENU_ITEM_EARTH_GODS_LYRIC => song::EARTH_GODS_LYRIC.unlock(),
            MENU_ITEM_WIND_GODS_ARIA => song::WIND_GODS_ARIA.unlock(),
            MENU_ITEM_SONG_OF_PASSING => song::SONG_OF_PASSING.unlock(),
            _ => {}
        }
    }
}

pub fn scroll_move_cursor() {
    if controller::DPAD_UP.is_pressed() && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            if cursor >= 4 && cursor - 4 < scroll_offset {
                scroll_offset = cursor - 4;
            }
        }
    } else if controller::DPAD_DOWN.is_pressed() && unsafe { cursor + 1 } < ITEMS.len() {
        unsafe {
            cursor += 1;
            if cursor + 4 < ITEMS.len() && cursor > scroll_offset + 20 {
                scroll_offset = cursor - 20;
            }
        }
    }
}

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Quest Menu");
    let _ = write!(lines[1].begin(), "==========");

    let pressed_b = controller::B.is_pressed();
    let a_button = controller::A.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Main;
        }
        transition(MenuState::InventoryMenu);
        return;
    } else if a_button {
        if unsafe { cursor } == MENU_ITEM_PROGRESSION {
            unsafe {
                inv_menu_state = InventoryMenu::Progression;
            }
            transition(MenuState::InventoryMenu);
            return;
        }
    }

    scroll_move_cursor();

    handle_item_switch();

    for (index, (line, &text)) in lines
        .into_iter()
        .skip(3)
        .zip(ITEMS.iter().skip(unsafe { scroll_offset }))
        .enumerate()
        .take(25)
    {
        let index = index + unsafe { scroll_offset };

        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let item_text = get_item_text(index);
        let item_text = if item_text.len() > 45 {
            &item_text[..45]
        } else {
            item_text
        };

        let _ = match index {
            MENU_ITEM_PROGRESSION | MENU_ITEM_SPACE_0 => write!(line.append(), "{}", text),
            _ => write!(line.append(), "{}: {}", text, item_text),
        };
    }
}

fn get_item_text<'a>(index: usize) -> &'a str {
    let link = Link::get();
    let quest_items = QuestItems::get();
    match index {
        MENU_ITEM_SWORD => item_id_to_str(link.sword_id),
        MENU_ITEM_SHIELD => item_id_to_str(link.shield_id),
        MENU_ITEM_POWER_BRACELETS => {
            if quest_items.has_power_bracelets {
                "Power Bracelets"
            } else {
                ""
            }
        }
        MENU_ITEM_HEROS_CHARM => match quest_items.heros_charm {
            HerosCharm::None => "",
            HerosCharm::Disabled => "Disabled",
            HerosCharm::Enabled => "Enabled",
        },
        MENU_ITEM_PIRATES_CHARM => {
            if quest_items.has_pirates_charm {
                "Pirate's Charm"
            } else {
                ""
            }
        }
        MENU_ITEM_WINDS_REQUIEM => {
            if song::WINDS_REQUIEM.is_unlocked() {
                "Wind's Requiem"
            } else {
                ""
            }
        }
        MENU_ITEM_BALLAD_OF_GALES => {
            if song::BALLAD_OF_GALES.is_unlocked() {
                "Ballad of Gales"
            } else {
                ""
            }
        }
        MENU_ITEM_COMMAND_MELODY => {
            if song::COMMAND_MELODY.is_unlocked() {
                "Command Melody"
            } else {
                ""
            }
        }
        MENU_ITEM_EARTH_GODS_LYRIC => {
            if song::EARTH_GODS_LYRIC.is_unlocked() {
                "Earth God's Lyric"
            } else {
                ""
            }
        }
        MENU_ITEM_WIND_GODS_ARIA => {
            if song::WIND_GODS_ARIA.is_unlocked() {
                "Wind God's Aria"
            } else {
                ""
            }
        }
        MENU_ITEM_SONG_OF_PASSING => {
            if song::SONG_OF_PASSING.is_unlocked() {
                "Song of Passing"
            } else {
                ""
            }
        }
        _ => "",
    }
}
