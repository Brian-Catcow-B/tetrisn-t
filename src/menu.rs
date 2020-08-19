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

const TEXT_SCALE_DOWN: f32 = 10.0;
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
    fn new(window_dimensions: (f32, f32)) -> Self {
        let mut num_players_text = Text::new(TextFragment {
            text: "Number of Players: ".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        num_players_text.add(TextFragment {
            text: "1".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        let mut starting_level_text = Text::new(TextFragment {
            text: "Starting Level: ".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        starting_level_text.add(TextFragment {
            text: "0".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        Self {
            selection: MainMenuOption::Start as u8,
            num_players: 1,
            starting_level: 0,
            not_enough_controls_flag: false,
            start_text: Text::new(TextFragment {
                text: "Start".to_string(),
                color: Some(SELECT_GREEN),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
            not_enough_controls_text: Text::new(TextFragment {
                text: "[!] Not enough controls setup to start".to_string(),
                color: Some(HELP_RED),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
            num_players_text,
            starting_level_text,
            controls_text: Text::new(TextFragment {
                text: "Controls".to_string(),
                color: Some(graphics::BLACK),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
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
    vec_keyboard_controls: Vec<(u8, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>,
    // text
    back_text: Text,
    player_num_text: Text,
    // subtext
    uninitialized_text: Text,
    left_text: Text,
    right_text: Text,
    down_text: Text,
    rotate_cw_text: Text,
    rotate_ccw_text: Text,
    start_text: Text,
}

impl InputConfigMenu {
    fn new(window_dimensions: (f32, f32)) -> Self {
        let mut player_num_text = Text::new(TextFragment {
            text: "Player Number: ".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        player_num_text.add(TextFragment {
            text: "1".to_string(), // display an index by 1 because users are stupid lol
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        let mut left_text = Text::new(TextFragment {
            text: "Left:     ".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        left_text.add(TextFragment {
            text: "None".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        let mut right_text = Text::new(TextFragment {
            text: "Right:    ".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        right_text.add(TextFragment {
            text: "None".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        let mut down_text = Text::new(TextFragment {
            text: "Down:     ".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        down_text.add(TextFragment {
            text: "None".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        let mut rotate_cw_text = Text::new(TextFragment {
            text: "RotateCw:  ".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        rotate_cw_text.add(TextFragment {
            text: "None".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        let mut rotate_ccw_text = Text::new(TextFragment {
            text: "RotateCcw: ".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        rotate_ccw_text.add(TextFragment {
            text: "None".to_string(),
            color: Some(graphics::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ..Default::default()
        });
        Self {
            selection: 0,
            player_num: 0,
            sub_selection: 0,
            sub_selection_flag: false,
            most_recently_pressed_key: None,
            vec_keyboard_controls: vec![],
            back_text: Text::new(TextFragment {
                text: "Back".to_string(),
                color: Some(SELECT_GREEN),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
            player_num_text,
            // subtext
            uninitialized_text: Text::new(TextFragment {
                text: "No Controls\nPress Space/Enter to edit".to_string(),
                color: Some(HELP_RED),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
            left_text,
            right_text,
            down_text,
            rotate_cw_text,
            rotate_ccw_text,
            start_text: Text::new(TextFragment {
                text: "Start/Pause: ESC".to_string(),
                color: Some(graphics::BLACK),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
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
    pub fn new(ctx: &mut Context) -> Self {
        let window_dimensions = graphics::size(ctx);
        Self {
            input: Input::new(),
            window_dimensions,
            state: MenuState::Main,
            main_menu: MainMenu::new(window_dimensions),
            input_config_menu: InputConfigMenu::new(window_dimensions),
        }
    }

    pub fn update(&mut self) -> Option<(ProgramState, GameOptions)> {
        // TODO: a lot of this is redundant; can probably get rid of the match statement (mostly)
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

                if self.input.keydown_right.1 {
                    self.inc_or_dec_selection(true);
                }

                if self.input.keydown_left.1 {
                    self.inc_or_dec_selection(false);
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
                        println!("[+] tried to start game with {} players and {} controls", self.main_menu.num_players, vec_control_scheme.len());
                    } else {
                        return Some((ProgramState::Game, GameOptions::new(self.main_menu.num_players, self.main_menu.starting_level, vec_control_scheme)));
                    }
                }
            },
            MenuState::InputConfig => {
                if self.input.keydown_right.1 {
                    self.inc_or_dec_selection(true);
                }

                if self.input.keydown_left.1 {
                    self.inc_or_dec_selection(false);
                }

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
                        match self.input_config_menu.sub_selection {
                            x if x == InputConfigMenuSubOption::Left as u8 => {
                                self.input_config_menu.vec_keyboard_controls[vec_length - 1].1 = self.input_config_menu.most_recently_pressed_key;
                            },
                            x if x == InputConfigMenuSubOption::Right as u8 => {
                                self.input_config_menu.vec_keyboard_controls[vec_length - 1].2 = self.input_config_menu.most_recently_pressed_key;
                            },
                            x if x == InputConfigMenuSubOption::Down as u8 => {
                                self.input_config_menu.vec_keyboard_controls[vec_length - 1].3 = self.input_config_menu.most_recently_pressed_key;
                            },
                            x if x == InputConfigMenuSubOption::RotateCw as u8 => {
                                self.input_config_menu.vec_keyboard_controls[vec_length - 1].4 = self.input_config_menu.most_recently_pressed_key;
                            },
                            x if x == InputConfigMenuSubOption::RotateCcw as u8 => {
                                self.input_config_menu.vec_keyboard_controls[vec_length - 1].5 = self.input_config_menu.most_recently_pressed_key;
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
                println!("not found!");
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
                // start
                let (start_text_width, start_text_height) = self.main_menu.start_text.dimensions(ctx);
                graphics::draw(ctx, &self.main_menu.start_text, DrawParam::new()
                .dest(Point2::new((self.window_dimensions.0 - start_text_width as f32) / 2.0, (self.window_dimensions.1 - start_text_height as f32) * 0.2))
                ).unwrap();

                // not enough controls
                if self.main_menu.not_enough_controls_flag {
                    let (not_enough_controls_text_width, not_enough_controls_text_height) = self.main_menu.not_enough_controls_text.dimensions(ctx);
                    graphics::draw(ctx, &self.main_menu.not_enough_controls_text, DrawParam::new()
                    .dest(Point2::new((self.window_dimensions.0 - not_enough_controls_text_width as f32) / 2.0, (self.window_dimensions.1 - not_enough_controls_text_height as f32) * 0.3))
                    ).unwrap();
                }
        
                // num players
                let (num_players_text_width, num_players_text_height) = self.main_menu.num_players_text.dimensions(ctx);
                graphics::draw(ctx, &self.main_menu.num_players_text, DrawParam::new()
                .dest(Point2::new((self.window_dimensions.0 - num_players_text_width as f32) / 2.0, (self.window_dimensions.1 - num_players_text_height as f32) * 0.4))
                ).unwrap();
        
                // starting level
                let (starting_level_text_width, starting_level_text_height) = self.main_menu.starting_level_text.dimensions(ctx);
                graphics::draw(ctx, &self.main_menu.starting_level_text, DrawParam::new()
                .dest(Point2::new((self.window_dimensions.0 - starting_level_text_width as f32) / 2.0, (self.window_dimensions.1 - starting_level_text_height as f32) * 0.6))
                ).unwrap();

                // controls
                let (controls_text_width, controls_text_height) = self.main_menu.controls_text.dimensions(ctx);
                graphics::draw(ctx, &self.main_menu.controls_text, DrawParam::new()
                .dest(Point2::new((self.window_dimensions.0 - controls_text_width as f32) / 2.0, (self.window_dimensions.1 - controls_text_height as f32) * 0.8))
                ).unwrap();
            },
            MenuState::InputConfig => {
                // back
                let (back_text_width, back_text_height) = self.input_config_menu.back_text.dimensions(ctx);
                graphics::draw(ctx, &self.input_config_menu.back_text, DrawParam::new()
                .dest(Point2::new((self.window_dimensions.0 - back_text_width as f32) / 2.0, (self.window_dimensions.1 - back_text_height as f32) * 0.1))
                ).unwrap();

                // player_num
                let (player_num_text_width, player_num_text_height) = self.input_config_menu.player_num_text.dimensions(ctx);
                graphics::draw(ctx, &self.input_config_menu.player_num_text, DrawParam::new()
                .dest(Point2::new((self.window_dimensions.0 - player_num_text_width as f32) / 2.0, (self.window_dimensions.1 - player_num_text_height as f32) * 0.3))
                ).unwrap();

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
                        // left_text
                        let (left_text_width, left_text_height) = self.input_config_menu.left_text.dimensions(ctx);
                        graphics::draw(ctx, &self.input_config_menu.left_text, DrawParam::new()
                        .dest(Point2::new((self.window_dimensions.0 - left_text_width as f32) / 2.0, (self.window_dimensions.1 - left_text_height as f32) * 0.5))
                        ).unwrap();

                        // right_text
                        let (right_text_width, right_text_height) = self.input_config_menu.right_text.dimensions(ctx);
                        graphics::draw(ctx, &self.input_config_menu.right_text, DrawParam::new()
                        .dest(Point2::new((self.window_dimensions.0 - right_text_width as f32) / 2.0, (self.window_dimensions.1 - right_text_height as f32) * 0.55))
                        ).unwrap();

                        // down_text
                        let (down_text_width, down_text_height) = self.input_config_menu.down_text.dimensions(ctx);
                        graphics::draw(ctx, &self.input_config_menu.down_text, DrawParam::new()
                        .dest(Point2::new((self.window_dimensions.0 - down_text_width as f32) / 2.0, (self.window_dimensions.1 - down_text_height as f32) * 0.6))
                        ).unwrap();

                        // rotate_cw_text
                        let (rotate_cw_text_width, rotate_cw_text_height) = self.input_config_menu.rotate_cw_text.dimensions(ctx);
                        graphics::draw(ctx, &self.input_config_menu.rotate_cw_text, DrawParam::new()
                        .dest(Point2::new((self.window_dimensions.0 - rotate_cw_text_width as f32) / 2.0, (self.window_dimensions.1 - rotate_cw_text_height as f32) * 0.65))
                        ).unwrap();

                        // rotate_ccw_text
                        let (rotate_ccw_text_width, rotate_ccw_text_height) = self.input_config_menu.rotate_ccw_text.dimensions(ctx);
                        graphics::draw(ctx, &self.input_config_menu.rotate_ccw_text, DrawParam::new()
                        .dest(Point2::new((self.window_dimensions.0 - rotate_ccw_text_width as f32) / 2.0, (self.window_dimensions.1 - rotate_ccw_text_height as f32) * 0.7))
                        ).unwrap();

                        // start_text
                        let (start_text_width, start_text_height) = self.input_config_menu.start_text.dimensions(ctx);
                        graphics::draw(ctx, &self.input_config_menu.start_text, DrawParam::new()
                        .dest(Point2::new((self.window_dimensions.0 - start_text_width as f32) / 2.0, (self.window_dimensions.1 - start_text_height as f32) * 0.75))
                        ).unwrap();
                    } else {
                        // uninitialized_text
                        let (uninitialized_text_width, uninitialized_text_height) = self.input_config_menu.uninitialized_text.dimensions(ctx);
                        graphics::draw(ctx, &self.input_config_menu.uninitialized_text, DrawParam::new()
                        .dest(Point2::new((self.window_dimensions.0 - uninitialized_text_width as f32) / 2.0, (self.window_dimensions.1 - uninitialized_text_height as f32) * 0.5))
                        ).unwrap();
                    }
                }
            }
        }
    }
}