use crate::movement::Movement;
use ggez::event::KeyCode;

// (is pressed down, was pressed this frame)
pub struct Input {
    pub keydown_left: (bool, bool),
    pub keydown_right: (bool, bool),
    pub keydown_down: (bool, bool),
    pub keydown_up: (bool, bool),
    pub keydown_rotate_cw: (bool, bool),
    pub keydown_rotate_ccw: (bool, bool),
    pub keydown_board_cw: (bool, bool),
    pub keydown_board_ccw: (bool, bool),
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
            keydown_board_cw: (false, false),
            keydown_board_ccw: (false, false),
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
        self.keydown_board_cw.1 = false;
        self.keydown_board_ccw.1 = false;
        self.keydown_start.1 = false;
    }

    pub fn reset_all(&mut self) {
        self.keydown_left = (false, false);
        self.keydown_right = (false, false);
        self.keydown_down = (false, false);
        self.keydown_up = (false, false);
        self.keydown_rotate_cw = (false, false);
        self.keydown_rotate_ccw = (false, false);
        self.keydown_board_cw = (false, false);
        self.keydown_board_ccw = (false, false);
        self.keydown_start = (false, false);
    }

    pub fn _debug_print_inputs(&self) {
        println!("Left:  ({}, {})", self.keydown_left.0, self.keydown_left.1);
        println!(
            "Right: ({}, {})",
            self.keydown_right.0, self.keydown_right.1
        );
        println!("Down:  ({}, {})", self.keydown_down.0, self.keydown_down.1);
        println!("Up:    ({}, {})", self.keydown_up.0, self.keydown_up.1);
        println!(
            "RotateCw:    ({}, {})",
            self.keydown_rotate_cw.0, self.keydown_rotate_cw.1
        );
        println!(
            "RotateCcw:   ({}, {})",
            self.keydown_rotate_ccw.0, self.keydown_rotate_ccw.1
        );
        println!(
            "BoardCw:    ({}, {})",
            self.keydown_board_cw.0, self.keydown_board_cw.1
        );
        println!(
            "BoardCcw:   ({}, {})",
            self.keydown_board_ccw.0, self.keydown_board_ccw.1
        );
        println!(
            "Start: ({}, {})",
            self.keydown_start.0, self.keydown_start.1
        );
    }
}

#[derive(Clone)]
pub struct KeyboardControlScheme {
    pub vec_keycode_movement_pair: Vec<(KeyCode, Movement)>,
}

impl KeyboardControlScheme {
    pub fn copy(&self) -> Self {
        let mut copy_vec_keycode_movement_pair: Vec<(KeyCode, Movement)> =
            Vec::with_capacity(self.vec_keycode_movement_pair.capacity());
        for item in self.vec_keycode_movement_pair.iter() {
            copy_vec_keycode_movement_pair.push(*item);
        }
        Self {
            vec_keycode_movement_pair: copy_vec_keycode_movement_pair,
        }
    }

    pub fn len(&self) -> usize {
        self.vec_keycode_movement_pair.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec_keycode_movement_pair.is_empty()
    }

    pub fn new_classic(
        left: KeyCode,
        right: KeyCode,
        down: KeyCode,
        rotate_cw: KeyCode,
        rotate_ccw: KeyCode,
    ) -> Self {
        let mut vec_keycode_movement_pair: Vec<(KeyCode, Movement)> = Vec::with_capacity(5);
        vec_keycode_movement_pair.push((left, Movement::Left));
        vec_keycode_movement_pair.push((right, Movement::Right));
        vec_keycode_movement_pair.push((down, Movement::Down));
        vec_keycode_movement_pair.push((rotate_cw, Movement::RotateCw));
        vec_keycode_movement_pair.push((rotate_ccw, Movement::RotateCcw));
        Self {
            vec_keycode_movement_pair,
        }
    }

    pub fn new_rotatris(
        left: KeyCode,
        right: KeyCode,
        down: KeyCode,
        rotate_cw: KeyCode,
        rotate_ccw: KeyCode,
        rotate_board_cw: KeyCode,
        rotate_board_ccw: KeyCode,
    ) -> Self {
        let mut vec_keycode_movement_pair: Vec<(KeyCode, Movement)> = Vec::with_capacity(7);
        vec_keycode_movement_pair.push((left, Movement::Left));
        vec_keycode_movement_pair.push((right, Movement::Right));
        vec_keycode_movement_pair.push((down, Movement::Down));
        vec_keycode_movement_pair.push((rotate_cw, Movement::RotateCw));
        vec_keycode_movement_pair.push((rotate_ccw, Movement::RotateCcw));
        vec_keycode_movement_pair.push((rotate_board_cw, Movement::BoardCw));
        vec_keycode_movement_pair.push((rotate_board_ccw, Movement::BoardCcw));
        Self {
            vec_keycode_movement_pair,
        }
    }

    pub fn keycode_from_movement(&self, m: Movement) -> Option<KeyCode> {
        for pair in self.vec_keycode_movement_pair.iter() {
            if pair.1 == m {
                return Some(pair.0);
            }
        }

        None
    }

    pub fn movement_from_keycode(&self, k: KeyCode) -> Option<Movement> {
        for pair in self.vec_keycode_movement_pair.iter() {
            if pair.0 == k {
                return Some(pair.1);
            }
        }

        None
    }

    pub fn add_pair(&mut self, k: KeyCode, m: Movement) {
        self.vec_keycode_movement_pair.push((k, m));
    }
}

impl Default for KeyboardControlScheme {
    fn default() -> Self {
        let vec_keycode_movement_pair: Vec<(KeyCode, Movement)> = vec![];
        Self {
            vec_keycode_movement_pair,
        }
    }
}
