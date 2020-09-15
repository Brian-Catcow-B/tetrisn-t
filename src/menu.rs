use ggez::Context;
use ggez::graphics::{self, DrawParam};
use ggez::event::{Button, Axis, KeyCode};
use ggez::graphics::{Color, Scale, Text, TextFragment};
use ggez::nalgebra::Point2;

use crate::control::ProgramState;
use crate::inputs::{Input, KeyboardControlScheme};

use crate::game::GameOptions;

const MAX_STARTING_LEVEL: u8 = 29; // this is just the fastest speed, so yeah
const MAX_NUM_PLAYERS: u8 = 62; // currently held back by board width being a u8 equal to 6 + 4 * num_players
const MAX_NUM_GAMEPAD_PROFILES: u8 = 9;

const DETECT_GAMEPAD_AXIS_THRESHOLD: f32 = 0.5;
const UNDETECT_GAMEPAD_AXIS_THRESHOLD: f32 = 0.3;

const SUB_TEXT_Y_TOP: f32 = 0.45;
const SUB_TEXT_Y_DIFF: f32 = 0.04;

const TEXT_SCALE_DOWN: f32 = 15.0;
const SUB_TEXT_SCALE_DOWN: f32 = 25.0;
const MINI_TEXT_SCALE_DOWN: f32 = 30.0;

const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
const DARK_GRAY: Color = Color::new(0.3, 0.3, 0.3, 1.0);
const LIGHT_GRAY: Color = Color::new(0.6, 0.6, 0.6, 1.0);
const SELECT_GREEN: Color = Color::new(0.153, 0.839, 0.075, 1.0);
const HELP_RED: Color = Color::new(0.9, 0.11, 0.11, 1.0);

#[repr(u8)]
enum MenuState {
    Main,
    InputConfig,
}

const NUM_MAINMENUOPTION_TEXT_ENTRIES: u8 = 4;
#[repr(u8)]
enum MainMenuOption {
    Start,
    NumPlayers,
    StartingLevel,
    Controls,
}

struct MainMenu {
    // logic
    selection: u8,
    num_players: u8,
    starting_level: u8,
    not_enough_controls_flag: bool,
    // drawing
    start_text: Text,
    not_enough_controls_text: Text,
    num_players_text: Text,
    starting_level_text: Text,
    controls_text: Text,
}

// for MenuState::Main
impl MainMenu {
    fn new(window_dimensions: (f32, f32), num_players: u8, starting_level: u8) -> Self {
        let mut num_players_text = Text::new(TextFragment::new("Number of Players: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        num_players_text.add(TextFragment::new(format!("{}", num_players)).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        let mut starting_level_text = Text::new(TextFragment::new("Starting Level: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        starting_level_text.add(TextFragment::new(format!("{}", starting_level)).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        Self {
            selection: MainMenuOption::Start as u8,
            num_players,
            starting_level,
            not_enough_controls_flag: false,
            start_text: Text::new(TextFragment::new("Start").color(SELECT_GREEN).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN))),
            not_enough_controls_text: Text::new(TextFragment::new("[!] Not enough controls setup to start").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            num_players_text,
            starting_level_text,
            controls_text: Text::new(TextFragment::new("Controls").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN))),
        }
    }
}

const NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES: u8 = 3;
#[repr(u8)]
enum InputConfigMenuOption {
    Back,
    GamepadProfile,
    PlayerInput,
}

// currently `Start` is, for keyboards, always `ESC`, and alternate controls are only for Left/Right/Down for controllers, so don't include that in the number of entries for keyboards
const NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES: u8 = 5;
#[repr(u8)]
enum InputConfigMenuSubOptionKeyboard {
    Left,
    Right,
    Down,
    RotateCw,
    RotateCcw,
}
const NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES: u8 = 9;
#[repr(u8)]
enum InputConfigMenuSubOptionGamepad {
    Left,
    AltLeft,
    Right,
    AltRight,
    Down,
    AltDown,
    RotateCw,
    RotateCcw,
    Start,
}

