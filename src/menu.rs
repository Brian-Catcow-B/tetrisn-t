use ggez::Context;
use ggez::event::{Button, Axis, KeyCode};
use ggez::graphics::{self, Color};

use crate::control::ProgramState;
use crate::inputs::{Input, KeyboardControlScheme};

pub const MAX_STARTING_LEVEL: u8 = 29; // this is just the fastest speed, so yeah
pub const MAX_NUM_PLAYERS: u8 = 62; // currently held back by board width being a u8 equal to 6 + 4 * num_players

pub const TEXT_SCALE_DOWN: f32 = 15.0;
pub const SUB_TEXT_SCALE_DOWN: f32 = 25.0;

const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const DARK_GRAY: Color = Color::new(0.3, 0.3, 0.3, 1.0);
pub const LIGHT_GRAY: Color = Color::new(0.6, 0.6, 0.6, 1.0);
pub const SELECT_GREEN: Color = Color::new(0.153, 0.839, 0.075, 1.0);
pub const HELP_RED: Color = Color::new(0.9, 0.11, 0.11, 1.0);

mod start;
mod inputconfig;
use start::StartMenu;
use inputconfig::InputConfigMenu;

pub struct MenuGameOptions {
    pub num_players: u8,
    pub starting_level: u8,
    pub arr_controls: [(Option<KeyboardControlScheme>, bool); MAX_NUM_PLAYERS as usize],
}

impl MenuGameOptions {
    fn new(
        num_players: u8,
        starting_level: u8,
        arr_split_controls: [(Option<(Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>, bool); MAX_NUM_PLAYERS as usize],
    ) -> Self {
        let mut arr_controls: [(Option<KeyboardControlScheme>, bool); MAX_NUM_PLAYERS as usize] = [(None, false); MAX_NUM_PLAYERS as usize];
        for (idx, ctrls) in arr_split_controls.iter().enumerate() {
            if let Some(k_ctrls) = ctrls.0 {
                arr_controls[idx] = (Some(KeyboardControlScheme::new(
                    k_ctrls.0.expect("[!] attempted to create KeyboardControlScheme with Left == None"),
                    k_ctrls.1.expect("[!] attempted to create KeyboardControlScheme with Right == None"),
                    k_ctrls.2.expect("[!] attempted to create KeyboardControlScheme with Down == None"),
                    k_ctrls.3.expect("[!] attempted to create KeyboardControlScheme with RotateCw == None"),
                    k_ctrls.4.expect("[!] attempted to create KeyboardControlScheme with RotateCcw == None"),
                    KeyCode::Escape,
                )), false);
            } else if ctrls.1 {
                arr_controls[idx] = (None, true);
            }
        }
        Self {
            num_players,
            starting_level,
            arr_controls,
        }
    }
}

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
    pub fn new(ctx: &mut Context, last_used_game_options: &Option<MenuGameOptions>) -> Self {
        let window_dimensions = graphics::size(ctx);
        if let Some(menu_game_options) = last_used_game_options {
            // previously used
            Self {
                input: Input::new(),
                state: MenuState::Start,
                start_menu: StartMenu::new(window_dimensions, menu_game_options.num_players, menu_game_options.starting_level),
                input_config_menu: InputConfigMenu::new(window_dimensions, menu_game_options.arr_controls),
            }
        } else {
            // defaults
            Self {
                input: Input::new(),
                state: MenuState::Start,
                start_menu: StartMenu::new(window_dimensions, 1, 0),
                input_config_menu: InputConfigMenu::new(window_dimensions, [(None, false); MAX_NUM_PLAYERS as usize]),
            }
        }
    }

    pub fn update(&mut self) -> Option<(ProgramState, MenuGameOptions)> {
        match self.state {
            MenuState::Start => {
                let ret_bools: (bool, bool) = self.start_menu.update(&self.input);
                if ret_bools.0 {
                    if self.ensure_enough_controls() {
                        return Some((ProgramState::Game, MenuGameOptions::new(self.start_menu.num_players, self.start_menu.starting_level, self.input_config_menu.arr_split_controls)));
                    } else {
                        self.start_menu.not_enough_controls_flag = true;
                    }
                } else if ret_bools.1 {
                    // we are entering the InputConfig menu
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

    fn ensure_enough_controls(&self) -> bool {
        let mut ctrls_count = 0;
        for ctrls in self.input_config_menu.arr_split_controls.iter() {
            if ctrls.0.is_some() || ctrls.1 {
                ctrls_count += 1;
            }
        }
        ctrls_count >= self.start_menu.num_players
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
        // self.input_config_menu.most_recently_pressed_gamepad_button = Some(btn);
        // println!("just set most_recently_pressed_gamepad_button to Some({:?})", btn);
    }

    pub fn gamepad_axis_event(&mut self, axis: Axis, value: f32) {
        // if !self.input_config_menu.gamepad_axis_wait.0 {
        //     if value < -DETECT_GAMEPAD_AXIS_THRESHOLD {
        //         self.input_config_menu.gamepad_axis_wait = (true, Some((axis, if value < 0.0 {false} else {true})));
        //         self.input_config_menu.most_recently_pressed_gamepad_axis = Some((axis, false));
        //         println!("just set most_recently_pressed_gamepad_axis to Some(({:?}, false))", axis);
        //     } else if value > DETECT_GAMEPAD_AXIS_THRESHOLD {
        //         self.input_config_menu.gamepad_axis_wait = (true, Some((axis, if value < 0.0 {false} else {true})));
        //         self.input_config_menu.most_recently_pressed_gamepad_axis = Some((axis, true));
        //         println!("just set most_recently_pressed_gamepad_axis to Some(({:?}, true))", axis);
        //     }
        // } else if value < UNDETECT_GAMEPAD_AXIS_THRESHOLD && value > -UNDETECT_GAMEPAD_AXIS_THRESHOLD && (self.input_config_menu.gamepad_axis_wait.1).expect("[!] axis waiting on None").0 == axis {
        //     self.input_config_menu.gamepad_axis_wait = (false, None);
        //     println!("set false");
        // }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, GRAY);

        match self.state {
            MenuState::Start => self.start_menu.draw(ctx),
            MenuState::InputConfig => self.input_config_menu.draw(ctx),
        }
    }
}