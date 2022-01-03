use crate::menu::menuhelpers::{MenuGameOptions, MenuItem, MenuItemTrigger, MenuItemValueType};

pub struct SettingsMenu {
    // logic
    selection: usize,
    // text
    vec_menu_items_main: Vec<MenuItem>,
}

impl SettingsMenu {
    pub fn new(game_options: &MenuGameOptions, window_dimensions: (f32, f32)) -> Self {
        let vec_menu_items_main = vec![
            MenuItem::new(
                "Back",
                MenuItemValueType::None,
                0,
                None,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
                MenuItemTrigger::Back
            ),
            MenuItem::new(
                "Ghost Pieces: ",
                MenuItemValueType::Custom,
                0,
                None,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
                MenuItemTrigger::Back
            ),
        ];
        Self {
            selection: 0,
            vec_menu_items_main,
        }
    }

    pub fn update(&mut self, input: &Input, game_options: &mut MenuGameOptions) -> bool {

    }
}
