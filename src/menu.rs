use ggez::Context;
use ggez::graphics::{self, DrawParam};
use ggez::event::KeyCode;
use ggez::graphics::{Color, Scale, Text, TextFragment};
use ggez::nalgebra::Point2;

use crate::control::ProgramState;
use crate::inputs::Input;

use crate::game::GameOptions;

const MAX_STARTING_LEVEL: u8 = 29; // this is just the fastest speed, so yeah
const MAX_NUM_PLAYERS: u8 = 62; // currently held back by board width being a u8 equal to 6 + 4 * num_players

const TEXT_SCALE_DOWN: f32 = 10.0;

const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
const SELECT_GREEN: Color = Color::new(0.153, 0.839, 0.075, 1.0);

const NUM_MENUOPTION_ENTRIES: u8 = 3;
#[repr(u8)]
enum MenuOption {
    Start,
    NumPlayers,
    StartingLevel,
}

pub struct Menu {
    // logic
    input: Input,
    selection: u8,
    num_players: u8,
    starting_level: u8,
    // drawing
    window_dimensions: (f32, f32),
    start_text: Text,
    num_players_text: Text,
    starting_level_text: Text,
}

impl Menu {
    pub fn new(ctx: &mut Context) -> Self {
        let window_dimensions = graphics::size(ctx);
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
            input: Input::new(),
            selection: MenuOption::Start as u8,
            num_players: 1,
            starting_level: 0,
            window_dimensions,
            start_text: Text::new(TextFragment {
                text: "Start".to_string(),
                color: Some(SELECT_GREEN),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
            num_players_text,
            starting_level_text,
        }
    }

    pub fn update(&mut self) -> Option<(ProgramState, GameOptions)> {
        if self.input.keydown_down.1 {
            self.set_select(false);
            self.selection = (self.selection + 1) % NUM_MENUOPTION_ENTRIES;
            self.set_select(true);
        }

        if self.input.keydown_up.1 {
            self.set_select(false);
            self.selection = if self.selection == 0 {NUM_MENUOPTION_ENTRIES - 1} else {self.selection - 1};
            self.set_select(true);
        }

        if self.input.keydown_right.1 {
            self.inc_or_dec_selection(true);
        }

        if self.input.keydown_left.1 {
            self.inc_or_dec_selection(false);
        }

        if self.input.keydown_start.1 && self.selection == MenuOption::Start as u8 {
            return Some((ProgramState::Game, GameOptions::new(self.num_players, self.starting_level)));
        }

        self.input.was_just_pressed_setfalse();
        None
    }

    fn set_select(&mut self, select_flag: bool) {
        match self.selection {
            x if x == MenuOption::Start as u8 => {
                if select_flag {
                    self.start_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                } else {
                    self.start_text.fragments_mut()[0].color = Some(graphics::BLACK);
                }
            },
            x if x == MenuOption::NumPlayers as u8 => {
                if select_flag {
                    self.num_players_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                    self.num_players_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    self.num_players_text.fragments_mut()[1].text = format!("<{}>", self.num_players);
                } else {
                    self.num_players_text.fragments_mut()[0].color = Some(graphics::BLACK);
                    self.num_players_text.fragments_mut()[1].color = Some(graphics::BLACK);
                    self.num_players_text.fragments_mut()[1].text = format!("{}", self.num_players);
                }
            },
            x if x == MenuOption::StartingLevel as u8 => {
                if select_flag {
                    self.starting_level_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                    self.starting_level_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    self.starting_level_text.fragments_mut()[1].text = format!("<{}>", self.starting_level);
                } else {
                    self.starting_level_text.fragments_mut()[0].color = Some(graphics::BLACK);
                    self.starting_level_text.fragments_mut()[1].color = Some(graphics::BLACK);
                    self.starting_level_text.fragments_mut()[1].text = format!("{}", self.starting_level);
                }
            },
            _ => println!("[!] menu_option didn't find match"),
        }
    }

    fn inc_or_dec_selection(&mut self, inc_flag: bool) {
        // the if/else here only includes MenuOptions that have a value that can be modified
        if self.selection == MenuOption::NumPlayers as u8 {
            // special case (index by 1 because we can't have 0 players)
            if inc_flag {
                self.num_players = self.num_players % MAX_NUM_PLAYERS + 1;
            } else {
                self.num_players = if self.num_players == 1 {MAX_NUM_PLAYERS} else {self.num_players - 1};
            }
            self.num_players_text.fragments_mut()[1].text = format!("<{}>", self.num_players);
        } else if self.selection == MenuOption::StartingLevel as u8 {
            if inc_flag {
                self.starting_level = (self.starting_level + 1) % (MAX_STARTING_LEVEL + 1);
            } else {
                self.starting_level = if self.starting_level == 0 {MAX_STARTING_LEVEL} else {self.starting_level - 1};
            }
            self.starting_level_text.fragments_mut()[1].text = format!("<{}>", self.starting_level);
        }
    }

    pub fn key_down_event(&mut self, keycode: KeyCode, _repeat: bool) {
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

        // start
        let (start_text_width, start_text_height) = self.start_text.dimensions(ctx);
        graphics::draw(ctx, &self.start_text, DrawParam::new()
        .offset(Point2::new(0.5, 0.5))
        .dest(Point2::new((self.window_dimensions.0 - start_text_width as f32) / 2.0, (self.window_dimensions.1 - start_text_height as f32) * 0.2))
        ).unwrap();

        // num players
        let (num_players_text_width, num_players_text_height) = self.num_players_text.dimensions(ctx);
        graphics::draw(ctx, &self.num_players_text, DrawParam::new()
        .offset(Point2::new(0.5, 0.5))
        .dest(Point2::new((self.window_dimensions.0 - num_players_text_width as f32) / 2.0, (self.window_dimensions.1 - num_players_text_height as f32) * 0.4))
        ).unwrap();

        // starting level
        let (starting_level_text_width, starting_level_text_height) = self.starting_level_text.dimensions(ctx);
        graphics::draw(ctx, &self.starting_level_text, DrawParam::new()
        .offset(Point2::new(0.5, 0.5))
        .dest(Point2::new((self.window_dimensions.0 - starting_level_text_width as f32) / 2.0, (self.window_dimensions.1 - starting_level_text_height as f32) * 0.6))
        ).unwrap();
    }
}