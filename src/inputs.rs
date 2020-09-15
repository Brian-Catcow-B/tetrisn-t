use ggez::event::{KeyCode, Axis, Button};

// (is pressed down, was pressed this frame)
pub struct Input {
    pub keydown_left: (bool, bool),
    pub keydown_right: (bool, bool),
    pub keydown_down: (bool, bool),
    pub keydown_up: (bool, bool),
    pub keydown_rotate_cw: (bool, bool),
    pub keydown_rotate_ccw: (bool, bool),
    pub keydown_start: (bool, bool),
}

impl Input {
    pub fn new() -> Self {
        Self {
            keydown_left: (false, false),
            keydown_right: (false, false),
            keydown_down: (false, false),
            keydown_up: (false, false),
            keydown_rotate_cw: (false, false),
            keydown_rotate_ccw: (false, false),
            keydown_start: (false, false),
        }
    }

    pub fn was_just_pressed_setfalse(&mut self) {
        self.keydown_left.1 = false;
        self.keydown_right.1 = false;
        self.keydown_down.1 = false;
        self.keydown_up.1 = false;
        self.keydown_rotate_cw.1 = false;
        self.keydown_rotate_ccw.1 = false;
        self.keydown_start.1 = false;
    }

    pub fn reset_all(&mut self) {
        self.keydown_left = (false, false);
        self.keydown_right = (false, false);
        self.keydown_down = (false, false);
        self.keydown_up = (false, false);
        self.keydown_rotate_cw = (false, false);
        self.keydown_rotate_ccw = (false, false);
        self.keydown_start = (false, false);
    }

    pub fn _print_inputs(&self) {
        println!("Left:  ({}, {})", self.keydown_left.0, self.keydown_left.1);
        println!("Right: ({}, {})", self.keydown_right.0, self.keydown_right.1);
        println!("Down:  ({}, {})", self.keydown_down.0, self.keydown_down.1);
        println!("Up:    ({}, {})", self.keydown_up.0, self.keydown_up.1);
        println!("Cw:    ({}, {})", self.keydown_rotate_cw.0, self.keydown_rotate_cw.1);
        println!("Ccw:   ({}, {})", self.keydown_rotate_ccw.0, self.keydown_rotate_ccw.1);
        println!("Start: ({}, {})", self.keydown_start.0, self.keydown_start.1);
    }
}

#[derive(Copy, Clone)]
pub struct KeyboardControlScheme {
    pub left: KeyCode,
    pub right: KeyCode,
    pub down: KeyCode,
    pub rotate_cw: KeyCode,
    pub rotate_ccw: KeyCode,
    pub start: KeyCode,
}

impl KeyboardControlScheme {
    pub fn new(
        left: KeyCode,
        right: KeyCode,
        down: KeyCode,
        rotate_cw: KeyCode,
        rotate_ccw: KeyCode,
        start: KeyCode,
    ) -> Self {
        Self {
            left,
            right,
            down,
            rotate_cw,
            rotate_ccw,
            start,
        }
    }
}

#[derive(Copy, Clone)]
pub struct GamepadControlScheme {
    pub left: (Option<Button>, Option<(Axis, bool)>),
    pub right: (Option<Button>, Option<(Axis, bool)>),
    pub down: (Option<Button>, Option<(Axis, bool)>),
    pub rotate_cw: Button,
    pub rotate_ccw: Button,
    pub start: Button,
}

impl GamepadControlScheme {
    pub fn new(
        left: (Option<Button>, Option<(Axis, bool)>),
        right: (Option<Button>, Option<(Axis, bool)>),
        down: (Option<Button>, Option<(Axis, bool)>),
        rotate_cw: Button,
        rotate_ccw: Button,
        start: Button,
    ) -> Self {
        Self {
            left,
            right,
            down,
            rotate_cw,
            rotate_ccw,
            start,
        }
    }
}