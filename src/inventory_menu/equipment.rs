use core::fmt::Write;
use libtww::game::Console;
use libtww::link::inventory::Inventory;
use libtww::link::item::*;

use super::{inv_menu_state, InventoryMenu};
use crate::controller;
use crate::utils::*;

static mut cursor: usize = 0;

#[derive(Copy, Clone)]
enum ItemType {
    SingleItem(u8),
    PictoBox,
    Bow,
    Bottle,
}

use self::ItemType::*;

const ITEM_SLOTS: [(&str, ItemType); 21] = [
    ("Telescope:     ", SingleItem(TELESCOPE)),
    ("Sail:          ", SingleItem(SAIL)),
    ("Wind Waker:    ", SingleItem(WIND_WAKER)),
    ("Grappling Hook:", SingleItem(GRAPPLING_HOOK)),
    ("Spoils Bag:    ", SingleItem(SPOILS_BAG)),
    ("Boomerang:     ", SingleItem(BOOMERANG)),
    ("Deku Leaf:     ", SingleItem(DEKU_LEAF)),
    ("Tingle Tuner:  ", SingleItem(TINGLE_TUNER)),
    ("Picto Box:     ", PictoBox),
    ("Iron Boots:    ", SingleItem(IRON_BOOTS)),
    ("Magic Armor:   ", SingleItem(MAGIC_ARMOR)),
    ("Bait Bag:      ", SingleItem(BAIT_BAG)),
    ("Bow:           ", Bow),
    ("Bombs:         ", SingleItem(BOMBS)),
    ("Bottle 1:      ", Bottle),
    ("Bottle 2:      ", Bottle),
    ("Bottle 3:      ", Bottle),
    ("Bottle 4:      ", Bottle),
    ("Delivery Bag:  ", SingleItem(DELIVERY_BAG)),
    ("Hookshot:      ", SingleItem(HOOKSHOT)),
    ("Skull Hammer:  ", SingleItem(SKULL_HAMMER)),
];

static mut scroll_offset: usize = 0;

pub fn transition_into() {}

pub fn scroll_move_cursor() {
    if controller::DPAD_UP.is_pressed() && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            if cursor >= 4 && cursor - 4 < scroll_offset {
                scroll_offset = cursor - 4;
            }
        }
    } else if controller::DPAD_DOWN.is_pressed() && unsafe { cursor + 1 } < ITEM_SLOTS.len() {
        unsafe {
            cursor += 1;
            if cursor + 4 < ITEM_SLOTS.len() && cursor > scroll_offset + 20 {
                scroll_offset = cursor - 20;
            }
        }
    }
}