struct InputConfigMenu {
    // logic
    selection: u8,
    player_controls: u8,
    profile_num: u8,
    choose_profile_num: u8,
    choose_profile_flag: bool,
    sub_selection_keyboard: u8,
    sub_selection_keyboard_flag: bool,
    sub_selection_gamepad: u8,
    sub_selection_gamepad_flag: bool,
    most_recently_pressed_key: Option<KeyCode>,
    most_recently_pressed_gamepad_button: Option<Button>,
    most_recently_pressed_gamepad_axis: Option<(Axis, bool)>,
    gamepad_axis_wait: (bool, Option<(Axis, bool)>),
    vec_used_keycode: Vec<KeyCode>,
    keycode_conflict_flag: bool,
    arr_controls: [(Option<(Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>, Option<u8>); MAX_NUM_PLAYERS as usize],
    arr_gamepad_profiles: [Option<((Option<Button>, Option<(Axis, bool)>), (Option<Button>, Option<(Axis, bool)>), (Option<Button>, Option<(Axis, bool)>), Option<Button>, Option<Button>, Option<Button>)>; MAX_NUM_GAMEPAD_PROFILES as usize],
    // text
    back_text: Text,
    gamepad_profile_text: Text,
    player_controls_text: Text,
    // subtext
    input_uninitialized_text: Text,
    gamepad_profile_uninitialized_text: Text,
    keycode_conflict_text: Text,
    skip_button_axis_text: Text,
    choose_profile_text: Text,
    left_text: Text,
    alt_left_text: Text,
    right_text: Text,
    alt_right_text: Text,
    down_text: Text,
    alt_down_text: Text,
    rotate_cw_text: Text,
    rotate_ccw_text: Text,
    start_text: Text,
}

impl InputConfigMenu {
    fn new(window_dimensions: (f32, f32), last_used_keyboard_controls: Vec<(u8, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>) -> Self {
        let mut arr_controls: [(Option<(Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>, Option<u8>); MAX_NUM_PLAYERS as usize] = [(None, None); MAX_NUM_PLAYERS as usize];
        for ctrls in last_used_keyboard_controls.iter() {
            if ctrls.0 < MAX_NUM_PLAYERS {
                arr_controls[ctrls.0 as usize].0 = Some((ctrls.1, ctrls.2, ctrls.3, ctrls.4, ctrls.5));
            }
        }
        let mut player_controls_text = Text::new(TextFragment::new("Player Number: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        player_controls_text.add(TextFragment::new("1").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        let mut gamepad_profile_text = Text::new(TextFragment::new("GamePad Profile: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        gamepad_profile_text.add(TextFragment::new("1").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        let mut choose_profile_text = Text::new(TextFragment::new("Profile:").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut left_text = Text::new(TextFragment::new("Left:     ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut alt_left_text = Text::new(TextFragment::new("Left (Axis):  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        let mut right_text = Text::new(TextFragment::new("Right:    ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut alt_right_text = Text::new(TextFragment::new("Right (Axis): ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        let mut down_text = Text::new(TextFragment::new("Down:     ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut alt_down_text = Text::new(TextFragment::new("Down (Axis):  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        let mut rotate_cw_text = Text::new(TextFragment::new("RotateCw:  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut rotate_ccw_text = Text::new(TextFragment::new("RotateCcw:  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut start_text = Text::new(TextFragment::new("Start/Pause: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        if last_used_keyboard_controls.is_empty() {
            left_text.add(TextFragment::new("None").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            right_text.add(TextFragment::new("None").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            down_text.add(TextFragment::new("None").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            rotate_cw_text.add(TextFragment::new("None").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            rotate_ccw_text.add(TextFragment::new("None").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        } else {
            left_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].1.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            right_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].2.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            down_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].3.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            rotate_cw_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].4.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            rotate_ccw_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].5.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        }
        choose_profile_text.add(TextFragment::new(" 1").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        alt_left_text.add(TextFragment::new("None").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        alt_right_text.add(TextFragment::new("None").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        alt_down_text.add(TextFragment::new("None").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        start_text.add(TextFragment::new("Esc").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        Self {
            selection: 0,
            player_controls: 0,
            profile_num: 0,
            choose_profile_num: 0,
            choose_profile_flag: false,
            sub_selection_keyboard: 0,
            sub_selection_keyboard_flag: false,
            sub_selection_gamepad: 0,
            sub_selection_gamepad_flag: false,
            most_recently_pressed_key: None,
            most_recently_pressed_gamepad_button: None,
            most_recently_pressed_gamepad_axis: None,
            gamepad_axis_wait: (false, None),
            vec_used_keycode: vec![KeyCode::Escape],
            keycode_conflict_flag: false,
            arr_controls,
            arr_gamepad_profiles: [None; MAX_NUM_GAMEPAD_PROFILES as usize],
            // text
            back_text: Text::new(TextFragment::new("Back").color(SELECT_GREEN).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN))),
            gamepad_profile_text,
            player_controls_text,
            // subtext
            input_uninitialized_text: Text::new(TextFragment::new("No Controls\nKeyboard: Space/Enter\nGamepad: 'G'").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            gamepad_profile_uninitialized_text: Text::new(TextFragment::new("Profile nonexistent\nCreate/edit: Space/Enter").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            keycode_conflict_text: Text::new(TextFragment::new("[!] Redundant KeyCode; ignoring input").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            skip_button_axis_text: Text::new(TextFragment::new("Skip Button/Axis: Space/Enter").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            choose_profile_text,
            left_text,
            alt_left_text,
            right_text,
            alt_right_text,
            down_text,
            alt_down_text,
            rotate_cw_text,
            rotate_ccw_text,
            start_text,
        }
    }
}

// we just have all the menu stuffs loaded into here because they're all connected
pub struct Menu {
    // logic
    input: Input,
    // drawing
    window_dimensions: (f32, f32),
    // states
    state: MenuState,
    main_menu: MainMenu,
    input_config_menu: InputConfigMenu,
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
            window_dimensions,
            state: MenuState::Main,
            main_menu: MainMenu::new(window_dimensions, num_players, starting_level),
            input_config_menu: InputConfigMenu::new(window_dimensions, arr_controls),
        }
    }

    pub fn update(&mut self) -> Option<(ProgramState, GameOptions)> {
        if self.input.keydown_right.1 && !self.input_config_menu.sub_selection_keyboard_flag {
            self.inc_or_dec_selection(true);
        }

        if self.input.keydown_left.1 && !self.input_config_menu.sub_selection_keyboard_flag {
            self.inc_or_dec_selection(false);
        }

        match self.state {
            MenuState::Main => {
                if self.input.keydown_down.1 {
                    self.set_select(false);
                    self.main_menu.selection = (self.main_menu.selection + 1) % NUM_MAINMENUOPTION_TEXT_ENTRIES;
                    self.set_select(true);
                }

                if self.input.keydown_up.1 {
                    self.set_select(false);
                    self.main_menu.selection = if self.main_menu.selection == 0 {NUM_MAINMENUOPTION_TEXT_ENTRIES - 1} else {self.main_menu.selection - 1};
                    self.set_select(true);
                }

                if self.input.keydown_start.1 && self.main_menu.selection == MainMenuOption::Controls as u8 {
                    self.main_menu.not_enough_controls_flag = false;
                    self.state = MenuState::InputConfig;
                }

                if self.input.keydown_start.1 && self.main_menu.selection == MainMenuOption::Start as u8 {
                    self.input_config_menu.arr_controls.sort_by_key(|ctrls| ctrls.0);
                    let mut vec_control_scheme: Vec<KeyboardControlScheme> = Vec::with_capacity(self.input_config_menu.arr_controls.len());
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
                    if vec_control_scheme.len() < self.main_menu.num_players as usize {
                        self.main_menu.not_enough_controls_flag = true;
                    } else {
                        return Some((ProgramState::Game, GameOptions::new(self.main_menu.num_players, self.main_menu.starting_level, vec_control_scheme)));
                    }
                }
            },
            MenuState::InputConfig => {
                if !self.input_config_menu.sub_selection_keyboard_flag && !self.input_config_menu.sub_selection_gamepad_flag && !self.input_config_menu.choose_profile_flag {
                    if self.input.keydown_down.1 {
                        self.set_select(false);
                        self.input_config_menu.selection = (self.input_config_menu.selection + 1) % NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES;
                        self.set_select(true);
                    }

                    if self.input.keydown_up.1 {
                        self.set_select(false);
                        self.input_config_menu.selection = if self.input_config_menu.selection == 0 {NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES - 1} else {self.input_config_menu.selection - 1};
                        self.set_select(true);
                    }

                    if self.input.keydown_rotate_cw.1 && self.input_config_menu.selection == InputConfigMenuOption::PlayerInput as u8 {
                        self.input_config_menu.choose_profile_flag = true;
                        self.set_select(true);
                    }

                    if self.input.keydown_start.1 {
                        if self.input_config_menu.selection == InputConfigMenuOption::Back as u8 {
                            self.input_config_menu.sub_selection_keyboard = 0;
                            self.state = MenuState::Main;
                        } else if self.input_config_menu.selection == InputConfigMenuOption::GamepadProfile as u8 {
                            self.input_config_menu.most_recently_pressed_gamepad_axis = None;
                            self.input_config_menu.most_recently_pressed_gamepad_button = None;
                            self.input_config_menu.arr_gamepad_profiles[self.input_config_menu.profile_num as usize] = Some(((None, None), (None, None), (None, None), None, None, None));
                            self.update_sub_text_strings_gamepad();
                            self.input_config_menu.sub_selection_gamepad_flag = true;
                            self.set_select(true);
                        } else if self.input_config_menu.selection == InputConfigMenuOption::PlayerInput as u8 {
                            self.input_config_menu.most_recently_pressed_key = None;
                            // remove the old keyboard controls after removing all the KeyCodes from the used keycodes vector since we are overwriting this one
                            let mut items_removed = 0;
                            // we must index because .remove() pulls the indices after it back by 1, so use `items_removed` to pull the index back with it
                            if let Some(ctrls) = self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0 {
                                for used_key_idx in 0..self.input_config_menu.vec_used_keycode.len() {
                                    if Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == ctrls.0
                                    || Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == ctrls.1
                                    || Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == ctrls.2
                                    || Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == ctrls.3
                                    || Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == ctrls.4 {
                                        self.input_config_menu.vec_used_keycode.remove(used_key_idx - items_removed);
                                        items_removed += 1;
                                        // we only need to get rid of NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES
                                        if items_removed >= NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES as usize {
                                            break;
                                        }
                                    }
                                }
                            }
                            self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0 = None;

                            self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0 = Some((None, None, None, None, None));
                            self.update_sub_text_strings_keyboard();
                            self.input_config_menu.sub_selection_keyboard_flag = true;
                            self.set_select(true);
                        }
                    }
                } else if self.input_config_menu.sub_selection_gamepad_flag {
                    if self.input_config_menu.most_recently_pressed_gamepad_button.is_some() {
                        let mut not_on_button_flag: bool = false;
                        if let Some(mut profile) = self.input_config_menu.arr_gamepad_profiles[self.input_config_menu.profile_num as usize].as_mut() {
                            match self.input_config_menu.sub_selection_gamepad {
                                x if x == InputConfigMenuSubOptionGamepad::Left as u8 => {
                                    (profile.0).0 = self.input_config_menu.most_recently_pressed_gamepad_button;
                                },
                                x if x == InputConfigMenuSubOptionGamepad::Right as u8 => {
                                    (profile.1).0 = self.input_config_menu.most_recently_pressed_gamepad_button;
                                },
                                x if x == InputConfigMenuSubOptionGamepad::Down as u8 => {
                                    (profile.2).0 = self.input_config_menu.most_recently_pressed_gamepad_button;
                                },
                                x if x == InputConfigMenuSubOptionGamepad::RotateCw as u8 => {
                                    profile.3 = self.input_config_menu.most_recently_pressed_gamepad_button;
                                },
                                x if x == InputConfigMenuSubOptionGamepad::RotateCcw as u8 => {
                                    profile.4 = self.input_config_menu.most_recently_pressed_gamepad_button;
                                },
                                x if x == InputConfigMenuSubOptionGamepad::Start as u8 => {
                                    profile.5 = self.input_config_menu.most_recently_pressed_gamepad_button;
                                },
                                _ => not_on_button_flag = true,
                            }
                        } else {
                            println!("[!] gamepad profile unexpectedly None");
                        }
                        if !not_on_button_flag {
                            self.set_select(false);
                            if self.input_config_menu.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                                self.input_config_menu.sub_selection_gamepad += 1;
                                self.set_select(true);
                            } else {
                                self.input_config_menu.sub_selection_gamepad = 0;
                                self.input_config_menu.sub_selection_gamepad_flag = false;
                            }
                        } else {
                            self.input_config_menu.most_recently_pressed_gamepad_button = None;
                        }

                        self.input_config_menu.most_recently_pressed_gamepad_axis = None;
                        self.input_config_menu.most_recently_pressed_key = None;
                    }

                    if self.input_config_menu.most_recently_pressed_gamepad_axis.is_some() {
                        let mut not_on_axis_flag: bool = false;
                        if let Some(mut profile) = self.input_config_menu.arr_gamepad_profiles[self.input_config_menu.profile_num as usize].as_mut() {
                            match self.input_config_menu.sub_selection_gamepad {
                                x if x == InputConfigMenuSubOptionGamepad::AltLeft as u8 => {
                                    (profile.0).1 = self.input_config_menu.most_recently_pressed_gamepad_axis;
                                },
                                x if x == InputConfigMenuSubOptionGamepad::AltRight as u8 => {
                                    (profile.1).1 = self.input_config_menu.most_recently_pressed_gamepad_axis;
                                },
                                x if x == InputConfigMenuSubOptionGamepad::AltDown as u8 => {
                                    (profile.2).1 = self.input_config_menu.most_recently_pressed_gamepad_axis;
                                },
                                _ => not_on_axis_flag = true,
                            }
                        } else {
                            println!("[!] gamepad profile unexpectedly None");
                        }
                        if !not_on_axis_flag {
                            self.set_select(false);
                            if self.input_config_menu.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                                self.input_config_menu.sub_selection_gamepad += 1;
                                self.set_select(true);
                            } else {
                                self.input_config_menu.sub_selection_gamepad = 0;
                                self.input_config_menu.sub_selection_gamepad_flag = false;
                            }
                        } else {
                            self.input_config_menu.most_recently_pressed_gamepad_axis = None;
                        }

                        self.input_config_menu.most_recently_pressed_key = None;
                    }

                    if self.input_config_menu.most_recently_pressed_key == Some(KeyCode::Escape) {
                        self.set_select(false);
                        self.input_config_menu.arr_gamepad_profiles[self.input_config_menu.profile_num as usize] = None;
                        self.input_config_menu.sub_selection_gamepad = 0;
                        self.input_config_menu.sub_selection_gamepad_flag = false;
                    }
                } else if self.input_config_menu.sub_selection_keyboard_flag {
                    if self.input_config_menu.most_recently_pressed_key.is_some() {
                        // first check if the KeyCode is Escape, and if it is, just delete the layout entry and go out of the subselection section
                        // second check if the KeyCode was already used. If it was, set the error message flag to true
                        if self.input_config_menu.most_recently_pressed_key == Some(KeyCode::Escape) {
                            self.set_select(false);
                            self.input_config_menu.keycode_conflict_flag = false;
                            self.input_config_menu.sub_selection_keyboard = 0;
                            self.input_config_menu.sub_selection_keyboard_flag = false;
                            // the user was in the middle of creating keyboard controls when they hit Escape, so pop however many KeyCode's off vec_used_keycode as the user set up
                            if let Some(ctrls) = self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0 {
                                if (ctrls.3).is_some() {
                                    for _ in 1..=4 {
                                        self.input_config_menu.vec_used_keycode.pop();
                                    }
                                } else if (ctrls.2).is_some() {
                                    for _ in 1..=3 {
                                        self.input_config_menu.vec_used_keycode.pop();
                                    }
                                } else if (ctrls.1).is_some() {
                                    for _ in 1..=2 {
                                        self.input_config_menu.vec_used_keycode.pop();
                                    }
                                } else if (ctrls.0).is_some() {
                                    self.input_config_menu.vec_used_keycode.pop();
                                }
                            }
                            self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0 = None;
                        } else if self.input_config_menu.vec_used_keycode.contains(&self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None")) {
                            self.input_config_menu.keycode_conflict_flag = true;
                        } else {
                            self.input_config_menu.keycode_conflict_flag = false;
                            match (self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0).as_mut() {
                                Some(mut ctrls) => {
                                    match self.input_config_menu.sub_selection_keyboard {
                                        x if x == InputConfigMenuSubOptionKeyboard::Left as u8 => {
                                            ctrls.0 = self.input_config_menu.most_recently_pressed_key;
                                            self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                        },
                                        x if x == InputConfigMenuSubOptionKeyboard::Right as u8 => {
                                            ctrls.1 = self.input_config_menu.most_recently_pressed_key;
                                            self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                        },
                                        x if x == InputConfigMenuSubOptionKeyboard::Down as u8 => {
                                            ctrls.2 = self.input_config_menu.most_recently_pressed_key;
                                            self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                        },
                                        x if x == InputConfigMenuSubOptionKeyboard::RotateCw as u8 => {
                                            ctrls.3 = self.input_config_menu.most_recently_pressed_key;
                                            self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                        },
                                        x if x == InputConfigMenuSubOptionKeyboard::RotateCcw as u8 => {
                                            ctrls.4 = self.input_config_menu.most_recently_pressed_key;
                                            self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                        },
                                        _ => println!("[!] couldn't get correct tuple index to set most recently pressed key"),
                                    }
                                },
                                None => {
                                    println!("[!] arr_controls[{}].0 was unexpectedly None", self.input_config_menu.player_controls);
                                }
                            }
                            self.set_select(false);
                            if self.input_config_menu.sub_selection_keyboard < NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES as u8 - 1 {
                                self.input_config_menu.sub_selection_keyboard += 1;
                                self.set_select(true);
                            } else {
                                self.input_config_menu.sub_selection_keyboard = 0;
                                self.input_config_menu.sub_selection_keyboard_flag = false;
                            }
                        }
                    }
                } else if self.input_config_menu.choose_profile_flag {
                    if self.input.keydown_start.1 {
                        self.set_select(false);
                        self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].1 = Some(self.input_config_menu.choose_profile_num);
                        self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0 = None;
                        self.input_config_menu.choose_profile_flag = false;
                    }
                }
            }
        }

        self.input.was_just_pressed_setfalse();
        None
    }

    fn set_select(&mut self, select_flag: bool) {
        match self.state {
            MenuState::Main => {
                match self.main_menu.selection {
                    x if x == MainMenuOption::Start as u8 => {
                        if select_flag {
                            self.main_menu.start_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        } else {
                            self.main_menu.start_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        }
                    },
                    x if x == MainMenuOption::NumPlayers as u8 => {
                        if select_flag {
                            self.main_menu.num_players_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                            self.main_menu.num_players_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            self.main_menu.num_players_text.fragments_mut()[1].text = format!("<{}>", self.main_menu.num_players);
                        } else {
                            self.main_menu.num_players_text.fragments_mut()[0].color = Some(graphics::BLACK);
                            self.main_menu.num_players_text.fragments_mut()[1].color = Some(graphics::BLACK);
                            self.main_menu.num_players_text.fragments_mut()[1].text = format!(" {}", self.main_menu.num_players);
                        }
                    },
                    x if x == MainMenuOption::StartingLevel as u8 => {
                        if select_flag {
                            self.main_menu.starting_level_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                            self.main_menu.starting_level_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            self.main_menu.starting_level_text.fragments_mut()[1].text = format!("<{}>", self.main_menu.starting_level);
                        } else {
                            self.main_menu.starting_level_text.fragments_mut()[0].color = Some(graphics::BLACK);
                            self.main_menu.starting_level_text.fragments_mut()[1].color = Some(graphics::BLACK);
                            self.main_menu.starting_level_text.fragments_mut()[1].text = format!(" {}", self.main_menu.starting_level);
                        }
                    },
                    x if x == MainMenuOption::Controls as u8 => {
                        if select_flag {
                            self.main_menu.controls_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        } else {
                            self.main_menu.controls_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        }
                    },
                    _ => println!("[!] main_menu_option didn't find match"),
                }
            },
            MenuState::InputConfig => {
                if !self.input_config_menu.sub_selection_keyboard_flag && !self.input_config_menu.sub_selection_gamepad_flag && !self.input_config_menu.choose_profile_flag {
                    match self.input_config_menu.selection {
                        x if x == InputConfigMenuOption::Back as u8 => {
                            if select_flag {
                                self.input_config_menu.back_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.back_text.fragments_mut()[0].color = Some(graphics::BLACK);
                            }
                        },
                        x if x == InputConfigMenuOption::GamepadProfile as u8 => {
                            if select_flag {
                                self.input_config_menu.gamepad_profile_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.gamepad_profile_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                                self.input_config_menu.gamepad_profile_text.fragments_mut()[1].text = format!("<{}>", self.input_config_menu.profile_num + 1);
                            } else {
                                self.input_config_menu.gamepad_profile_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.gamepad_profile_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                self.input_config_menu.gamepad_profile_text.fragments_mut()[1].text = format!(" {}", self.input_config_menu.profile_num + 1);
                            }
                        },
                        x if x == InputConfigMenuOption::PlayerInput as u8 => {
                            if select_flag {
                                self.input_config_menu.player_controls_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.player_controls_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                                self.input_config_menu.player_controls_text.fragments_mut()[1].text = format!("<{}>", self.input_config_menu.player_controls + 1);
                            } else {
                                self.input_config_menu.player_controls_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.player_controls_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                self.input_config_menu.player_controls_text.fragments_mut()[1].text = format!(" {}", self.input_config_menu.player_controls + 1);
                            }
                        },
                        _ => println!("[!] input_config_menu_option didn't find match"),
                    }
                } else if self.input_config_menu.sub_selection_gamepad_flag {
                    match self.input_config_menu.sub_selection_gamepad {
                        x if x == InputConfigMenuSubOptionGamepad::Left as u8 => {
                            if select_flag {
                                self.input_config_menu.left_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.left_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.left_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.left_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_button {
                                    Some(button) => self.input_config_menu.left_text.fragments_mut()[1].text = format!("{:?}", button),
                                    None => self.input_config_menu.left_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_button = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionGamepad::AltLeft as u8 => {
                            if select_flag {
                                self.input_config_menu.alt_left_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.alt_left_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.alt_left_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.alt_left_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_axis {
                                    Some(axis) => self.input_config_menu.alt_left_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'}),
                                    None => self.input_config_menu.alt_left_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_axis = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionGamepad::Right as u8 => {
                            if select_flag {
                                self.input_config_menu.right_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.right_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.right_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.right_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_button {
                                    Some(button) => self.input_config_menu.right_text.fragments_mut()[1].text = format!("{:?}", button),
                                    None => self.input_config_menu.right_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_button = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionGamepad::AltRight as u8 => {
                            if select_flag {
                                self.input_config_menu.alt_right_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.alt_right_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.alt_right_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.alt_right_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_axis {
                                    Some(axis) => self.input_config_menu.alt_right_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'}),
                                    None => self.input_config_menu.alt_right_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_axis = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionGamepad::Down as u8 => {
                            if select_flag {
                                self.input_config_menu.down_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.down_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.down_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.down_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_button {
                                    Some(button) => self.input_config_menu.down_text.fragments_mut()[1].text = format!("{:?}", button),
                                    None => self.input_config_menu.down_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_button = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionGamepad::AltDown as u8 => {
                            if select_flag {
                                self.input_config_menu.alt_down_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.alt_down_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.alt_down_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.alt_down_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_axis {
                                    Some(axis) => self.input_config_menu.alt_down_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'}),
                                    None => self.input_config_menu.alt_down_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_axis = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionGamepad::RotateCw as u8 => {
                            if select_flag {
                                self.input_config_menu.rotate_cw_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.rotate_cw_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.rotate_cw_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.rotate_cw_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_button {
                                    Some(button) => self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = format!("{:?}", button),
                                    None => self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_button = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionGamepad::RotateCcw as u8 => {
                            if select_flag {
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_button {
                                    Some(button) => self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = format!("{:?}", button),
                                    None => self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_button = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionGamepad::Start as u8 => {
                            if select_flag {
                                self.input_config_menu.start_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.start_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.start_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.start_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                match self.input_config_menu.most_recently_pressed_gamepad_button {
                                    Some(button) => self.input_config_menu.start_text.fragments_mut()[1].text = format!("{:?}", button),
                                    None => self.input_config_menu.start_text.fragments_mut()[1].text = format!("None"),
                                }
                                self.input_config_menu.most_recently_pressed_gamepad_button = None;
                            }
                        },
                        _ => println!("[!] input_config_menu_option didn't find match"),
                    }
                } else if self.input_config_menu.sub_selection_keyboard_flag {
                    match self.input_config_menu.sub_selection_keyboard {
                        x if x == InputConfigMenuSubOptionKeyboard::Left as u8 => {
                            if select_flag {
                                self.input_config_menu.left_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.left_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.left_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.left_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                self.input_config_menu.left_text.fragments_mut()[1].text = format!(" {:?}", self.input_config_menu.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                                self.input_config_menu.most_recently_pressed_key = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionKeyboard::Right as u8 => {
                            if select_flag {
                                self.input_config_menu.right_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.right_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.right_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.right_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                self.input_config_menu.right_text.fragments_mut()[1].text = format!(" {:?}", self.input_config_menu.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                                self.input_config_menu.most_recently_pressed_key = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionKeyboard::Down as u8 => {
                            if select_flag {
                                self.input_config_menu.down_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.down_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.down_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.down_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                self.input_config_menu.down_text.fragments_mut()[1].text = format!(" {:?}", self.input_config_menu.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                                self.input_config_menu.most_recently_pressed_key = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionKeyboard::RotateCw as u8 => {
                            if select_flag {
                                self.input_config_menu.rotate_cw_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.rotate_cw_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.rotate_cw_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.rotate_cw_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = format!(" {:?}", self.input_config_menu.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                                self.input_config_menu.most_recently_pressed_key = None;
                            }
                        },
                        x if x == InputConfigMenuSubOptionKeyboard::RotateCcw as u8 => {
                            if select_flag {
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = format!(" {:?}", self.input_config_menu.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                                self.input_config_menu.most_recently_pressed_key = None;
                            }
                        },
                        _ => println!("[!] input_config_menu_option didn't find match"),
                    } 
                } else if self.input_config_menu.choose_profile_flag {
                    if select_flag {
                        self.input_config_menu.choose_profile_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.input_config_menu.choose_profile_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                        self.input_config_menu.choose_profile_text.fragments_mut()[1].text = format!("<{}>", self.input_config_menu.choose_profile_num + 1);
                    } else {
                        self.input_config_menu.choose_profile_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.input_config_menu.choose_profile_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.input_config_menu.choose_profile_text.fragments_mut()[1].text = format!(" {}", self.input_config_menu.choose_profile_num + 1);
                        self.input_config_menu.most_recently_pressed_key = None;
                    }
                }
            }
        }
    }

    fn inc_or_dec_selection(&mut self, inc_flag: bool) {
        match self.state {
            MenuState::Main => {
                // the if/else here only includes MainMenuOptions that have a value that can be modified
                if self.main_menu.selection == MainMenuOption::NumPlayers as u8 {
                    // special case (index by 1 because we can't have 0 players)
                    if inc_flag {
                        self.main_menu.num_players = self.main_menu.num_players % MAX_NUM_PLAYERS + 1;
                    } else {
                        self.main_menu.num_players = if self.main_menu.num_players == 1 {MAX_NUM_PLAYERS} else {self.main_menu.num_players - 1};
                    }
                    self.main_menu.num_players_text.fragments_mut()[1].text = format!("<{}>", self.main_menu.num_players);
                } else if self.main_menu.selection == MainMenuOption::StartingLevel as u8 {
                    if inc_flag {
                        self.main_menu.starting_level = (self.main_menu.starting_level + 1) % (MAX_STARTING_LEVEL + 1);
                    } else {
                        self.main_menu.starting_level = if self.main_menu.starting_level == 0 {MAX_STARTING_LEVEL} else {self.main_menu.starting_level - 1};
                    }
                    self.main_menu.starting_level_text.fragments_mut()[1].text = format!("<{}>", self.main_menu.starting_level);
                }
            },
            MenuState::InputConfig => {
                if !self.input_config_menu.sub_selection_keyboard_flag && !self.input_config_menu.sub_selection_gamepad_flag && !self.input_config_menu.choose_profile_flag {
                    if self.input_config_menu.selection == InputConfigMenuOption::PlayerInput as u8 {
                        if inc_flag {
                            self.input_config_menu.player_controls = (self.input_config_menu.player_controls + 1) % MAX_NUM_PLAYERS;
                        } else {
                            self.input_config_menu.player_controls = if self.input_config_menu.player_controls == 0 {MAX_NUM_PLAYERS - 1} else {self.input_config_menu.player_controls - 1};
                        }
                        // display player_controls + 1 because index by 1 to users
                        self.input_config_menu.player_controls_text.fragments_mut()[1].text = format!("<{}>", self.input_config_menu.player_controls + 1);
                        self.update_sub_text_strings_keyboard();
                    } else if self.input_config_menu.selection == InputConfigMenuOption::GamepadProfile as u8 {
                        if inc_flag {
                            self.input_config_menu.profile_num = (self.input_config_menu.profile_num + 1) % MAX_NUM_GAMEPAD_PROFILES;
                        } else {
                            self.input_config_menu.profile_num = if self.input_config_menu.profile_num == 0 {MAX_NUM_GAMEPAD_PROFILES - 1} else {self.input_config_menu.profile_num - 1};
                        }
                        // display profile_num + 1 because index by 1 to users
                        self.input_config_menu.gamepad_profile_text.fragments_mut()[1].text = format!("<{}>", self.input_config_menu.profile_num + 1);
                        self.update_sub_text_strings_gamepad();
                    }
                } else if self.input_config_menu.choose_profile_flag {
                    if inc_flag {
                        self.input_config_menu.choose_profile_num = (self.input_config_menu.choose_profile_num + 1) % MAX_NUM_GAMEPAD_PROFILES;
                    } else {
                        self.input_config_menu.choose_profile_num = if self.input_config_menu.choose_profile_num == 0 {MAX_NUM_GAMEPAD_PROFILES - 1} else {self.input_config_menu.choose_profile_num - 1};
                    }
                    self.input_config_menu.choose_profile_text.fragments_mut()[1].text = format!("<{}>", self.input_config_menu.choose_profile_num + 1);
                }
            }
        }
    }

    fn set_gamepad_specific_sub_text_strings(&mut self) {
        self.input_config_menu.left_text.fragments_mut()[0].text = "Left (Button):  ".to_string();
        self.input_config_menu.right_text.fragments_mut()[0].text = "Right (Button): ".to_string();
        self.input_config_menu.down_text.fragments_mut()[0].text = "Down (Button):  ".to_string();
        self.input_config_menu.start_text.fragments_mut()[0].text = "Start: ".to_string();
    }

    fn set_keyboard_specific_sub_text_strings(&mut self) {
        self.input_config_menu.left_text.fragments_mut()[0].text = "Left:     ".to_string();
        self.input_config_menu.right_text.fragments_mut()[0].text = "Right:    ".to_string();
        self.input_config_menu.down_text.fragments_mut()[0].text = "Down:     ".to_string();
        self.input_config_menu.start_text.fragments_mut()[0].text = "Start/Pause: ".to_string();
        self.input_config_menu.start_text.fragments_mut()[1].text = "Esc".to_string();
    }

    fn update_sub_text_strings_gamepad(&mut self) {
        self.set_gamepad_specific_sub_text_strings();
        match self.input_config_menu.arr_gamepad_profiles[self.input_config_menu.profile_num as usize] {
            Some(profile) => {
                match (profile.0).0 {
                    Some(button) => {
                        self.input_config_menu.left_text.fragments_mut()[1].text = format!("{:?}", button);
                    },
                    None => {
                        self.input_config_menu.left_text.fragments_mut()[1].text = "None".to_string();
                    }
                }
                match (profile.0).1 {
                    Some(axis) => {
                        self.input_config_menu.alt_left_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'});
                    },
                    None => {
                        self.input_config_menu.alt_left_text.fragments_mut()[1].text = "None".to_string();
                    }
                }

                match (profile.1).0 {
                    Some(button) => {
                        self.input_config_menu.right_text.fragments_mut()[1].text = format!("{:?}", button);
                    },
                    None => {
                        self.input_config_menu.right_text.fragments_mut()[1].text = "None".to_string();
                    }
                }
                match (profile.1).1 {
                    Some(axis) => {
                        self.input_config_menu.alt_right_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'});
                    },
                    None => {
                        self.input_config_menu.alt_right_text.fragments_mut()[1].text = "None".to_string();
                    }
                }

                match (profile.2).0 {
                    Some(button) => {
                        self.input_config_menu.down_text.fragments_mut()[1].text = format!("{:?}", button);
                    },
                    None => {
                        self.input_config_menu.down_text.fragments_mut()[1].text = "None".to_string();
                    }
                }
                match (profile.2).1 {
                    Some(axis) => {
                        self.input_config_menu.alt_down_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'});
                    },
                    None => {
                        self.input_config_menu.alt_down_text.fragments_mut()[1].text = "None".to_string();
                    }
                }

                match profile.3 {
                    Some(button) => {
                        self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = format!("{:?}", button);
                    },
                    None => {
                        self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = "None".to_string();
                    }
                }

                match profile.4 {
                    Some(button) => {
                        self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = format!("{:?}", button);
                    },
                    None => {
                        self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = "None".to_string();
                    }
                }

                match profile.5 {
                    Some(button) => {
                        self.input_config_menu.start_text.fragments_mut()[1].text = format!("{:?}", button);
                    },
                    None => {
                        self.input_config_menu.start_text.fragments_mut()[1].text = "None".to_string();
                    }
                }
            }
            None => {
                println!("it is none!");
                self.input_config_menu.left_text.fragments_mut()[1].text = "None".to_string();
                self.input_config_menu.alt_left_text.fragments_mut()[1].text = "None".to_string();
                self.input_config_menu.right_text.fragments_mut()[1].text = "None".to_string();
                self.input_config_menu.alt_right_text.fragments_mut()[1].text = "None".to_string();
                self.input_config_menu.down_text.fragments_mut()[1].text = "None".to_string();
                self.input_config_menu.alt_down_text.fragments_mut()[1].text = "None".to_string();
                self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = "None".to_string();
                self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = "None".to_string();
                self.input_config_menu.start_text.fragments_mut()[1].text = "None".to_string();
            }
        }
    }

    fn update_sub_text_strings_keyboard(&mut self) {
        self.set_keyboard_specific_sub_text_strings();

        if let Some(ctrls) = self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0 {
            match ctrls.0 {
                Some(keycode) => {
                    self.input_config_menu.left_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.input_config_menu.left_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.1 {
                Some(keycode) => {
                    self.input_config_menu.right_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.input_config_menu.right_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.2 {
                Some(keycode) => {
                    self.input_config_menu.down_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.input_config_menu.down_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.3 {
                Some(keycode) => {
                    self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.4 {
                Some(keycode) => {
                    self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = "None".to_string();
                }
            }
        } else if let Some(profile) = self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].1 {
            self.input_config_menu.choose_profile_text.fragments_mut()[1].text = format!(" {}", profile + 1);
        }
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
        self.window_dimensions = graphics::size(ctx);

        match self.state {
            MenuState::Main => {
                if self.main_menu.not_enough_controls_flag {
                    self.draw_text(ctx, &self.main_menu.not_enough_controls_text, 0.1);
                }
                self.draw_text(ctx, &self.main_menu.start_text, 0.2);
                self.draw_text(ctx, &self.main_menu.num_players_text, 0.4);
                self.draw_text(ctx, &self.main_menu.starting_level_text, 0.6);
                self.draw_text(ctx, &self.main_menu.controls_text, 0.8);
            },
            MenuState::InputConfig => {
                self.draw_text(ctx, &self.input_config_menu.back_text, 0.1);
                self.draw_text(ctx, &self.input_config_menu.gamepad_profile_text, 0.2);
                self.draw_text(ctx, &self.input_config_menu.player_controls_text, 0.3);

                if self.input_config_menu.keycode_conflict_flag {
                    self.draw_text(ctx, &self.input_config_menu.keycode_conflict_text, 0.2);
                }

                // display nothing special on InputConfigMenuOption::Back, so just draw the extra stuff when it's not on InputConfigMenuOption::Back
                // and then later determine which of the other InputConfigMenuOption's it is for the specifics
                if self.input_config_menu.selection != InputConfigMenuOption::Back as u8 {
                    // draw a rectangle containing the subtexts for choosing controls
                    // with a color based on whether or not the user is editing controls
                    let editing_indicator_rectangle: graphics::Mesh;
                    let rect_w = self.window_dimensions.0 / 2.0;
                    let rect_h = self.window_dimensions.1 / 2.0;
                    let rect_x = (self.window_dimensions.0 - rect_w) / 2.0;
                    let rect_y = self.window_dimensions.1 * 0.4;
                    if !self.input_config_menu.sub_selection_keyboard_flag && !self.input_config_menu.sub_selection_gamepad_flag && !self.input_config_menu.choose_profile_flag {
                        editing_indicator_rectangle = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect {
                                x: rect_x,
                                y: rect_y,
                                w: rect_w,
                                h: rect_h,
                            },
                            DARK_GRAY,
                        ).unwrap();
                    } else {
                        editing_indicator_rectangle = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect {
                                x: rect_x,
                                y: rect_y,
                                w: rect_w,
                                h: rect_h,
                            },
                            LIGHT_GRAY,
                        ).unwrap();
                    }
                    graphics::draw(ctx, &editing_indicator_rectangle, (Point2::new(0.0, 0.0),)).unwrap();

                    if self.input_config_menu.selection == InputConfigMenuOption::GamepadProfile as u8 {
                        if self.input_config_menu.arr_gamepad_profiles[self.input_config_menu.profile_num as usize].is_some() {
                            self.draw_text(ctx, &self.input_config_menu.left_text, SUB_TEXT_Y_TOP);
                            self.draw_text(ctx, &self.input_config_menu.alt_left_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF);
                            self.draw_text(ctx, &self.input_config_menu.right_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 2.0);
                            self.draw_text(ctx, &self.input_config_menu.alt_right_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 3.0);
                            self.draw_text(ctx, &self.input_config_menu.down_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 4.0);
                            self.draw_text(ctx, &self.input_config_menu.alt_down_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 5.0);
                            self.draw_text(ctx, &self.input_config_menu.rotate_cw_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 6.0);
                            self.draw_text(ctx, &self.input_config_menu.rotate_ccw_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 7.0);
                            self.draw_text(ctx, &self.input_config_menu.start_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 8.0);
                        } else {
                            self.draw_text(ctx, &self.input_config_menu.gamepad_profile_uninitialized_text, 0.5);
                        }

                        if self.input_config_menu.sub_selection_gamepad_flag {
                            self.draw_text(ctx, &self.input_config_menu.skip_button_axis_text, 0.9);
                        }
                    } else if self.input_config_menu.selection == InputConfigMenuOption::PlayerInput as u8 {
                        if self.input_config_menu.choose_profile_flag {
                            self.draw_text(ctx, &self.input_config_menu.choose_profile_text, 0.5);
                        } else {
                            if (self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].0).is_some() {
                                self.draw_text(ctx, &self.input_config_menu.left_text, 0.5);
                                self.draw_text(ctx, &self.input_config_menu.right_text, 0.55);
                                self.draw_text(ctx, &self.input_config_menu.down_text, 0.6);
                                self.draw_text(ctx, &self.input_config_menu.rotate_cw_text, 0.65);
                                self.draw_text(ctx, &self.input_config_menu.rotate_ccw_text, 0.7);
                                self.draw_text(ctx, &self.input_config_menu.start_text, 0.75);
                            } else if (self.input_config_menu.arr_controls[self.input_config_menu.player_controls as usize].1).is_some() {
                                self.draw_text(ctx, &self.input_config_menu.choose_profile_text, 0.5);
                            } else {
                                self.draw_text(ctx, &self.input_config_menu.input_uninitialized_text, 0.5);
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw_text(&self, ctx: &mut Context, text_var: &Text, vertical_position: f32) {
        let (text_width, text_height) = text_var.dimensions(ctx);
        graphics::draw(ctx, text_var, DrawParam::new()
        .dest(Point2::new((self.window_dimensions.0 - text_width as f32) / 2.0, (self.window_dimensions.1 - text_height as f32) * vertical_position))
        ).unwrap();
    }
}