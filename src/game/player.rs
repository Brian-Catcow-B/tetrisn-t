use ggez::event::{Axis, Button, KeyCode};
use rand::random;

use crate::game::piece::Shapes;
use crate::game::{DAS_THRESHOLD_BIG, FORCE_FALL_DELAY, INITIAL_HANG_FRAMES};
use crate::game::{DETECT_GAMEPAD_AXIS_THRESHOLD, UNDETECT_GAMEPAD_AXIS_THRESHOLD};
use crate::inputs::{Input, KeyboardControlScheme};
use crate::movement::Movement;

pub const SPAWN_DELAY: i16 = 20i16;

pub struct Player {
    pub player_num: u8,
    pub control_scheme: (Option<KeyboardControlScheme>, bool),
    axis_wait_for_unpress_x: bool,
    axis_wait_for_unpress_y: bool,
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
    pub fn new(
        player_num: u8,
        control_scheme: (Option<KeyboardControlScheme>, bool),
        spawn_column: u8,
    ) -> Self {
        Self {
            player_num,
            control_scheme,
            axis_wait_for_unpress_x: false,
            axis_wait_for_unpress_y: false,
            input: Input::new(),
            spawn_piece_flag: true,
            spawn_column,
            spawn_delay: SPAWN_DELAY,
            next_piece_shape: {
                let mut rand: u8;
                loop {
                    rand = random::<u8>();
                    if rand < 252 {
                        break;
                    }
                }
                Shapes::from(rand % 7)
            },
            redraw_next_piece_flag: true,
            fall_countdown: INITIAL_HANG_FRAMES,
            force_fall_countdown: FORCE_FALL_DELAY,
            das_countdown: DAS_THRESHOLD_BIG,
            waiting_to_shift: false,
        }
    }

    pub fn tick_das_countdown(&mut self) -> bool {
        if self.das_countdown > 0 {
            self.das_countdown -= 1;
        }
        if self.das_countdown == 0 || self.waiting_to_shift {
            return true;
        }

        false
    }

