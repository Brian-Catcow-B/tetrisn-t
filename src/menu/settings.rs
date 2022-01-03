use crate::menu::menuhelpers::TEXT_SCALE_DOWN;
use crate::menu::menuhelpers::{MenuGameOptions, MenuItem, MenuItemTrigger, MenuItemValueType};

use crate::inputs::{Input, KeyboardControlScheme};

pub struct SettingsMenu {
    // logic
    selection: usize,
    // text
    vec_menu_items_main: Vec<MenuItem>,
}

enum SettingsMenuItemId {
    Back = 0,
    GhostPieces = 1,
}

impl SettingsMenu {
    pub fn new(game_options: &MenuGameOptions, window_dimensions: (f32, f32)) -> Self {
        let vec_menu_items_main = vec![
            MenuItem::new_novalue(
                "Back",
                SettingsMenuItemId::Back as u8,
                MenuItemTrigger::Back,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
            ),
            MenuItem::new_onoffvalue(
                "Ghost Pieces: ",
                SettingsMenuItemId::GhostPieces as u8,
                true,
                MenuItemTrigger::None,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
            ),
        ];
        Self {
            selection: 0,
            vec_menu_items_main,
        }
    }

    pub fn update(&mut self, input: &Input, game_options: &mut MenuGameOptions) -> bool {
        false
    }
}
