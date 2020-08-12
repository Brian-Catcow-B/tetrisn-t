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

const NUM_MENUOPTION_ENTRIES = 3;
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
            
        }

        ProgramState::Menu
    }

    fn set_select(&mut self, menu_option: MneuOption, select_flag: bool) {
        match menu_option {
            Start => {
                if select_flag {
                    self.
                }
            },
            NumPlayers => {
                if select_flag {
                    self.
                }
            },
            StartingLevel => {
                if select_flag {
                    self.
                }
            },
        }
    }

    pub fn key_down_event(&mut self, keycode: KeyCode, repeat: bool) {
        println!("hey we are in the keydown for menu. wow!");
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        println!("hey we are in the keyup for menu. wow!");
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