pub fn item_id_to_str(item_id: u8) -> &'static str {
    match item_id {
        HEART_DROP => "Heart Drop",
        GREEN_RUPEE_DROP => "Green Rupee Drop",
        BLUE_RUPEE_DROP => "Blue Rupee Drop",
        YELLOW_RUPEE_DROP => "Yellow Rupee Drop",
        RED_RUPEE_DROP => "Red Rupee Drop",
        PURPLE_RUPEE_DROP => "Purple Rupee Drop",
        ORANGE_RUPEE_DROP => "Orange Rupee Drop",
        HEART_PIECE => "Heart Piece",
        HEART_CONTAINER => "Heart Container",
        SMALL_MAGIC_DROP => "Small Magic Drop",
        LARGE_MAGIC_DROP => "Large Magic Drop",
        BOMB_DROP => "Bomb Drop",
        SILVER_RUPEE_DROP => "Silver Rupee Drop",
        SINGLE_ARROW_DROP => "Single Arrow Drop",
        DOUBLE_ARROW_DROP => "Double Arrow Drop",
        TRIPLE_ARROW_DROP => "Triple Arrow Drop",
        KEY_DROP => "Key Drop",
        FAIRY_DROP => "Fairy Drop",
        TRIPLE_HEART_DROP => "Triple Heart Drop",
        JOY_PENDANT => "Joy Pendant",
        TELESCOPE => "Telescope",
        TINGLE_TUNER => "Tingle Tuner",
        WIND_WAKER => "Wind Waker",
        PICTO_BOX => "Picto Box",
        SPOILS_BAG => "Spoils Bag",
        GRAPPLING_HOOK => "Grappling Hook",
        DELUXE_PICTO_BOX => "Deluxe Picto Box",
        BOW => "Bow",
        POWER_BRACELETS => "Power Bracelets",
        IRON_BOOTS => "Iron Boots",
        MAGIC_ARMOR => "Magic Armor",
        WATER_BOOTS => "Water Boots",
        BAIT_BAG => "Bait Bag",
        BOOMERANG => "Boomerang",
        HOOKSHOT => "Hookshot",
        DELIVERY_BAG => "Delivery Bag",
        BOMBS => "Bombs",
        TUNIC => "Tunic",
        SKULL_HAMMER => "Skull Hammer",
        DEKU_LEAF => "Deku Leaf",
        BOW_WITH_FIRE_AND_ICE_ARROWS => "Bow With Fire And Ice Arrows",
        BOW_WITH_LIGHT_ARROWS => "Bow With Light Arrows",
        HEROS_SWORD => "Heros Sword",
        UNCHARGED_MASTER_SWORD => "Uncharged Master Sword",
        HALF_CHARGED_MASTER_SWORD => "Half Charged Master Sword",
        HEROS_SHIELD => "Heros Shield",
        MIRROR_SHIELD => "Mirror Shield",
        FULLY_CHARGED_MASTER_SWORD => "Fully Charged Master Sword",
        PIRATES_CHARM => "Pirates Charm",
        HEROS_CHARM => "Heros Charm",
        SKULL_NECKLACE => "Skull Necklace",
        BOKO_BABA_SEED => "Boko Baba Seed",
        GOLDEN_FEATHER => "Golden Feather",
        KNIGHTS_CREST => "Knights Crest",
        RED_CHU_JELLY => "Red Chu Jelly",
        GREEN_CHU_JELLY => "Green Chu Jelly",
        BLUE_CHU_JELLY => "Blue Chu Jelly",
        MAP => "Map",
        COMPASS => "Compass",
        BIG_KEY => "Big Key",
        EMPTY_BOTTLE => "Empty Bottle",
        RED_POTION => "Red Potion",
        GREEN_POTION => "Green Potion",
        BLUE_POTION => "Blue Potion",
        ELIXIR_SOUP_HALF => "Elixir Soup Half",
        ELIXIR_SOUP => "Elixir Soup",
        WATER => "Water",
        FAIRY => "Fairy",
        FOREST_FIREFLY => "Forest Firefly",
        FOREST_WATER => "Forest Water",
        TRIFORCE_PIECE_1 => "Triforce Piece 1",
        TRIFORCE_PIECE_2 => "Triforce Piece 2",
        TRIFORCE_PIECE_3 => "Triforce Piece 3",
        TRIFORCE_PIECE_4 => "Triforce Piece 4",
        TRIFORCE_PIECE_5 => "Triforce Piece 5",
        TRIFORCE_PIECE_6 => "Triforce Piece 6",
        TRIFORCE_PIECE_7 => "Triforce Piece 7",
        TRIFORCE_PIECE_8 => "Triforce Piece 8",
        NAYRUS_PEARL => "Nayrus Pearl",
        DINS_PEARL => "Dins Pearl",
        FARORES_PEARL => "Farores Pearl",
        SAIL => "Sail",
        TRIFORCE_CHART => "Triforce Chart",
        ALL_PURPOSE_BAIT => "All Purpose Bait",
        HYOI_PEAR => "Hyoi Pear",
        TOWN_FLOWER => "Town Flower",
        SEA_FLOWER => "Sea Flower",
        EXOTIC_FLOWER => "Exotic Flower",
        HEROS_FLAG => "Heros Flag",
        BIG_CATCH_FLAG => "Big Catch Flag",
        BIG_SALE_FLAG => "Big Sale Flag",
        PINWHEEL => "Pinwheel",
        SICKLE_MOON_FLAG => "Sickle Moon Flag",
        SKULL_TOWER_IDOL => "Skull Tower Idol",
        FOUNTAIN_IDOL => "Fountain Idol",
        POSTMAN_STATUE => "Postman Statue",
        SHOP_GURU_STATUE => "Shop Guru Statue",
        FATHERS_LETTER => "Fathers Letter",
        NOTE_TO_MOM => "Note To Mom",
        MAGGIES_LETTER => "Maggies Letter",
        MOBLINS_LETTER => "Moblins Letter",
        CABANA_DEED => "Cabana Deed",
        COMPLIMENTARY_ID => "Complimentary Id",
        FILL_UP_COUPON => "Fill Up Coupon",
        GOLDEN_TINGLE_HEAD => "Golden Tingle Head",
        EMPTY => "",
        _ => "???",
    }
}

