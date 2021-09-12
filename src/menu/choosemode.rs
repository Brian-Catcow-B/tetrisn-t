use ggez::graphics::{self, DrawParam, Text};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::game::GameMode;
use crate::inputs::Input;
use crate::menu::menuhelpers::TEXT_SCALE_DOWN;
use crate::menu::menuhelpers::{MenuItem, MenuItemTrigger, MenuItemValueType};

pub struct ChooseModeMenu {
    // logic
    selection: usize,
    pub game_mode: GameMode,
    // drawing
    vec_menu_items: Vec<MenuItem>,
}

impl ChooseModeMenu {
    pub fn new(game_mode: GameMode, window_dimensions: (f32, f32)) -> Self {
        let mut vec_menu_items: Vec<MenuItem> = Vec::with_capacity(1);
        vec_menu_items.push(MenuItem::new(
            "Mode: ",
            MenuItemValueType::Custom,
            match game_mode {
                GameMode::Classic => 0,
                GameMode::Rotatris => 1,
            },
            None,
            window_dimensions.1,
            TEXT_SCALE_DOWN,
            MenuItemTrigger::SubMenu,
        ));
        vec_menu_items[0].set_num_values(2);
        vec_menu_items[0].text.fragments_mut()[1].text = format!("{:?}", game_mode);
        vec_menu_items[0].set_select(true);
        Self {
            // logic
            selection: 0,
            game_mode: game_mode,
            vec_menu_items,
        }
    }

    pub fn update(&mut self, input: &Input) -> MenuItemTrigger {
        if input.keydown_left.1
            || input.keydown_right.1
            || input.keydown_down.1
            || input.keydown_up.1
        {
            self.vec_menu_items[self.selection].inc_or_dec(true);
            self.game_mode = GameMode::from(self.vec_menu_items[self.selection].value as usize);
            self.vec_menu_items[self.selection].text.fragments_mut()[1].text =
                format!("{:?}", self.game_mode);
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
