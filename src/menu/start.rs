use ggez::graphics::{self, DrawParam, Text, TextFragment};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::game::GameMode;
use crate::inputs::Input;
use crate::menu::menuhelpers::{MenuGameOptions, MenuItem, MenuItemTrigger, MenuItemValueType};
use crate::menu::menuhelpers::{HELP_RED, TEXT_SCALE_DOWN};

pub struct StartMenu {
    // logic
    selection: usize,
    game_mode: GameMode,
    pub not_enough_controls_flag: bool,
    vec_menu_items: Vec<MenuItem>,
    // drawing
    not_enough_controls_text: Text,
}

impl StartMenu {
    pub fn new(
        window_dimensions: (f32, f32),
        num_players: u8,
        starting_level: u8,
        game_mode: GameMode,
    ) -> Self {
        let mut vec_menu_items: Vec<MenuItem> = Vec::with_capacity(4);
        let mut selection = 0;
        Self::fill_vec_menu_items(
            &mut vec_menu_items,
            game_mode,
            &mut selection,
            num_players,
            starting_level,
            window_dimensions,
        );
        vec_menu_items[0].set_select(true);
        Self {
            // logic
            selection,
            not_enough_controls_flag: false,
            game_mode,
            vec_menu_items,
            // drawing
            not_enough_controls_text: Text::new(
                TextFragment::new("[!] Not enough Controls Setup to Start").color(HELP_RED),
            ),
        }
    }

    pub fn update(&mut self, input: &Input, game_options: &mut MenuGameOptions) -> MenuItemTrigger {
        if input.keydown_rotate_ccw.1 {
            // escape was pressed; go back to the choosemode menu after resetting some things
            self.not_enough_controls_flag = false;
            return MenuItemTrigger::Back;
        }

        if input.keydown_right.1 {
            self.vec_menu_items[self.selection].inc_or_dec(true);
        }

        if input.keydown_left.1 {
            self.vec_menu_items[self.selection].inc_or_dec(false);
        }

        game_options.num_players = self.get_num_players() + 1;
        game_options.starting_level = self.get_starting_level();

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

    pub fn set_game_mode(
        &mut self,
        mode: GameMode,
        game_options: &MenuGameOptions,
        window_dimensions: (f32, f32),
    ) {
        if self.game_mode != mode {
            self.game_mode = mode;
            self.vec_menu_items.clear();
            Self::fill_vec_menu_items(
                &mut self.vec_menu_items,
                mode,
                &mut self.selection,
                game_options.num_players,
                game_options.starting_level,
                window_dimensions,
            );
        }
    }

    fn fill_vec_menu_items(
        vec_menu_items: &mut Vec<MenuItem>,
        mode: GameMode,
        selection: &mut usize,
        num_players: u8,
        starting_level: u8,
        window_dimensions: (f32, f32),
    ) {
        vec_menu_items.push(MenuItem::new(
            "Start",
            MenuItemValueType::None,
            0,
            None,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::StartGame,
        ));

        if mode == GameMode::Classic {
            vec_menu_items.push(MenuItem::new(
                "Number of Players: ",
                MenuItemValueType::NumPlayers,
                num_players - 1,
                None,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
                MenuItemTrigger::StartGame,
            ));
        }
        vec_menu_items.push(MenuItem::new(
            "Starting Level: ",
            MenuItemValueType::StartingLevel,
            starting_level,
            None,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::StartGame,
        ));
        vec_menu_items.push(MenuItem::new(
            "Controls",
            MenuItemValueType::None,
            0,
            None,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::SubMenu,
        ));
        *selection = 0;
        vec_menu_items[0].set_select(true);
    }

    fn get_num_players(&self) -> u8 {
        for item in self.vec_menu_items.iter() {
            if item.value_type == MenuItemValueType::NumPlayers {
                return item.value;
            }
        }
        0u8
    }

    fn get_starting_level(&self) -> u8 {
        for item in self.vec_menu_items.iter() {
            if item.value_type == MenuItemValueType::StartingLevel {
                return item.value;
            }
        }
        0u8
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
