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
