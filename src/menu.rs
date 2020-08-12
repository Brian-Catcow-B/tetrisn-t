use ggez::Context;
use ggez::graphics::{self, DrawParam};
use ggez::event::KeyCode;
use ggez::graphics::{Align, Color, Scale, Text, TextFragment};
use ggez::nalgebra::{Point2, Vector2};

use crate::control::ProgramState;
use crate::inputs::Input;

const TEXT_SCALE_DOWN: f32 = 10.0;

const GRAY: graphics::Color = graphics::Color::new(0.5, 0.5, 0.5, 1.0);
const SELECT_GREEN: graphics::Color = graphics::Color::new(0.153, 0.839, 0.075, 1.0);

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
            num_players_text: Text::new(TextFragment {
                text: "Number of Players: 1".to_string(),
                color: Some(graphics::BLACK),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
            starting_level_text: Text::new(TextFragment {
                text: "Starting Level: 0".to_string(),
                color: Some(graphics::BLACK),
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
                ..Default::default()
            }),
        }
    }

    pub fn update(&mut self) -> ProgramState {
        if self.input.keydown_down.1 {
            self.set_select(self.selection, false);
            self.selection = (self.selection + 1) % NUM_MENUOPTION_ENTRIES;
            self.set_select(self.selection, true);
        }

        if self.input.keydown_up.1 {
            self.set_select(self.selection, false);
            self.selection = if self.selection == 0 {NUM_MENUOPTION_ENTRIES - 1} else {self.selection - 1};
            self.set_select(self.selection, true);
        }

        self.input.was_just_pressed_setfalse();
        ProgramState::Menu
    }

    fn set_select(&mut self, menu_option: u8, select_flag: bool) {
        match menu_option {
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
                } else {
                    self.num_players_text.fragments_mut()[0].color = Some(graphics::BLACK);
                }
            },
            x if x == MenuOption::StartingLevel as u8 => {
                if select_flag {
                    self.starting_level_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                } else {
                    self.starting_level_text.fragments_mut()[0].color = Some(graphics::BLACK);
                }
            },
            _ => println!("[!] menu_option didn't find match"),
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