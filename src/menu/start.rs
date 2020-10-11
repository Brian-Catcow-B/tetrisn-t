use ggez::graphics::{self, DrawParam};
use ggez::graphics::{Scale, Text, TextFragment};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::inputs::Input;

use crate::menu::MAX_NUM_PLAYERS;
use crate::menu::MAX_STARTING_LEVEL;

use crate::menu::SUB_TEXT_SCALE_DOWN;
use crate::menu::TEXT_SCALE_DOWN;

use crate::menu::HELP_RED;
use crate::menu::SELECT_GREEN;

const NUM_STARTMENUOPTION_TEXT_ENTRIES: u8 = 4;
#[repr(u8)]
enum StartMenuOption {
    Start,
    NumPlayers,
    StartingLevel,
    Controls,
}

pub struct StartMenu {
    // logic
    selection: u8,
    pub num_players: u8,
    pub starting_level: u8,
    pub not_enough_controls_flag: bool,
    // drawing
    start_text: Text,
    not_enough_controls_text: Text,
    num_players_text: Text,
    starting_level_text: Text,
    controls_text: Text,
}

impl StartMenu {
    pub fn new(window_dimensions: (f32, f32), num_players: u8, starting_level: u8) -> Self {
        let mut num_players_text = Text::new(
            TextFragment::new("Number of Players: ")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
        );
        num_players_text.add(
            TextFragment::new(format!("{}", num_players))
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
        );
        let mut starting_level_text = Text::new(
            TextFragment::new("Starting Level: ")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
        );
        starting_level_text.add(
            TextFragment::new(format!("{}", starting_level))
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
        );
        Self {
            selection: StartMenuOption::Start as u8,
            num_players,
            starting_level,
            not_enough_controls_flag: false,
            start_text: Text::new(
                TextFragment::new("Start")
                    .color(SELECT_GREEN)
                    .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ),
            not_enough_controls_text: Text::new(
                TextFragment::new("[!] Not enough controls setup to start")
                    .color(HELP_RED)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ),
            num_players_text,
            starting_level_text,
            controls_text: Text::new(
                TextFragment::new("Controls")
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ),
        }
    }

    pub fn update(&mut self, input: &Input) -> (bool, bool) {
        if input.keydown_right.1 {
            self.inc_or_dec_selection(true);
        }

        if input.keydown_left.1 {
            self.inc_or_dec_selection(false);
        }

        if input.keydown_down.1 {
            self.set_select(false);
            self.selection = (self.selection + 1) % NUM_STARTMENUOPTION_TEXT_ENTRIES;
            self.set_select(true);
        }

        if input.keydown_up.1 {
            self.set_select(false);
            self.selection = if self.selection == 0 {
                NUM_STARTMENUOPTION_TEXT_ENTRIES - 1
            } else {
                self.selection - 1
            };
            self.set_select(true);
        }

        if input.keydown_start.1 && self.selection == StartMenuOption::Controls as u8 {
            self.not_enough_controls_flag = false;
            return (false, true);
        }

        if input.keydown_start.1 && self.selection == StartMenuOption::Start as u8 {
            return (true, false);
        }
        (false, false)
    }

    fn set_select(&mut self, select_flag: bool) {
        match self.selection {
            x if x == StartMenuOption::Start as u8 => {
                if select_flag {
                    self.start_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                } else {
                    self.start_text.fragments_mut()[0].color = Some(graphics::BLACK);
                }
            }
            x if x == StartMenuOption::NumPlayers as u8 => {
                if select_flag {
                    self.num_players_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                    self.num_players_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    self.num_players_text.fragments_mut()[1].text =
                        format!("<{}>", self.num_players);
                } else {
                    self.num_players_text.fragments_mut()[0].color = Some(graphics::BLACK);
                    self.num_players_text.fragments_mut()[1].color = Some(graphics::BLACK);
                    self.num_players_text.fragments_mut()[1].text =
                        format!(" {}", self.num_players);
                }
            }
            x if x == StartMenuOption::StartingLevel as u8 => {
                if select_flag {
                    self.starting_level_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                    self.starting_level_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    self.starting_level_text.fragments_mut()[1].text =
                        format!("<{}>", self.starting_level);
                } else {
                    self.starting_level_text.fragments_mut()[0].color = Some(graphics::BLACK);
                    self.starting_level_text.fragments_mut()[1].color = Some(graphics::BLACK);
                    self.starting_level_text.fragments_mut()[1].text =
                        format!(" {}", self.starting_level);
                }
            }
            x if x == StartMenuOption::Controls as u8 => {
                if select_flag {
                    self.controls_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                } else {
                    self.controls_text.fragments_mut()[0].color = Some(graphics::BLACK);
                }
            }
            _ => println!("[!] main_menu_option didn't find match"),
        }
    }

    fn inc_or_dec_selection(&mut self, inc_flag: bool) {
        // the if/else here only includes StartMenuOptions that have a value that can be modified
        if self.selection == StartMenuOption::NumPlayers as u8 {
            // special case (index by 1 because we can't have 0 players)
            if inc_flag {
                self.num_players = self.num_players % MAX_NUM_PLAYERS + 1;
            } else {
                self.num_players = if self.num_players == 1 {
                    MAX_NUM_PLAYERS
                } else {
                    self.num_players - 1
                };
            }
            self.num_players_text.fragments_mut()[1].text = format!("<{}>", self.num_players);
        } else if self.selection == StartMenuOption::StartingLevel as u8 {
            if inc_flag {
                self.starting_level = (self.starting_level + 1) % (MAX_STARTING_LEVEL + 1);
            } else {
                self.starting_level = if self.starting_level == 0 {
                    MAX_STARTING_LEVEL
                } else {
                    self.starting_level - 1
                };
            }
            self.starting_level_text.fragments_mut()[1].text = format!("<{}>", self.starting_level);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let window_dimensions = graphics::size(ctx);

        if self.not_enough_controls_flag {
            self.draw_text(ctx, &self.not_enough_controls_text, 0.1, &window_dimensions);
        }
        self.draw_text(ctx, &self.start_text, 0.2, &window_dimensions);
        self.draw_text(ctx, &self.num_players_text, 0.4, &window_dimensions);
        self.draw_text(ctx, &self.starting_level_text, 0.6, &window_dimensions);
        self.draw_text(ctx, &self.controls_text, 0.8, &window_dimensions);
    }

    fn draw_text(
        &self,
        ctx: &mut Context,
        text_var: &Text,
        vertical_position: f32,
        window_dimensions: &(f32, f32),
    ) {
        let (text_width, text_height) = text_var.dimensions(ctx);
        graphics::draw(
            ctx,
            text_var,
            DrawParam::new().dest(Point2::new(
                (window_dimensions.0 - text_width as f32) / 2.0,
                (window_dimensions.1 - text_height as f32) * vertical_position,
            )),
        )
        .unwrap();
    }
}
