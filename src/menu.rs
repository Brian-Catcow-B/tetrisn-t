use ggez::Context;
use ggez::event::{Button, Axis, KeyCode};
use ggez::graphics::{self, Color};

use crate::control::ProgramState;
use crate::inputs::{Input, KeyboardControlScheme, GamepadProfileScheme};

use crate::game::GameOptions;

pub const MAX_STARTING_LEVEL: u8 = 29; // this is just the fastest speed, so yeah
pub const MAX_NUM_PLAYERS: u8 = 62; // currently held back by board width being a u8 equal to 6 + 4 * num_players
pub const MAX_NUM_GAMEPAD_PROFILES: u8 = 9;

const DETECT_GAMEPAD_AXIS_THRESHOLD: f32 = 0.5;
const UNDETECT_GAMEPAD_AXIS_THRESHOLD: f32 = 0.3;

pub const TEXT_SCALE_DOWN: f32 = 15.0;
pub const SUB_TEXT_SCALE_DOWN: f32 = 25.0;
pub const MINI_TEXT_SCALE_DOWN: f32 = 30.0;

const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const DARK_GRAY: Color = Color::new(0.3, 0.3, 0.3, 1.0);
pub const LIGHT_GRAY: Color = Color::new(0.6, 0.6, 0.6, 1.0);
pub const SELECT_GREEN: Color = Color::new(0.153, 0.839, 0.075, 1.0);
pub const HELP_RED: Color = Color::new(0.9, 0.11, 0.11, 1.0);

mod start;
mod inputconfig;
use start::StartMenu;
use inputconfig::InputConfigMenu;

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum MenuState {
    Start,
    InputConfig,
}

// we just have all the menu stuffs loaded into here because they're all connected and it's not much memory anyways
pub struct Menu {
    // logic
    input: Input,
    // states
    state: MenuState,
    start_menu: start::StartMenu,
    input_config_menu: inputconfig::InputConfigMenu,
}

impl Menu {
    pub fn new(ctx: &mut Context, last_used_game_options: &Option<GameOptions>) -> Self {
        let window_dimensions = graphics::size(ctx);
        // get the last used options if there are any
        let mut arr_controls: Vec<(u8, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)> = vec![];
        let mut num_players: u8 = 1;
        let mut starting_level: u8 = 0;
        if let Some(game_options) = last_used_game_options {
            for (player, controls) in game_options.vec_keyboard_inputs.iter().enumerate() {
                arr_controls.push(
                    (player as u8,
                    Some(controls.left),
                    Some(controls.right),
                    Some(controls.down),
                    Some(controls.rotate_cw),
                    Some(controls.rotate_ccw),)
                );
            }
            num_players = game_options.num_players;
            starting_level = game_options.starting_level;
        }
        Self {
            input: Input::new(),
            state: MenuState::Start,
            start_menu: StartMenu::new(window_dimensions, num_players, starting_level),
            input_config_menu: InputConfigMenu::new(window_dimensions, arr_controls),
        }
    }

    pub fn update(&mut self) -> Option<(ProgramState, GameOptions)> {
        match self.state {
            MenuState::Start => {
                let ret_bools: (bool, bool) = self.start_menu.update(&self.input);
                if ret_bools.0 {
                    self.input_config_menu.arr_controls.sort_by_key(|ctrls| ctrls.0);
                    let mut vec_control_scheme: Vec<KeyboardControlScheme> = Vec::with_capacity(self.input_config_menu.arr_controls.len());
                    let mut arr_profile_schemes: [Option<GamepadProfileScheme>; MAX_NUM_GAMEPAD_PROFILES as usize] = [None; MAX_NUM_GAMEPAD_PROFILES as usize];
                    // TODO: use a closure if that's better. It's too late at night for me to figure this out; I just want this to work; I've written ~500 lines of GUI code today; help
                    for controls in self.input_config_menu.arr_controls.iter() {
                        if let Some(ctrls) = controls.0 {
                            vec_control_scheme.push(KeyboardControlScheme::new(
                                ctrls.0.expect("[!] key left set to None"),
                                ctrls.1.expect("[!] key right set to None"),
                                ctrls.2.expect("[!] key down set to None"),
                                ctrls.3.expect("[!] key rotate_cw set to None"),
                                ctrls.4.expect("[!] key rotate_ccw set to None"),
                                KeyCode::Escape,
                            ));
                        }
                    }
                    for (idx, opt_profile) in self.input_config_menu.arr_gamepad_profiles.iter().enumerate() {
                        if let Some(profile) = opt_profile {
                            arr_profile_schemes[idx] = Some(GamepadProfileScheme::new(
                                ((profile.0).0, (profile.0).1),
                                ((profile.1).0, (profile.1).1),
                                ((profile.2).0, (profile.2).1),
                                (profile.3).expect("[!] RotateCw was unexpectedly set to None"),
                                (profile.4).expect("[!] RotateCcw was unexpectedly set to None"),
                                (profile.5).expect("[!] Start was unexpectedly set to None"),
                            ));
                        }
                    }
                    if vec_control_scheme.len() < self.start_menu.num_players as usize {
                        self.start_menu.not_enough_controls_flag = true;
                    } else {
                        return Some((ProgramState::Game, GameOptions::new(self.start_menu.num_players, self.start_menu.starting_level, vec_control_scheme, arr_profile_schemes)));
                    }
                } else if ret_bools.1 {
                    self.state = MenuState::InputConfig;
                }
            }
            MenuState::InputConfig => {
                if self.input_config_menu.update(&self.input) {
                    self.state = MenuState::Start;
                }
            }
        }

        self.input.was_just_pressed_setfalse();
        None
    }

