use ggez::graphics::{self, DrawParam, Text};
use ggez::mint::Point2;
use ggez::Context;

use crate::game::GameSettings;
use crate::inputs::Input;
use crate::menu::menuhelpers::TEXT_SCALE_DOWN;
use crate::menu::menuhelpers::{MenuItem, MenuItemTrigger};

enum SettingsMenuItemId {
    Back,
    GhostPiecesState,
}

pub struct SettingsMenu {
    // logic
    selection: usize,
    vec_menu_items: Vec<MenuItem>,
}

impl SettingsMenu {
    pub fn new(starting_settings: &GameSettings, window_dimensions: (f32, f32)) -> Self {
        let mut vec_menu_items: Vec<MenuItem> = vec![
            MenuItem::new_novalue(
                "Back",
                SettingsMenuItemId::Back as u8,
                MenuItemTrigger::Back,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
            ),
            MenuItem::new_onoffvalue(
                "Ghost Pieces: ",
                SettingsMenuItemId::GhostPiecesState as u8,
                starting_settings.ghost_pieces_state,
                MenuItemTrigger::None,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
            ),
        ];
        vec_menu_items[0].set_select(true);
        Self {
            // logic
            selection: 0,
            vec_menu_items,
        }
    }

    pub fn update(&mut self, input: &Input, settings: &mut GameSettings) -> MenuItemTrigger {
        if input.keydown_rotate_ccw.1 {
            // escape was pressed
            return MenuItemTrigger::Back;
        }

        if input.keydown_right.1 {
            self.vec_menu_items[self.selection].change_val(true);
        }

        if input.keydown_left.1 {
            self.vec_menu_items[self.selection].change_val(false);
        }

        settings.ghost_pieces_state = self.get_ghost_pieces_state();

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
            return self.vec_menu_items[self.selection].trigger.clone();
        }

        MenuItemTrigger::None
    }

    fn get_ghost_pieces_state(&self) -> bool {
        for item in self.vec_menu_items.iter() {
            if item.id == SettingsMenuItemId::GhostPiecesState as u8 {
                return item.on;
            }
        }
        unreachable!("Failed to get ghost pieces state in Menu::Settings");
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
        let text_var_dimensions = text_var.dimensions(ctx);
        graphics::draw(
            ctx,
            text_var,
            DrawParam::new().dest(Point2::from_slice(&[
                (window_dimensions.0 - text_var_dimensions.w as f32) / 2.0,
                (window_dimensions.1 - text_var_dimensions.h as f32) * vertical_position,
            ])),
        )
        .unwrap();
    }

    pub fn resize_event(&mut self, height: f32) {
        for item in self.vec_menu_items.iter_mut() {
            item.resize(height);
        }
    }
}
