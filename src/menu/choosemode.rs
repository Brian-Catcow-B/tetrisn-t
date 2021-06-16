use ggez::event::KeyCode;
use ggez::graphics::{self, DrawParam, Scale, Text, TextFragment};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::game::GameMode;
use crate::inputs::Input;
use crate::menu::menuhelpers::{MenuGameOptions, MenuItem, MenuItemTrigger, MenuItemValueType};
use crate::menu::menuhelpers::{HELP_RED, TEXT_SCALE_DOWN};

pub struct ChooseModeMenu {
    // logic
    selection: usize,
    pub game_mode: GameMode,
    // drawing
    vec_menu_items: Vec<MenuItem>,
}

impl ChooseModeMenu {
    pub fn new(window_dimensions: (f32, f32)) -> Self {
        let mut vec_menu_items: Vec<MenuItem> = Vec::with_capacity(1);
        vec_menu_items.push(MenuItem::new(
            "Mode: ",
            MenuItemValueType::Custom,
            0,
            None,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::StartGame,
        ));
        vec_menu_items[0].set_num_values(2);
        vec_menu_items[0].set_select(true);
        Self {
            // logic
            selection: 0,
            game_mode: GameMode::from_usize(0),
            vec_menu_items,
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

    pub fn draw(&mut self, ctx: &mut Context) {
        let window_dimensions = graphics::size(ctx);
        let num_menu_items_to_draw = self.vec_menu_items.len();

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
