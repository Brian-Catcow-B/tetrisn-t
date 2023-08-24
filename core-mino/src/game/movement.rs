use crate::menu::menuhelpers::MenuItemTrigger;

use std::convert::TryFrom;

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Movement {
    Down,
    Left,
    Up,
    Right,
    RotateCw,
    RotateCcw,
    DoubleRotate,
    BoardCw,
    BoardCcw,
    None,
}

pub static CONVERSION_FAILED_MOVEMENT_FROM_U8: &str = "[!] Failed to get Movement value from u8";

impl TryFrom<u8> for Movement {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Movement::Down),
            1 => Ok(Movement::Left),
            2 => Ok(Movement::Up),
            3 => Ok(Movement::Right),
            4 => Ok(Movement::RotateCw),
            5 => Ok(Movement::RotateCcw),
            6 => Ok(Movement::DoubleRotate),
            7 => Ok(Movement::BoardCw),
            8 => Ok(Movement::BoardCcw),
            9 => Ok(Movement::None),
            _ => Err("Invalid u8 value"),
        }
    }
}

pub static CONVERSION_FAILED_MOVEMENT_FROM_MENUITEMTRIGGER: &str =
    "[!] Failed to get Movement value from MenuItemTrigger";

impl TryFrom<MenuItemTrigger> for Movement {
    type Error = &'static str;

    fn try_from(value: MenuItemTrigger) -> Result<Self, Self::Error> {
        match value {
            MenuItemTrigger::KeyLeft => Ok(Movement::Left),
            MenuItemTrigger::KeyRight => Ok(Movement::Right),
            MenuItemTrigger::KeyDown => Ok(Movement::Down),
            MenuItemTrigger::KeyRotateCw => Ok(Movement::RotateCw),
            MenuItemTrigger::KeyRotateCcw => Ok(Movement::RotateCcw),
            MenuItemTrigger::KeyBoardCw => Ok(Movement::BoardCw),
            MenuItemTrigger::KeyBoardCcw => Ok(Movement::BoardCcw),
            _ => Err("Invalid MenuItemTrigger value"),
        }
    }
}
