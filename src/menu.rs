use ggez::Context;
use ggez::graphics::{self, DrawParam};
use ggez::event::KeyCode;
use ggez::graphics::{Color, Scale, Text, TextFragment};
use ggez::nalgebra::Point2;

use crate::control::ProgramState;
use crate::inputs::{Input, ControlScheme};

use crate::game::GameOptions;

const MAX_STARTING_LEVEL: u8 = 29; // this is just the fastest speed, so yeah
const MAX_NUM_PLAYERS: u8 = 62; // currently held back by board width being a u8 equal to 6 + 4 * num_players

const TEXT_SCALE_DOWN: f32 = 15.0;
const SUB_TEXT_SCALE_DOWN: f32 = 20.0;

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

const NUM_MAINMENUOPTION_ENTRIES: u8 = 4;
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

const NUM_INPUTCONFIGMENUOPTION_ENTRIES: u8 = 2;
#[repr(u8)]
enum InputConfigMenuOption {
    Back,
    PlayerInput,
}

// currently `Start` is, for keyboards, always `ESC`
const NUM_INPUTCONFIGMENUSUBOPTION_ENTRIES: u8 = 5;
#[repr(u8)]
enum InputConfigMenuSubOption {
    Left,
    Right,
    Down,
    RotateCw,
    RotateCcw,
    // Start,
}

