use ggez::graphics;
use ggez::Context;

use crate::game::GameSettings;
use crate::inputs::Input;
use crate::menu::menuhelpers::draw_text;
use crate::menu::menuhelpers::TEXT_SCALE_DOWN;
use crate::menu::menuhelpers::{MenuItem, MenuItemTrigger};

use crate::game::board::BoardDim;

enum SettingsMenuItemId {
    Back,
    GhostPiecesState,
    BoardWidthPerPlayer,
    ExtraBoardWidth,
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
            MenuItem::new_numericalvalue(
                "Board Width Per Player: ",
                SettingsMenuItemId::BoardWidthPerPlayer as u8,
                starting_settings.board_width_per_player as u8,
                4,
                7,
                0,
                MenuItemTrigger::None,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
            ),
            MenuItem::new_numericalvalue(
                "Extra Board Width: ",
                SettingsMenuItemId::ExtraBoardWidth as u8,
                starting_settings.board_width_constant as u8,
                0,
                21,
                0,
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
        settings.board_width_per_player = self.get_board_width_per_player() as BoardDim;
        settings.board_width_constant = self.get_board_width_constant() as BoardDim;

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

    fn get_board_width_per_player(&self) -> u8 {
        for item in self.vec_menu_items.iter() {
            if item.id == SettingsMenuItemId::BoardWidthPerPlayer as u8 {
                return item.value;
            }
        }
        unreachable!("Failed to get board width per player in Menu::Settings");
    }

    fn get_board_width_constant(&self) -> u8 {
        for item in self.vec_menu_items.iter() {
            if item.id == SettingsMenuItemId::ExtraBoardWidth as u8 {
                return item.value;
            }
        }
        unreachable!("Failed to get extra board width in Menu::Settings");
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let window_dimensions = graphics::size(ctx);
        let num_menu_items_to_draw = self.vec_menu_items.len();

        for (index, item) in self.vec_menu_items.iter().enumerate() {
            draw_text(
                ctx,
                &item.text,
                (1.0 * (index + 1) as f32) / (num_menu_items_to_draw + 1) as f32,
                &window_dimensions,
            );
        }
    }

    pub fn resize_event(&mut self, height: f32) {
        for item in self.vec_menu_items.iter_mut() {
            item.resize(height);
        }
    }
}
