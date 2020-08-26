use ggez::event::KeyCode;
use rand::random;
use crate::inputs::{Input, ControlScheme};
use crate::game::piece::Shapes;
use crate::game::{FORCE_FALL_DELAY, DAS_THRESHOLD_BIG, INITIAL_HANG_FRAMES};

pub const SPAWN_DELAY: i16 = 20i16;

pub struct Player {
    pub player_num: u8,
    pub control_scheme: ControlScheme,
    pub input: Input,
    pub spawn_piece_flag: bool,
    pub spawn_column: u8,
    pub spawn_delay: i16,
    pub next_piece_shape: Shapes,
    pub redraw_next_piece_flag: bool,
    pub fall_countdown: u8,
    pub force_fall_countdown: u8,
    pub das_countdown: u8,
    pub waiting_to_shift: bool,
}

impl Player {
    pub fn new(player_num: u8, control_scheme: ControlScheme, spawn_column: u8) -> Self {
        Self {
            player_num,
            control_scheme,
            input: Input::new(),
            spawn_piece_flag: true,
            spawn_column,
            spawn_delay: SPAWN_DELAY,
            next_piece_shape: Shapes::from_u8(random::<u8>() % 7),
            redraw_next_piece_flag: true,
            fall_countdown: INITIAL_HANG_FRAMES,
            force_fall_countdown: FORCE_FALL_DELAY,
            das_countdown: DAS_THRESHOLD_BIG,
            waiting_to_shift: false,
        }
    }

    pub fn update_input_keydown(&mut self, input: KeyCode) -> bool {
        if input == self.control_scheme.left {
            if !self.input.keydown_left.0 {
                self.input.keydown_left = (true, true);
                // for auto-shift reasons and controller reasons...
                self.input.keydown_right.0 = false;
                return true;
            }
        } else if input == self.control_scheme.right {
            if !self.input.keydown_right.0 {
                self.input.keydown_right = (true, true);
                // for auto-shift reasons and controller reasons...
                self.input.keydown_left.0 = false;
                return true;
            }
        } else if input == self.control_scheme.down {
            if !self.input.keydown_down.0 {
                self.input.keydown_down = (true, true);
                return true;
            }
        } else if input == self.control_scheme.rotate_cw {
            if !self.input.keydown_rotate_cw.0 {
                self.input.keydown_rotate_cw = (true, true);
                return true;
            }
        } else if input == self.control_scheme.rotate_ccw {
            if !self.input.keydown_rotate_ccw.0 {
                self.input.keydown_rotate_ccw = (true, true);
                return true;
            }
        } else if input == self.control_scheme.start {
            if !self.input.keydown_start.0 {
                self.input.keydown_start = (true, true);
                return true;
            }
        }

        false
    }

pub fn update_input_keyup(&mut self, input: KeyCode) -> bool {
        if input == self.control_scheme.left {
            // for auto-shift reasons
            if self.input.keydown_left.0 {
                self.das_countdown = DAS_THRESHOLD_BIG;
                self.waiting_to_shift = false;
            }
            self.input.keydown_left = (false, false);
            return true;
        } else if input == self.control_scheme.right {
            // for auto-shift reasons
            if self.input.keydown_right.0 {
                self.das_countdown = DAS_THRESHOLD_BIG;
                self.waiting_to_shift = false;
            }
            self.input.keydown_right = (false, false);
            return true;
        } else if input == self.control_scheme.down {
            self.input.keydown_down = (false, false);
            return true;
        } else if input == self.control_scheme.rotate_cw {
            self.input.keydown_rotate_cw = (false, false);
            return true;
        } else if input == self.control_scheme.rotate_ccw {
            self.input.keydown_rotate_ccw = (false, false);
            return true;
        } else if input == self.control_scheme.start {
            self.input.keydown_start = (false, false);
            return true;
        }

        false
    }
}