struct InputConfigMenu {
    // logic
    selection: u8,
    player_num: u8,
    sub_selection: u8,
    sub_selection_flag: bool,
    most_recently_pressed_key: Option<KeyCode>,
    vec_used_keycode: Vec<KeyCode>,
    keycode_conflict_flag: bool,
    vec_keyboard_controls: Vec<(u8, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>,
    // text
    back_text: Text,
    player_num_text: Text,
    // subtext
    uninitialized_text: Text,
    keycode_conflict_text: Text,
    left_text: Text,
    right_text: Text,
    down_text: Text,
    rotate_cw_text: Text,
    rotate_ccw_text: Text,
    start_text: Text,
}

impl InputConfigMenu {
    fn new(window_dimensions: (f32, f32), last_used_keyboard_controls: Vec<(u8, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>) -> Self {
        let mut player_num_text = Text::new(TextFragment::new("Player Number: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        player_num_text.add(TextFragment::new("1").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        let mut left_text = Text::new(TextFragment::new("Left:     ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut right_text = Text::new(TextFragment::new("Right:    ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut down_text = Text::new(TextFragment::new("Down:     ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut rotate_cw_text = Text::new(TextFragment::new("RotateCw:  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut rotate_ccw_text = Text::new(TextFragment::new("RotateCcw:  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
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
        Self {
            selection: 0,
            player_num: 0,
            sub_selection: 0,
            sub_selection_flag: false,
            most_recently_pressed_key: None,
            vec_used_keycode: vec![KeyCode::Escape],
            keycode_conflict_flag: false,
            vec_keyboard_controls: last_used_keyboard_controls,
            // text
            back_text: Text::new(TextFragment::new("Back").color(SELECT_GREEN).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN))),
            player_num_text,
            // subtext
            uninitialized_text: Text::new(TextFragment::new("No Controls\nPress Space/Enter to edit").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            keycode_conflict_text: Text::new(TextFragment::new("[!] Redundant KeyCode; ignoring input").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            left_text,
            right_text,
            down_text,
            rotate_cw_text,
            rotate_ccw_text,
            start_text: Text::new(TextFragment::new("Start/Pause: ESC").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
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
        let mut vec_keyboard_controls: Vec<(u8, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)> = vec![];
        let mut num_players: u8 = 1;
        let mut starting_level: u8 = 0;
        if let Some(game_options) = last_used_game_options {
            for (player, controls) in game_options.vec_keyboard_inputs.iter().enumerate() {
                vec_keyboard_controls.push(
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
            input_config_menu: InputConfigMenu::new(window_dimensions, vec_keyboard_controls),
        }
    }

    pub fn update(&mut self) -> Option<(ProgramState, GameOptions)> {
        if self.input.keydown_right.1 && !self.input_config_menu.sub_selection_flag {
            self.inc_or_dec_selection(true);
        }

        if self.input.keydown_left.1 && !self.input_config_menu.sub_selection_flag {
            self.inc_or_dec_selection(false);
        }

        match self.state {
            MenuState::Main => {
                if self.input.keydown_down.1 {
                    self.set_select(false);
                    self.main_menu.selection = (self.main_menu.selection + 1) % NUM_MAINMENUOPTION_ENTRIES;
                    self.set_select(true);
                }

                if self.input.keydown_up.1 {
                    self.set_select(false);
                    self.main_menu.selection = if self.main_menu.selection == 0 {NUM_MAINMENUOPTION_ENTRIES - 1} else {self.main_menu.selection - 1};
                    self.set_select(true);
                }

                if self.input.keydown_start.1 && self.main_menu.selection == MainMenuOption::Controls as u8 {
                    self.main_menu.not_enough_controls_flag = false;
                    self.state = MenuState::InputConfig;
                }

                if self.input.keydown_start.1 && self.main_menu.selection == MainMenuOption::Start as u8 {
                    self.input_config_menu.vec_keyboard_controls.sort_by_key(|ctrls| ctrls.0);
                    let mut vec_control_scheme: Vec<ControlScheme> = Vec::with_capacity(self.input_config_menu.vec_keyboard_controls.len());
                    // TODO: use a closure if that's better. It's too late at night for me to figure this out; I just want this to work; I've written ~500 lines of GUI code today; help
                    for keyboard_controls in self.input_config_menu.vec_keyboard_controls.iter() {
                        vec_control_scheme.push(ControlScheme::new(
                            keyboard_controls.1.expect("[!] key left set to None"),
                            keyboard_controls.2.expect("[!] key right set to None"),
                            keyboard_controls.3.expect("[!] key down set to None"),
                            keyboard_controls.4.expect("[!] key rotate_cw set to None"),
                            keyboard_controls.5.expect("[!] key rotate_ccw set to None"),
                            KeyCode::Escape,
                        ));
                    }
                    if vec_control_scheme.len() < self.main_menu.num_players as usize {
                        self.main_menu.not_enough_controls_flag = true;
                    } else {
                        return Some((ProgramState::Game, GameOptions::new(self.main_menu.num_players, self.main_menu.starting_level, vec_control_scheme)));
                    }
                }
            },
            MenuState::InputConfig => {
                if !self.input_config_menu.sub_selection_flag {
                    if self.input.keydown_down.1 {
                        self.set_select(false);
                        self.input_config_menu.selection = (self.input_config_menu.selection + 1) % NUM_INPUTCONFIGMENUOPTION_ENTRIES;
                        self.set_select(true);
                    }

                    if self.input.keydown_up.1 {
                        self.set_select(false);
                        self.input_config_menu.selection = if self.input_config_menu.selection == 0 {NUM_INPUTCONFIGMENUOPTION_ENTRIES - 1} else {self.input_config_menu.selection - 1};
                        self.set_select(true);
                    }

                    if self.input.keydown_start.1 && self.input_config_menu.selection == InputConfigMenuOption::Back as u8 {
                        self.input_config_menu.sub_selection = 0;
                        self.state = MenuState::Main;
                    }

                    if self.input.keydown_start.1 && self.input_config_menu.selection == InputConfigMenuOption::PlayerInput as u8 {
                        self.input_config_menu.most_recently_pressed_key = None;
                        for (idx, ctrls) in self.input_config_menu.vec_keyboard_controls.iter().enumerate() {
                            if ctrls.0 == self.input_config_menu.player_num {
                                // remove the old keyboard controls after removing all the KeyCodes from the used keycodes vector since we are overwriting this one
                                let mut items_removed = 0;
                                // we must index because .remove() pulls the indices after it back by 1, so use `items_removed` to pull the index back with it
                                for used_key_idx in 0..self.input_config_menu.vec_used_keycode.len() {
                                    if Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == self.input_config_menu.vec_keyboard_controls[idx].1
                                    || Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == self.input_config_menu.vec_keyboard_controls[idx].2
                                    || Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == self.input_config_menu.vec_keyboard_controls[idx].3
                                    || Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == self.input_config_menu.vec_keyboard_controls[idx].4
                                    || Some(self.input_config_menu.vec_used_keycode[used_key_idx - items_removed]) == self.input_config_menu.vec_keyboard_controls[idx].5 {
                                        self.input_config_menu.vec_used_keycode.remove(used_key_idx - items_removed);
                                        items_removed += 1;
                                        // we only need to get rid of 5
                                        if items_removed >= 5 {
                                            break;
                                        }
                                    }
                                }
                                self.input_config_menu.vec_keyboard_controls.remove(idx);
                                break;
                            }
                        }
                        self.input_config_menu.vec_keyboard_controls.push((self.input_config_menu.player_num, None, None, None, None, None));
                        self.update_sub_text_strings();
                        self.input_config_menu.sub_selection_flag = true;
                        self.set_select(true);
                    }
                } else {
                    if self.input_config_menu.most_recently_pressed_key.is_some() && self.input_config_menu.vec_keyboard_controls.len() > 0 {
                        // set the tuple index to the correct key of the tuple just pushed to the vector
                        let vec_length = self.input_config_menu.vec_keyboard_controls.len();
                        // first check if the KeyCode is Escape, and if it is, just delete the layout entry and go out of the subselection section
                        // second check if the KeyCode was already used. If it was, set the error message flag to true
                        if self.input_config_menu.most_recently_pressed_key == Some(KeyCode::Escape) {
                            self.set_select(false);
                            self.input_config_menu.keycode_conflict_flag = false;
                            self.input_config_menu.sub_selection = 0;
                            self.input_config_menu.sub_selection_flag = false;
                            if self.input_config_menu.vec_keyboard_controls[vec_length - 1].4.is_some() {
                                for _ in 1..=4 {
                                    self.input_config_menu.vec_used_keycode.pop();
                                }
                            } else if self.input_config_menu.vec_keyboard_controls[vec_length - 1].3.is_some() {
                                for _ in 1..=3 {
                                    self.input_config_menu.vec_used_keycode.pop();
                                }
                            } else if self.input_config_menu.vec_keyboard_controls[vec_length - 1].2.is_some() {
                                for _ in 1..=2 {
                                    self.input_config_menu.vec_used_keycode.pop();
                                }
                            } else if self.input_config_menu.vec_keyboard_controls[vec_length - 1].1.is_some() {
                                self.input_config_menu.vec_used_keycode.pop();
                            }
                            self.input_config_menu.vec_keyboard_controls.pop();
                        } else if self.input_config_menu.vec_used_keycode.contains(&self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None")) {
                            self.input_config_menu.keycode_conflict_flag = true;
                        } else {
                            self.input_config_menu.keycode_conflict_flag = false;
                            match self.input_config_menu.sub_selection {
                                x if x == InputConfigMenuSubOption::Left as u8 => {
                                    self.input_config_menu.vec_keyboard_controls[vec_length - 1].1 = self.input_config_menu.most_recently_pressed_key;
                                    self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                x if x == InputConfigMenuSubOption::Right as u8 => {
                                    self.input_config_menu.vec_keyboard_controls[vec_length - 1].2 = self.input_config_menu.most_recently_pressed_key;
                                    self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                x if x == InputConfigMenuSubOption::Down as u8 => {
                                    self.input_config_menu.vec_keyboard_controls[vec_length - 1].3 = self.input_config_menu.most_recently_pressed_key;
                                    self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                x if x == InputConfigMenuSubOption::RotateCw as u8 => {
                                    self.input_config_menu.vec_keyboard_controls[vec_length - 1].4 = self.input_config_menu.most_recently_pressed_key;
                                    self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                x if x == InputConfigMenuSubOption::RotateCcw as u8 => {
                                    self.input_config_menu.vec_keyboard_controls[vec_length - 1].5 = self.input_config_menu.most_recently_pressed_key;
                                    self.input_config_menu.vec_used_keycode.push(self.input_config_menu.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                _ => println!("[!] couldn't get correct tuple index to set most recently pressed key"),
                            }
                            self.set_select(false);
                            if self.input_config_menu.sub_selection < NUM_INPUTCONFIGMENUSUBOPTION_ENTRIES as u8 - 1 {
                                self.input_config_menu.sub_selection += 1;
                                self.set_select(true);
                            } else {
                                self.input_config_menu.sub_selection = 0;
                                self.input_config_menu.sub_selection_flag = false;
                            }
                        }
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
                if !self.input_config_menu.sub_selection_flag {
                    match self.input_config_menu.selection {
                        x if x == InputConfigMenuOption::Back as u8 => {
                            if select_flag {
                                self.input_config_menu.back_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                            } else {
                                self.input_config_menu.back_text.fragments_mut()[0].color = Some(graphics::BLACK);
                            }
                        },
                        x if x == InputConfigMenuOption::PlayerInput as u8 => {
                            if select_flag {
                                self.input_config_menu.player_num_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                                self.input_config_menu.player_num_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                                self.input_config_menu.player_num_text.fragments_mut()[1].text = format!("<{}>", self.input_config_menu.player_num + 1);
                            } else {
                                self.input_config_menu.player_num_text.fragments_mut()[0].color = Some(graphics::BLACK);
                                self.input_config_menu.player_num_text.fragments_mut()[1].color = Some(graphics::BLACK);
                                self.input_config_menu.player_num_text.fragments_mut()[1].text = format!(" {}", self.input_config_menu.player_num + 1);
                            }
                        },
                        _ => println!("[!] input_config_menu_option didn't find match"),
                    }
                } else {
                    match self.input_config_menu.sub_selection {
                        x if x == InputConfigMenuSubOption::Left as u8 => {
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
                        x if x == InputConfigMenuSubOption::Right as u8 => {
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
                        x if x == InputConfigMenuSubOption::Down as u8 => {
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
                        x if x == InputConfigMenuSubOption::RotateCw as u8 => {
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
                        x if x == InputConfigMenuSubOption::RotateCcw as u8 => {
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
                if !self.input_config_menu.sub_selection_flag {
                    if self.input_config_menu.selection == InputConfigMenuOption::PlayerInput as u8 {
                        if inc_flag {
                            self.input_config_menu.player_num = (self.input_config_menu.player_num + 1) % MAX_NUM_PLAYERS;
                        } else {
                            self.input_config_menu.player_num = if self.input_config_menu.player_num == 0 {MAX_NUM_PLAYERS - 1} else {self.input_config_menu.player_num - 1};
                        }
                        // display player_num + 1 because index by 1 to users
                        self.input_config_menu.player_num_text.fragments_mut()[1].text = format!("<{}>", self.input_config_menu.player_num + 1);
                        self.update_sub_text_strings();
                    }
                }
            }
        }
    }

    fn update_sub_text_strings(&mut self) {
        let mut index_found: Option<u8> = None;
        for (idx, ctrls) in self.input_config_menu.vec_keyboard_controls.iter().enumerate() {
            if ctrls.0 == self.input_config_menu.player_num {
                index_found = Some(idx as u8);
                break;
            }
        }

        match index_found {
            Some(index) => {
                match self.input_config_menu.vec_keyboard_controls[index as usize].1 {
                    Some(keycode) => {
                        self.input_config_menu.left_text.fragments_mut()[1].text = format!(" {:?}", keycode);
                    },
                    None => {
                        self.input_config_menu.left_text.fragments_mut()[1].text = " None".to_string();
                    }
                }
                match self.input_config_menu.vec_keyboard_controls[index as usize].2 {
                    Some(keycode) => {
                        self.input_config_menu.right_text.fragments_mut()[1].text = format!(" {:?}", keycode);
                    },
                    None => {
                        self.input_config_menu.right_text.fragments_mut()[1].text = " None".to_string();
                    }
                }
                match self.input_config_menu.vec_keyboard_controls[index as usize].3 {
                    Some(keycode) => {
                        self.input_config_menu.down_text.fragments_mut()[1].text = format!(" {:?}", keycode);
                    },
                    None => {
                        self.input_config_menu.down_text.fragments_mut()[1].text = " None".to_string();
                    }
                }
                match self.input_config_menu.vec_keyboard_controls[index as usize].4 {
                    Some(keycode) => {
                        self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = format!(" {:?}", keycode);
                    },
                    None => {
                        self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = " None".to_string();
                    }
                }
                match self.input_config_menu.vec_keyboard_controls[index as usize].5 {
                    Some(keycode) => {
                        self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = format!(" {:?}", keycode);
                    },
                    None => {
                        self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = " None".to_string();
                    }
                }
            }
            None => {
                self.input_config_menu.left_text.fragments_mut()[1].text = " None".to_string();
                self.input_config_menu.right_text.fragments_mut()[1].text = " None".to_string();
                self.input_config_menu.down_text.fragments_mut()[1].text = " None".to_string();
                self.input_config_menu.rotate_cw_text.fragments_mut()[1].text = " None".to_string();
                self.input_config_menu.rotate_ccw_text.fragments_mut()[1].text = " None".to_string();
            }
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
        // } else if keycode == KeyCode:: {
        //     if !self.input.keydown_rotate_cw.0 {
        //         self.input.keydown_rotate_cw = (true, true);
        //     }
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
        // } else if keycode == KeyCode:: {
        //     self.input.keydown_rotate_cw = (false, false);
        // } else if keycode == KeyCode:: {
        //     self.input.keydown_rotate_ccw = (false, false);
        } else if keycode == KeyCode::Space || keycode == KeyCode::Return {
            self.input.keydown_start = (false, false);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, GRAY);
        self.window_dimensions = graphics::size(ctx);

        match self.state {
            MenuState::Main => {
                self.draw_text(ctx, &self.main_menu.start_text, 0.2);
                if self.main_menu.not_enough_controls_flag {
                    self.draw_text(ctx, &self.main_menu.not_enough_controls_text, 0.3);
                }
                self.draw_text(ctx, &self.main_menu.num_players_text, 0.4);
                self.draw_text(ctx, &self.main_menu.starting_level_text, 0.6);
                self.draw_text(ctx, &self.main_menu.controls_text, 0.8);
            },
            MenuState::InputConfig => {
                self.draw_text(ctx, &self.input_config_menu.back_text, 0.1);
                self.draw_text(ctx, &self.input_config_menu.player_num_text, 0.3);

                if self.input_config_menu.keycode_conflict_flag {
                    self.draw_text(ctx, &self.input_config_menu.keycode_conflict_text, 0.2);
                }

                if self.input_config_menu.selection == InputConfigMenuOption::PlayerInput as u8 {
                    // draw a rectangle containing the subtexts for choosing controls
                    // with a color based on whether or not the user is editing controls
                    let editing_indicator_rectangle: graphics::Mesh;
                    let rect_w = self.window_dimensions.0 / 2.0;
                    let rect_h = self.window_dimensions.1 / 2.0;
                    let rect_x = (self.window_dimensions.0 - rect_w) / 2.0;
                    let rect_y = self.window_dimensions.1 * 0.4;
                    if !self.input_config_menu.sub_selection_flag {
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

                    // determine if uninitialized_text xor input stuffs should be displayed
                    let mut player_input_exists: bool = false;
                    for inputs in self.input_config_menu.vec_keyboard_controls.iter() {
                        if inputs.0 == self.input_config_menu.player_num {
                            player_input_exists = true;
                            break;
                        }
                    }

                    if player_input_exists {
                        self.draw_text(ctx, &self.input_config_menu.left_text, 0.5);
                        self.draw_text(ctx, &self.input_config_menu.right_text, 0.55);
                        self.draw_text(ctx, &self.input_config_menu.down_text, 0.6);
                        self.draw_text(ctx, &self.input_config_menu.rotate_cw_text, 0.65);
                        self.draw_text(ctx, &self.input_config_menu.rotate_ccw_text, 0.7);
                        self.draw_text(ctx, &self.input_config_menu.start_text, 0.75);
                    } else {
                        self.draw_text(ctx, &self.input_config_menu.uninitialized_text, 0.5);
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