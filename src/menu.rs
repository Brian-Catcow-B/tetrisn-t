use ggez::Context;
use ggez::event::{Button, Axis, KeyCode};
use ggez::graphics::{self, Color};

use crate::control::ProgramState;
use crate::inputs::{Input, KeyboardControlScheme, GamepadProfileScheme};

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

pub struct MenuGameOptions {
    pub num_players: u8,
    pub starting_level: u8,
    pub arr_controls: [(Option<KeyboardControlScheme>, Option<u8>); MAX_NUM_PLAYERS as usize],
    pub arr_gamepad_profiles: [Option<GamepadProfileScheme>; MAX_NUM_GAMEPAD_PROFILES as usize],
}

impl MenuGameOptions {
    fn new(
        num_players: u8,
        starting_level: u8,
        arr_split_controls: [(Option<(Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>, Option<u8>); MAX_NUM_PLAYERS as usize],
        arr_split_gamepad_profiles: [Option<((Option<Button>, Option<(Axis, bool)>), (Option<Button>, Option<(Axis, bool)>), (Option<Button>, Option<(Axis, bool)>), Option<Button>, Option<Button>, Option<Button>)>; MAX_NUM_GAMEPAD_PROFILES as usize],
    ) -> Self {
        let mut arr_controls: [(Option<KeyboardControlScheme>, Option<u8>); MAX_NUM_PLAYERS as usize] = [(None, None); MAX_NUM_PLAYERS as usize];
        let mut arr_gamepad_profiles: [Option<GamepadProfileScheme>; MAX_NUM_GAMEPAD_PROFILES as usize] = [None; MAX_NUM_GAMEPAD_PROFILES as usize];
        for (idx, ctrls) in arr_split_controls.iter().enumerate() {
            if let Some(k_ctrls) = ctrls.0 {
                arr_controls[idx] = (Some(KeyboardControlScheme::new(
                    k_ctrls.0.expect("[!] attempted to create KeyboardControlScheme with Left == None"),
                    k_ctrls.1.expect("[!] attempted to create KeyboardControlScheme with Right == None"),
                    k_ctrls.2.expect("[!] attempted to create KeyboardControlScheme with Down == None"),
                    k_ctrls.3.expect("[!] attempted to create KeyboardControlScheme with RotateCw == None"),
                    k_ctrls.4.expect("[!] attempted to create KeyboardControlScheme with RotateCcw == None"),
                    KeyCode::Escape,
                )), None);
            } else if let Some(profile_num) = ctrls.1 {
                arr_controls[idx] = (None, Some(profile_num));
            }
        }
        for (idx, profile_opt) in arr_split_gamepad_profiles.iter().enumerate() {
            if let Some(profile) = profile_opt {
                arr_gamepad_profiles[idx] = Some(GamepadProfileScheme::new(
                    profile.0,
                    profile.1,
                    profile.2,
                    (profile.3).expect("[!] attempted to create GamepadProfileScheme with RotateCw == None"),
                    (profile.4).expect("[!] attempted to create GamepadProfileScheme with RotateCcw == None"),
                    (profile.5).expect("[!] attempted to create GamepadProfileScheme with Start == None"),
                ));
            }
        }
        Self {
            num_players,
            starting_level,
            arr_controls,
            arr_gamepad_profiles,
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
        // defaults
        // let mut vec_keyboard_controls: Vec<(u8, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)> = vec![];
        // let mut arr_profile_schemes: [Option<((Option<Button>, Option<(Axis, bool)>), (Option<Button>, Option<(Axis, bool)>), (Option<Button>, Option<(Axis, bool)>), Option<Button>, Option<Button>, Option<Button>)>; MAX_NUM_GAMEPAD_PROFILES as usize]
        //     = [None; MAX_NUM_GAMEPAD_PROFILES as usize];
        // let mut num_players: u8 = 1;
        // let mut starting_level: u8 = 0;
        // if there were game options, use those
        if let Some(menu_game_options) = last_used_game_options {
            // for (player, controls) in game_options.vec_keyboard_inputs.iter().enumerate() {
            //     vec_keyboard_controls.push(
            //         (player as u8,
            //         Some(controls.left),
            //         Some(controls.right),
            //         Some(controls.down),
            //         Some(controls.rotate_cw),
            //         Some(controls.rotate_ccw),)
            //     );
            // }
            // for (profile_idx, profile) in game_options.arr_profile_schemes.iter().enumerate() {
            //     arr_profile_schemes[profile_idx] = match profile {
            //         Some(p) => Some((p.left, p.right, p.down, Some(p.rotate_cw), Some(p.rotate_ccw), Some(p.start))),
            //         None => None,
            //     }
            // }
            // num_players = game_options.num_players;
            // starting_level = game_options.starting_level;

            // previously used
            Self {
                input: Input::new(),
                state: MenuState::Start,
                start_menu: StartMenu::new(window_dimensions, menu_game_options.num_players, menu_game_options.starting_level),
                input_config_menu: InputConfigMenu::new(window_dimensions, menu_game_options.arr_controls, menu_game_options.arr_gamepad_profiles),
            }
        } else {
            // defaults
            Self {
                input: Input::new(),
                state: MenuState::Start,
                start_menu: StartMenu::new(window_dimensions, 1, 0),
                input_config_menu: InputConfigMenu::new(window_dimensions, [(None, None); MAX_NUM_PLAYERS as usize], [None; MAX_NUM_GAMEPAD_PROFILES as usize]),
            }
        }
    }

    pub fn update(&mut self) -> Option<(ProgramState, MenuGameOptions)> {
        match self.state {
            MenuState::Start => {
                let ret_bools: (bool, bool) = self.start_menu.update(&self.input);
                if ret_bools.0 {
                    // we are starting the game
                    // let mut vec_control_scheme: Vec<KeyboardControlScheme> = Vec::with_capacity(self.input_config_menu.arr_controls.len());
                    // let mut arr_profile_schemes: [Option<GamepadProfileScheme>; MAX_NUM_GAMEPAD_PROFILES as usize] = [None; MAX_NUM_GAMEPAD_PROFILES as usize];
                    // // TODO: use a closure if that's better. It's too late at night for me to figure this out; I just want this to work; I've written ~500 lines of GUI code today; help
                    // for controls in self.input_config_menu.arr_controls.iter() {
                    //     if let Some(ctrls) = controls.0 {
                    //         vec_control_scheme.push(KeyboardControlScheme::new(
                    //             ctrls.0.expect("[!] key left set to None"),
                    //             ctrls.1.expect("[!] key right set to None"),
                    //             ctrls.2.expect("[!] key down set to None"),
                    //             ctrls.3.expect("[!] key rotate_cw set to None"),
                    //             ctrls.4.expect("[!] key rotate_ccw set to None"),
                    //             KeyCode::Escape,
                    //         ));
                    //     }
                    // }
                    // for (idx, opt_profile) in self.input_config_menu.arr_gamepad_profiles.iter().enumerate() {
                    //     if let Some(profile) = opt_profile {
                    //         arr_profile_schemes[idx] = Some(GamepadProfileScheme::new(
                    //             ((profile.0).0, (profile.0).1),
                    //             ((profile.1).0, (profile.1).1),
                    //             ((profile.2).0, (profile.2).1),
                    //             (profile.3).expect("[!] RotateCw was unexpectedly set to None"),
                    //             (profile.4).expect("[!] RotateCcw was unexpectedly set to None"),
                    //             (profile.5).expect("[!] Start was unexpectedly set to None"),
                    //         ));
                    //     }
                    // }
                    if self.ensure_enough_controls() {
                        return Some((ProgramState::Game, MenuGameOptions::new(self.start_menu.num_players, self.start_menu.starting_level, self.input_config_menu.arr_split_controls, self.input_config_menu.arr_split_gamepad_profiles)));
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
            if ctrls.0.is_some() || ctrls.1.is_some() {
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