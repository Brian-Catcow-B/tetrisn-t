use crate::menu::menuhelpers::MenuItemTrigger;

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
    None,
}

impl From<u8> for Movement {
    fn from(value: u8) -> Movement {
        match value {
            0 => Movement::Down,
            1 => Movement::Left,
            2 => Movement::Up,
            3 => Movement::Right,
            4 => Movement::RotateCw,
            5 => Movement::RotateCcw,
            6 => Movement::DoubleRotate,
            7 => Movement::None,
            _ => panic!("[!] Unknown Movement value: {}", value),
        }
    }
}

impl From<MenuItemTrigger> for Movement {
    fn from(value: MenuItemTrigger) -> Movement {
        match value {
            MenuItemTrigger::KeyLeft => Movement::Left,
            MenuItemTrigger::KeyRight => Movement::Right,
            MenuItemTrigger::KeyDown => Movement::Down,
            MenuItemTrigger::KeyRotateCw => Movement::RotateCw,
            MenuItemTrigger::KeyRotateCcw => Movement::RotateCcw,
            MenuItemTrigger::KeyBoardCw => Movement::None,
            MenuItemTrigger::KeyBoardCcw => Movement::None,
            _ => panic!(
                "[!] Unexpected value converting MenuItemTrigger to Movement: {:?}",
                value
            ),
        }
    }
}