    pub fn key_down_event(&mut self, keycode: KeyCode, _repeat: bool) {
        self.input_config_menu.most_recently_pressed_key = Some(keycode);
        if keycode == KeyCode::Left {
            if !self.input.keydown_left.0 {
                self.input.keydown_left = (true, true);
            }
        } else if keycode == KeyCode::Right {
            if !self.input.keydown_right.0 {
                self.input.keydown_right = (true, true);
            }
        } else if keycode == KeyCode::Down {
            if !self.input.keydown_down.0 {
                self.input.keydown_down = (true, true);
            }
        } else if keycode == KeyCode::Up {
            if !self.input.keydown_up.0 {
                self.input.keydown_up = (true, true);
            }
        } else if keycode == KeyCode::G {
            if !self.input.keydown_rotate_cw.0 {
                self.input.keydown_rotate_cw = (true, true);
            }
        // } else if keycode == KeyCode:: {
        //     if !self.input.keydown_rotate_ccw.0 {
        //         self.input.keydown_rotate_ccw = (true, true);
        //     }
        } else if keycode == KeyCode::Space || keycode == KeyCode::Return {
            if !self.input.keydown_start.0 {
                self.input.keydown_start = (true, true);
            }
        }
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        if keycode == KeyCode::Left {
            self.input.keydown_left = (false, false);
        } else if keycode == KeyCode::Right {
            self.input.keydown_right = (false, false);
        } else if keycode == KeyCode::Down {
            self.input.keydown_down = (false, false);
        } else if keycode == KeyCode::Up {
            self.input.keydown_up = (false, false);
        } else if keycode == KeyCode::G {
            self.input.keydown_rotate_cw = (false, false);
        // } else if keycode == KeyCode:: {
        //     self.input.keydown_rotate_ccw = (false, false);
        } else if keycode == KeyCode::Space || keycode == KeyCode::Return {
            self.input.keydown_start = (false, false);
        }
    }

    pub fn gamepad_button_down_event(&mut self, btn: Button) {
        self.input_config_menu.most_recently_pressed_gamepad_button = Some(btn);
        println!("just set most_recently_pressed_gamepad_button to Some({:?})", btn);
    }

    pub fn gamepad_axis_event(&mut self, axis: Axis, value: f32) {
        if !self.input_config_menu.gamepad_axis_wait.0 {
            if value < -DETECT_GAMEPAD_AXIS_THRESHOLD {
                self.input_config_menu.gamepad_axis_wait = (true, Some((axis, if value < 0.0 {false} else {true})));
                self.input_config_menu.most_recently_pressed_gamepad_axis = Some((axis, false));
                println!("just set most_recently_pressed_gamepad_axis to Some(({:?}, false))", axis);
            } else if value > DETECT_GAMEPAD_AXIS_THRESHOLD {
                self.input_config_menu.gamepad_axis_wait = (true, Some((axis, if value < 0.0 {false} else {true})));
                self.input_config_menu.most_recently_pressed_gamepad_axis = Some((axis, true));
                println!("just set most_recently_pressed_gamepad_axis to Some(({:?}, true))", axis);
            }
        } else if value < UNDETECT_GAMEPAD_AXIS_THRESHOLD && value > -UNDETECT_GAMEPAD_AXIS_THRESHOLD && (self.input_config_menu.gamepad_axis_wait.1).expect("[!] axis waiting on None").0 == axis {
            self.input_config_menu.gamepad_axis_wait = (false, None);
            println!("set false");
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, GRAY);

        match self.state {
            MenuState::Start => self.start_menu.draw(ctx),
            MenuState::InputConfig => self.input_config_menu.draw(ctx),
        }
    }
}