fn handle_item_switch() {
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();
    let item_slot = unsafe { cursor };
    let item_id = Inventory::get_by_slot_id(item_slot);
    let (_, item_type) = ITEM_SLOTS[item_slot];
    if dpad_left {
        let new_item_id = match (item_type, item_id) {
            (PictoBox, DELUXE_PICTO_BOX) => PICTO_BOX,
            (Bow, BOW_WITH_LIGHT_ARROWS) => BOW_WITH_FIRE_AND_ICE_ARROWS,
            (Bow, BOW_WITH_FIRE_AND_ICE_ARROWS) => BOW,
            (Bottle, FOREST_WATER) => FOREST_FIREFLY,
            (Bottle, FOREST_FIREFLY) => FAIRY,
            (Bottle, FAIRY) => WATER,
            (Bottle, WATER) => ELIXIR_SOUP,
            (Bottle, ELIXIR_SOUP) => ELIXIR_SOUP_HALF,
            (Bottle, ELIXIR_SOUP_HALF) => BLUE_POTION,
            (Bottle, BLUE_POTION) => GREEN_POTION,
            (Bottle, GREEN_POTION) => RED_POTION,
            (Bottle, RED_POTION) => EMPTY_BOTTLE,
            _ => EMPTY,
        };
        Inventory::set_by_slot_id(item_slot, new_item_id);
    } else if dpad_right {
        let new_item_id = match (item_type, item_id) {
            (PictoBox, PICTO_BOX) => DELUXE_PICTO_BOX,
            (PictoBox, EMPTY) => PICTO_BOX,
            (Bow, BOW_WITH_FIRE_AND_ICE_ARROWS) => BOW_WITH_LIGHT_ARROWS,
            (Bow, BOW) => BOW_WITH_FIRE_AND_ICE_ARROWS,
            (Bow, EMPTY) => BOW,
            (Bottle, FOREST_FIREFLY) => FOREST_WATER,
            (Bottle, FAIRY) => FOREST_FIREFLY,
            (Bottle, WATER) => FAIRY,
            (Bottle, ELIXIR_SOUP) => WATER,
            (Bottle, ELIXIR_SOUP_HALF) => ELIXIR_SOUP,
            (Bottle, BLUE_POTION) => ELIXIR_SOUP_HALF,
            (Bottle, GREEN_POTION) => BLUE_POTION,
            (Bottle, RED_POTION) => GREEN_POTION,
            (Bottle, EMPTY_BOTTLE) => RED_POTION,
            (Bottle, EMPTY) => EMPTY_BOTTLE,
            (SingleItem(x), _) => x,
            (_, x) => x,
        };
        Inventory::set_by_slot_id(item_slot, new_item_id);
    }
}

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Inventory Screen");
    let _ = write!(lines[1].begin(), "================");

    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Main;
        }
        transition(MenuState::InventoryMenu);
        return;
    }

    scroll_move_cursor();

    handle_item_switch();

    for (index, (line, &(text, _))) in lines
        .into_iter()
        .skip(3)
        .zip(ITEM_SLOTS.iter().skip(unsafe { scroll_offset }))
        .enumerate()
        .take(25)
    {
        let index = index + unsafe { scroll_offset };
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }
        let item_id = Inventory::get_by_slot_id(index);
        let item_text = item_id_to_str(item_id);
        let _ = write!(line.append(), "{} {}", text, item_text);
    }
}