    pub fn update_input_keydown(&mut self, input: KeyCode) -> bool {
        if let Some(k_ctrl_scheme) = &self.control_scheme.0 {
            let movement_opt = k_ctrl_scheme.movement_from_keycode(input);
            if let Some(movement) = movement_opt {
                match movement {
                    Movement::Down => {
                        if !self.input.keydown_down.0 {
                            self.input.keydown_down = (true, true);
                            return true;
                        }
                    }
                    Movement::Left => {
                        if !self.input.keydown_left.0 {
                            self.input.keydown_left = (true, true);
                            // for auto-shift reasons and controller reasons...
                            self.input.keydown_right.0 = false;
                            return true;
                        }
                    }
                    Movement::Up => {
                        if !self.input.keydown_up.0 {
                            self.input.keydown_up = (true, true);
                            return true;
                        }
                    }
                    Movement::Right => {
                        if !self.input.keydown_right.0 {
                            self.input.keydown_right = (true, true);
                            // for auto-shift reasons and controller reasons...
                            self.input.keydown_left.0 = false;
                            return true;
                        }
                    }
                    Movement::RotateCw => {
                        if !self.input.keydown_rotate_cw.0 {
                            self.input.keydown_rotate_cw = (true, true);
                            return true;
                        }
                    }
                    Movement::RotateCcw => {
                        if !self.input.keydown_rotate_ccw.0 {
                            self.input.keydown_rotate_ccw = (true, true);
                            return true;
                        }
                    }
                    Movement::BoardCw => {
                        if !self.input.keydown_board_cw.0 {
                            self.input.keydown_board_cw = (true, true);
                            return true;
                        }
                    }
                    Movement::BoardCcw => {
                        if !self.input.keydown_board_ccw.0 {
                            self.input.keydown_board_ccw = (true, true);
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }

        false
    }

    pub fn update_input_keyup(&mut self, input: KeyCode) -> bool {
        if let Some(k_ctrl_scheme) = &self.control_scheme.0 {
            let movement_opt = k_ctrl_scheme.movement_from_keycode(input);
            if let Some(movement) = movement_opt {
                match movement {
                    Movement::Down => {
                        self.input.keydown_down = (false, false);
                        return true;
                    }
                    Movement::Left => {
                        // for auto-shift reasons
                        if self.input.keydown_left.0 {
                            self.das_countdown = DAS_THRESHOLD_BIG;
                            self.waiting_to_shift = false;
                        }
                        self.input.keydown_left = (false, false);
                        return true;
                    }
                    Movement::Up => {
                        self.input.keydown_up = (false, false);
                        return true;
                    }
                    Movement::Right => {
                        // for auto-shift reasons
                        if self.input.keydown_right.0 {
                            self.das_countdown = DAS_THRESHOLD_BIG;
                            self.waiting_to_shift = false;
                        }
                        self.input.keydown_right = (false, false);
                        return true;
                    }
                    Movement::RotateCw => {
                        self.input.keydown_rotate_cw = (false, false);
                        return true;
                    }
                    Movement::RotateCcw => {
                        self.input.keydown_rotate_ccw = (false, false);
                        return true;
                    }
                    Movement::BoardCw => {
                        self.input.keydown_board_cw = (false, false);
                        return true;
                    }
                    Movement::BoardCcw => {
                        self.input.keydown_board_ccw = (false, false);
                        return true;
                    }
                    _ => {}
                }
            }
        }

        false
    }

    pub fn update_input_buttondown(&mut self, btn: Button) {
        if btn == Button::DPadLeft {
            self.input.keydown_left = (true, true);
            self.input.keydown_right = (false, false);
        } else if btn == Button::DPadRight {
            self.input.keydown_right = (true, true);
            self.input.keydown_left = (false, false);
        } else if btn == Button::DPadDown {
            self.input.keydown_down = (true, true);
        } else if btn == Button::East {
            self.input.keydown_rotate_cw = (true, true);
        } else if btn == Button::South {
            self.input.keydown_rotate_ccw = (true, true);
        } else if btn == Button::Start {
            self.input.keydown_start = (true, true);
        }
    }

    pub fn update_input_buttonup(&mut self, btn: Button) {
        if btn == Button::DPadLeft {
            // for auto-shift reasons
            if self.input.keydown_left.0 {
                self.das_countdown = DAS_THRESHOLD_BIG;
                self.waiting_to_shift = false;
            }
            self.input.keydown_left = (false, false);
        } else if btn == Button::DPadRight {
            // for auto-shift reasons
            if self.input.keydown_right.0 {
                self.das_countdown = DAS_THRESHOLD_BIG;
                self.waiting_to_shift = false;
            }
            self.input.keydown_right = (false, false);
        } else if btn == Button::DPadDown {
            self.input.keydown_down = (false, false);
        } else if btn == Button::East {
            self.input.keydown_rotate_cw = (false, false);
        } else if btn == Button::South {
            self.input.keydown_rotate_ccw = (false, false);
        } else if btn == Button::Start {
            self.input.keydown_start = (false, false);
        }
    }

    pub fn update_input_axis(&mut self, axis: Axis, value: f32) {
        if axis == Axis::LeftStickX {
            // left and right
            if !self.axis_wait_for_unpress_x && value < -DETECT_GAMEPAD_AXIS_THRESHOLD {
                // press left
                self.axis_wait_for_unpress_x = true;
                self.input.keydown_left = (true, true);
                self.input.keydown_right = (false, false);
            } else if !self.axis_wait_for_unpress_x && value > DETECT_GAMEPAD_AXIS_THRESHOLD {
                // press right
                self.axis_wait_for_unpress_x = true;
                self.input.keydown_right = (true, true);
                self.input.keydown_left = (false, false);
            } else if self.axis_wait_for_unpress_x
                && value < UNDETECT_GAMEPAD_AXIS_THRESHOLD
                && value > -UNDETECT_GAMEPAD_AXIS_THRESHOLD
            {
                // unpress left and right
                self.das_countdown = DAS_THRESHOLD_BIG;
                self.waiting_to_shift = false;
                self.axis_wait_for_unpress_x = false;
                self.input.keydown_left = (false, false);
                self.input.keydown_right = (false, false);
            }
        } else if axis == Axis::LeftStickY {
            // down
            if !self.axis_wait_for_unpress_y && value < -DETECT_GAMEPAD_AXIS_THRESHOLD {
                // press down
                self.axis_wait_for_unpress_y = true;
                self.input.keydown_down = (true, true);
            } else if self.axis_wait_for_unpress_y && value > -UNDETECT_GAMEPAD_AXIS_THRESHOLD {
                // unpress down
                self.axis_wait_for_unpress_y = false;
                self.input.keydown_down = (false, false);
            }
        }
    }
}
