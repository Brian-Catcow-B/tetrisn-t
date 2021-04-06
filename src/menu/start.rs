use ggez::graphics::{self, DrawParam};
use ggez::graphics::{Text, TextFragment};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::inputs::Input;
use crate::menu::{MenuItem, MenuItemTrigger, MenuItemValueType};

use crate::menu::TEXT_SCALE_DOWN;

use crate::menu::HELP_RED;

pub struct StartMenu {
    // logic
    selection: usize,
    pub not_enough_controls_flag: bool,
    // drawing
    vec_menu_items: Vec<MenuItem>,
    not_enough_controls_text: Text,
}

impl StartMenu {
    pub fn new(window_dimensions: (f32, f32), num_players: u8, starting_level: u8) -> Self {
        let mut vec_menu_items: Vec<MenuItem> = Vec::with_capacity(4);
        vec_menu_items.push(MenuItem::new(
            "Start",
            MenuItemValueType::None,
            0,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::StartGame,
        ));
        vec_menu_items.push(MenuItem::new(
            "Number of Players: ",
            MenuItemValueType::NumPlayers,
            num_players - 1,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::StartGame,
        ));
        vec_menu_items.push(MenuItem::new(
            "Starting Level: ",
            MenuItemValueType::StartingLevel,
            starting_level,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::StartGame,
        ));
        vec_menu_items.push(MenuItem::new(
            "Controls",
            MenuItemValueType::None,
            0,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::SubMenu1,
        ));
        vec_menu_items[0].set_select(true);
        Self {
            // logic
            selection: 0,
            not_enough_controls_flag: false,
            vec_menu_items,
            // drawing
            not_enough_controls_text: Text::new(
                TextFragment::new("[!] Not enough Controls Setup to Start").color(HELP_RED),
            ),
        }
    }

    pub fn update(&mut self, input: &Input) -> MenuItemTrigger {
        if input.keydown_right.1 {
            self.vec_menu_items[self.selection].inc_or_dec(true);
        }

        if input.keydown_left.1 {
            self.vec_menu_items[self.selection].inc_or_dec(false);
        }

        if input.keydown_down.1 {
            self.vec_menu_items[self.selection].set_select(false);
            self.selection = (self.selection + 1) % self.vec_menu_items.len();
            self.vec_menu_items[self.selection].set_select(true);
        }

        if input.keydown_up.1 {
            self.vec_menu_items[self.selection].set_select(false);
            self.selection = if self.selection == 0 {
                self.vec_menu_items.len() - 1
            } else {
                self.selection - 1
            };
            self.vec_menu_items[self.selection].set_select(true);
        }

        if input.keydown_start.1 {
            return self.vec_menu_items[self.selection].trigger;
        }

        MenuItemTrigger::None
    }

    // searches through self.vec_menu_items and gets the important values; returns (num_players, starting_level)
    pub fn find_important_values(&self) -> (u8, u8) {
        let mut num_players = 0;
        let mut starting_level = 0;
        for item in self.vec_menu_items.iter() {
            match item.value_type {
                MenuItemValueType::None => {}
                MenuItemValueType::NumPlayers => num_players = item.value + 1,
                MenuItemValueType::StartingLevel => starting_level = item.value,
            }
        }

        (num_players, starting_level)
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let window_dimensions = graphics::size(ctx);
        let num_menu_items_to_draw = self.vec_menu_items.len();

        if self.not_enough_controls_flag {
            self.draw_text(ctx, &self.not_enough_controls_text, 0.1, &window_dimensions);
        }
        for (index, item) in self.vec_menu_items.iter().enumerate() {
            self.draw_text(
                ctx,
                &item.text,
                (1.0 * (index + 1) as f32) / (num_menu_items_to_draw + 1) as f32,
                &window_dimensions,
            );
        }
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

    pub fn resize_event(&mut self, height: f32) {
        for item in self.vec_menu_items.iter_mut() {
            item.resize(height);
        }
    }
}
