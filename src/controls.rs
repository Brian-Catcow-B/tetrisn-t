use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods};
use crate::piece;

// (is pressed down, was unpressed previous frame)
pub struct Input {
    pub keydown_left: (bool, bool),
    pub keydown_right: (bool, bool),
    pub keydown_down: (bool, bool),
    pub keydown_rotate_cw: (bool, bool),
    pub keydown_rotate_ccw: (bool, bool),
}

impl Input {
    pub fn new() -> Self {
        Self {
            keydown_left: (false, false),
            keydown_right: (false, false),
            keydown_down: (false, false),
            keydown_rotate_cw: (false, false),
            keydown_rotate_ccw: (false, false),
        }
    }

    pub fn was_unpressed_previous_frame_setfalse(&mut self) {
        self.keydown_left.1 = false;
        self.keydown_right.1 = false;
        self.keydown_down.1 = false;
        self.keydown_rotate_cw.1 = false;
        self.keydown_rotate_ccw.1 = false;
    }
}

pub struct ControlScheme {
    left: KeyCode,
    right: KeyCode,
    down: KeyCode,
    rotate_cw: KeyCode,
    rotate_ccw: KeyCode,
}

impl ControlScheme {
    pub fn new(left: KeyCode, right: KeyCode,down: KeyCode, rotate_cw: KeyCode, rotate_ccw: KeyCode) -> Self {
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