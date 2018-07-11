use arrayvec::ArrayVec;
use core::fmt::Write;
use core::fmt::{Debug, Display, Error, Formatter};
use libtww::game::Console;
use libtww::system::memory;
use libtww::Addr;

use controller;
use core::cell::RefCell;
use utils::*;
use Mutex;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Type {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Type::u8 => write!(f, "u8    "),
            Type::i8 => write!(f, "i8    "),
            Type::u16 => write!(f, "u16   "),
            Type::i16 => write!(f, "i16   "),
            Type::u32 => write!(f, "u32   "),
            Type::i32 => write!(f, "i32   "),
            Type::f32 => write!(f, "f32   "),
            Type::String => write!(f, "String"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Watch {
    addr: Addr,
    x: u32,
    y: u32,
    t: Type,
    hex: bool,
}

impl Default for Watch {
    fn default() -> Self {
        Watch {
            addr: 0x80000000,
            x: 0,
            y: 0,
            t: Type::String,
            hex: true,
        }
    }
}

impl Debug for Watch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{:<8X} {:<3} {:<3} {:<5} {:<6} ",
            self.addr, self.x, self.y, self.hex, self.t,
        )?;
        match self.t {
            Type::u8 => {
                let value = memory::read::<u8>(self.addr);
                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i8 => {
                let value = memory::read::<i8>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::u16 => {
                let value = memory::read::<u16>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i16 => {
                let value = memory::read::<i16>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::u32 => {
                let value = memory::read::<u32>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i32 => {
                let value = memory::read::<i32>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::f32 => {
                if self.hex {
                    let value = memory::read::<u32>(self.addr);
                    write!(f, "{:X}", value)
                } else {
                    let value = memory::read::<f32>(self.addr);
                    write!(f, "{:.*}", 5, value)
                }
            }
            Type::String => {
                let value = memory::read_str(memory::ptr(self.addr));
                write!(f, "{}", value)
            }
        }
    }
}

impl Display for Watch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.t {
            Type::u8 => {
                let value = memory::read::<u8>(self.addr);
                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i8 => {
                let value = memory::read::<i8>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::u16 => {
                let value = memory::read::<u16>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i16 => {
                let value = memory::read::<i16>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::u32 => {
                let value = memory::read::<u32>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i32 => {
                let value = memory::read::<i32>(self.addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::f32 => {
                if self.hex {
                    let value = memory::read::<u32>(self.addr);
                    write!(f, "{:X}", value)
                } else {
                    let value = memory::read::<f32>(self.addr);
                    write!(f, "{:.*}", 5, value)
                }
            }
            Type::String => {
                let value = memory::read_str(memory::ptr(self.addr));
                write!(f, "{}", value)
            }
        }
    }
}

static mut cursor: usize = 0;
static mut edit_cursor: usize = 0;
static mut in_submenu: bool = false;
const LINE_LEN: usize = 25;

lazy_static! {
    pub static ref ITEMS: Mutex<ArrayVec<[Watch; 128]>> = {
        let mut vec = ArrayVec::new();
        vec.push(Watch {
            addr: 0x80ACE9E8,
            x: 0,
            y: 0,
            t: Type::f32,
            hex: false,
        });
        Mutex(RefCell::new(vec))
    };
}

pub fn transition_into() {}

pub fn render_watches() {
    let console = Console::get();
    console.clear();
    console.visible = true;
    console.background_color.a = 0;

    let lines = &mut console.lines;

    ITEMS.borrow().iter().for_each(|item| {
        let mut line = lines[item.y as usize].begin();
        for _ in 0..item.x {
            let _ = write!(line, " ");
        }
        let _ = write!(line, "{}", item);
    });
}

pub fn render() {
    let console = Console::get();
    console.clear();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Memory Menu");
    let _ = write!(lines[1].begin(), "===========");
    let _ = write!(
        lines[3].begin(),
        "  {:<8} {:<3} {:<3} {:<5} {:<6} ",
        "Address",
        "X",
        "Y",
        "Hex",
        "Type"
    );
    let pressed_b = controller::B.is_pressed();
    let pressed_a = controller::A.is_pressed();

    unsafe {
        if in_submenu {
            if pressed_b {
                in_submenu = false;
                edit_cursor = 2;
            }

            let mut current_watch: Watch = ITEMS.borrow_mut().remove(cursor);

            if controller::DPAD_LEFT.is_pressed() && edit_cursor > 2 {
                if edit_cursor == 25 {
                    edit_cursor = 19;
                } else if edit_cursor == 19 {
                    edit_cursor = 15;
                } else if edit_cursor == 15 {
                    edit_cursor = 11;
                } else if edit_cursor == 11 {
                    edit_cursor = 9;
                } else {
                    edit_cursor -= 1;
                }
            } else if controller::DPAD_RIGHT.is_pressed() && edit_cursor + 1 < LINE_LEN {
                if edit_cursor == 9 {
                    edit_cursor = 11;
                } else if edit_cursor == 11 {
                    edit_cursor = 15;
                } else if edit_cursor == 15 {
                    edit_cursor = 19;
                } else if edit_cursor == 19 {
                    edit_cursor = 25;
                } else {
                    edit_cursor += 1;
                }
            } else if controller::DPAD_UP.is_pressed() {
                match edit_cursor {
                    3 => {
                        if current_watch.addr < 0x8F000000 {
                            current_watch.addr += 0x1000000;
                        }
                    }
                    4 => {
                        if current_watch.addr < 0x8FF00000 {
                            current_watch.addr += 0x100000;
                        }
                    }
                    5 => {
                        if current_watch.addr < 0x8FFF0000 {
                            current_watch.addr += 0x10000;
                        }
                    }
                    6 => {
                        if current_watch.addr < 0x8FFFF000 {
                            current_watch.addr += 0x1000;
                        }
                    }
                    7 => {
                        if current_watch.addr < 0x8FFFFF00 {
                            current_watch.addr += 0x100;
                        }
                    }
                    8 => {
                        if current_watch.addr < 0x8FFFFFF0 {
                            current_watch.addr += 0x10;
                        }
                    }
                    9 => {
                        if current_watch.addr < 0x8FFFFFFF {
                            current_watch.addr += 0x1;
                        }
                    }
                    11 => {
                        if current_watch.x < 255 {
                            current_watch.x += 1;
                        }
                    }
                    15 => {
                        if current_watch.y < 255 {
                            current_watch.y += 1;
                        }
                    }
                    19 => current_watch.hex = !current_watch.hex,
                    25 => {
                        current_watch.t = match current_watch.t {
                            Type::u8 => Type::i8,
                            Type::i8 => Type::u16,
                            Type::u16 => Type::i16,
                            Type::i16 => Type::u32,
                            Type::u32 => Type::i32,
                            Type::i32 => Type::f32,
                            Type::f32 => Type::String,
                            Type::String => Type::u8,
                        }
                    }
                    _ => {}
                }
            } else if controller::DPAD_DOWN.is_pressed() {
                match edit_cursor {
                    3 => {
                        if current_watch.addr > 0x80FFFFFF {
                            current_watch.addr -= 0x1000000;
                        }
                    }
                    4 => {
                        if current_watch.addr > 0x800FFFFF {
                            current_watch.addr -= 0x100000;
                        }
                    }
                    5 => {
                        if current_watch.addr > 0x8000FFFF {
                            current_watch.addr -= 0x10000;
                        }
                    }
                    6 => {
                        if current_watch.addr > 0x80000FFF {
                            current_watch.addr -= 0x1000;
                        }
                    }
                    7 => {
                        if current_watch.addr > 0x800000FF {
                            current_watch.addr -= 0x100;
                        }
                    }
                    8 => {
                        if current_watch.addr > 0x8000000F {
                            current_watch.addr -= 0x10;
                        }
                    }
                    9 => {
                        if current_watch.addr > 0x80000000 {
                            current_watch.addr -= 0x1;
                        }
                    }
                    11 => {
                        if current_watch.x > 0 {
                            current_watch.x -= 1;
                        }
                    }
                    15 => {
                        if current_watch.y > 0 {
                            current_watch.y -= 1;
                        }
                    }
                    19 => current_watch.hex = !current_watch.hex,
                    25 => {
                        current_watch.t = match current_watch.t {
                            Type::u8 => Type::String,
                            Type::i8 => Type::u8,
                            Type::u16 => Type::i8,
                            Type::i16 => Type::u16,
                            Type::u32 => Type::i16,
                            Type::i32 => Type::u32,
                            Type::f32 => Type::i32,
                            Type::String => Type::f32,
                        }
                    }
                    _ => {}
                }
            }
            ITEMS.borrow_mut().insert(cursor, current_watch);

            for (index, (line, content)) in lines
                .into_iter()
                .skip(4)
                .zip(ITEMS.borrow().iter())
                .enumerate()
            {
                if index == cursor {
                    let _ = write!(line.begin(), "> ");
                    let _ = write!(line.append(), "{:?}", content);
                } else {
                    let _ = write!(line.begin(), "  ");
                    let _ = write!(line.append(), "{:?}", content);
                }
            }
            let mut line = lines[cursor + 5].begin();
            for _ in 0..edit_cursor {
                let _ = write!(line, " ");
            }
            let _ = write!(line, "^");
        } else {
            if pressed_b {
                transition(MenuState::MainMenu);
                cursor = 0;
                edit_cursor = 2;
                return;
            }
            if pressed_a {
                if ITEMS.borrow().len() > 0 {
                    in_submenu = true;
                }
            }

            if controller::X.is_pressed() {
                ITEMS.borrow_mut().push(Watch::default());
            }
            if controller::Y.is_pressed() {
                let mut items = ITEMS.borrow_mut();
                if items.len() > 0 {
                    if cursor < items.len() {
                        items.remove(cursor);
                    }
                }
            }

            if cursor >= ITEMS.borrow().len() {
                cursor = ITEMS.borrow().len() - 1;
            }

            if controller::DPAD_UP.is_pressed() && cursor > 0 {
                cursor -= 1;
            } else if controller::DPAD_DOWN.is_pressed() && cursor + 1 < ITEMS.borrow().len() {
                cursor += 1;
            }

            for (index, (line, content)) in lines
                .into_iter()
                .skip(4)
                .zip(ITEMS.borrow().iter())
                .enumerate()
            {
                if index == cursor {
                    let _ = write!(line.begin(), "> ");
                } else {
                    let _ = write!(line.begin(), "  ");
                }

                let _ = write!(line.append(), "{:?}", content);
            }
        }
    }
}
