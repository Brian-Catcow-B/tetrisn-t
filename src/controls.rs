use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods};
pub mod piece;

pub struct ControlScheme {
    left: KeyCode,
    right: KeyCode,
    down: KeyCode,
    rotate_cw: KeyCode,
    rotate_ccw: KeyCode,
}

impl ControlScheme {
    pub fn new(player: u8, left: KeyCode, right: KeyCode,down: KeyCode, rotate_cw: KeyCode, rotate_ccw: KeyCode) -> Self {
        Self {
            left: left,
            right: right,
            down: down,
            rotate_cw: rotate_cw,
            rotate_ccw: rotate_ccw,
        }
    }

    pub fn find_move(&self, input: KeyCode) -> piece::Movement {
        if input == self.left {
            return piece::Movement::Left;
        } else if input == self.right {
            return piece::Movement::Right;
        } else if input == self.down {
            return piece::Movement::Down;
        } else if input == self.rotate_cw {
            return piece::Movement::RotateCw;
        } else if input == self.rotate_ccw {
            return piece::Movement::RotateCcw;
        } else {
            return piece::Movement::None;
        }
    }
}