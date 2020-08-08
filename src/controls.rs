use ggez::event::KeyCode;

// (is pressed down, was pressed this frame)
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

    pub fn was_just_pressed_setfalse(&mut self) {
        self.keydown_left.1 = false;
        self.keydown_right.1 = false;
        self.keydown_down.1 = false;
        self.keydown_rotate_cw.1 = false;
        self.keydown_rotate_ccw.1 = false;
    }

    pub fn _print_inputs(&self) {
        println!("Left:  ({}, {})", self.keydown_left.0, self.keydown_left.1);
        println!("Right: ({}, {})", self.keydown_right.0, self.keydown_right.1);
        println!("Down:  ({}, {})", self.keydown_down.0, self.keydown_down.1);
        println!("Cw:    ({}, {})", self.keydown_rotate_cw.0, self.keydown_rotate_cw.1);
        println!("Ccw:   ({}, {})", self.keydown_rotate_ccw.0, self.keydown_rotate_ccw.1);
    }
}

pub struct ControlScheme {
    pub left: KeyCode,
    pub right: KeyCode,
    pub down: KeyCode,
    pub rotate_cw: KeyCode,
    pub rotate_ccw: KeyCode,
}

impl ControlScheme {
    pub fn new(left: KeyCode, right: KeyCode,down: KeyCode, rotate_cw: KeyCode, rotate_ccw: KeyCode) -> Self {
        Self {
            left,
            right,
            down,
            rotate_cw,
            rotate_ccw,
        }
    }
}