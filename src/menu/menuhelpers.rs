use ggez::event::KeyCode;
use ggez::graphics::{self, Color, Font, Scale, Text, TextFragment};

use crate::inputs::KeyboardControlScheme;

pub const MAX_STARTING_LEVEL: u8 = 29; // this is just the fastest speed, so yeah
pub const MAX_NUM_PLAYERS: u8 = 62; // currently held back by board width being a u8 equal to 6 + 4 * num_players

pub const GRAY: Color = Color::new(0.4, 0.4, 0.4, 1.0);
pub const DARK_GRAY: Color = Color::new(0.3, 0.3, 0.3, 1.0);
pub const LIGHT_GRAY: Color = Color::new(0.6, 0.6, 0.6, 1.0);
pub const SELECT_GREEN: Color = Color::new(0.153, 0.839, 0.075, 1.0);
pub const HELP_RED: Color = Color::new(0.9, 0.11, 0.11, 1.0);

pub const TEXT_SCALE_DOWN: f32 = 15.0;
pub const SUB_TEXT_SCALE_DOWN: f32 = 25.0;

#[derive(Eq, PartialEq)]
pub enum MenuItemValueType {
    None,
    NumPlayers,
    StartingLevel,
    PlayerNum,
    KeyCode,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MenuItemTrigger {
    None,
    StartGame,
    SubMenu1,
    SubMenu2,
    Back,
    SubSelection,
    KeyLeft,
    KeyRight,
    KeyDown,
    KeyRotateCw,
    KeyRotateCcw,
    KeyBoardCw,
    KeyBoardCcw,
}

pub struct MenuItem {
    pub text: Text,
    pub value_type: MenuItemValueType,
    max_value: u8,
    pub value: u8,
    pub keycode: Option<KeyCode>,
    value_show_increase: u8,
    text_scale_down: f32,
    pub trigger: MenuItemTrigger,
    selected: bool,
}

impl MenuItem {
    pub fn new(
        item_start_str: &str,
        value_type: MenuItemValueType,
        value: u8,
        keycode: Option<KeyCode>,
        window_height: f32,
        text_scale_down: f32,
        trigger: MenuItemTrigger,
    ) -> Self {
        let mut text = Text::new(TextFragment::new(item_start_str).color(graphics::BLACK));
        let mut max_value = 0;
        let mut value_show_increase = 0;
        match value_type {
            MenuItemValueType::None => {}
            MenuItemValueType::NumPlayers => {
                max_value = MAX_NUM_PLAYERS;
                value_show_increase = 1;
                text.add(
                    TextFragment::new(format!(" {}", value + value_show_increase))
                        .color(graphics::BLACK),
                );
            }
            MenuItemValueType::StartingLevel => {
                max_value = MAX_STARTING_LEVEL;
                text.add(TextFragment::new(format!(" {}", value)).color(graphics::BLACK));
            }
            MenuItemValueType::PlayerNum => {
                max_value = MAX_NUM_PLAYERS;
                value_show_increase = 1;
                text.add(
                    TextFragment::new(format!(" {}", value + value_show_increase))
                        .color(graphics::BLACK),
                );
            }
            MenuItemValueType::KeyCode => {
                match keycode {
                    Some(key) => {
                        text.add(TextFragment::new(format!("{:?}", key)).color(graphics::BLACK))
                    }
                    None => text.add(TextFragment::new("None".to_string()).color(graphics::BLACK)),
                };
            }
        }
        text.set_font(
            Font::default(),
            Scale::uniform(window_height / text_scale_down),
        );
        Self {
            text,
            value_type,
            max_value,
            value,
            keycode,
            value_show_increase,
            text_scale_down,
            trigger,
            selected: false,
        }
    }

    pub fn set_select(&mut self, select: bool) {
        self.selected = select;
        self.text.fragments_mut()[0].color = Some(if select {
            SELECT_GREEN
        } else {
            graphics::BLACK
        });
        if self.value_type != MenuItemValueType::None {
            self.text.fragments_mut()[1].color = Some(if select {
                SELECT_GREEN
            } else {
                graphics::BLACK
            });
            if self.value_type == MenuItemValueType::NumPlayers
                || self.value_type == MenuItemValueType::StartingLevel
                || self.value_type == MenuItemValueType::PlayerNum
            {
                self.text.fragments_mut()[1].text = if select {
                    format!("<{}>", self.value + self.value_show_increase)
                } else {
                    format!(" {}", self.value + self.value_show_increase)
                };
            }
        }
    }

    pub fn inc_or_dec(&mut self, inc: bool) {
        if self.value_type == MenuItemValueType::NumPlayers
            || self.value_type == MenuItemValueType::StartingLevel
            || self.value_type == MenuItemValueType::PlayerNum
        {
            self.value = if inc {
                (self.value + 1) % self.max_value
            } else {
                (self.value - 1 + self.max_value) % self.max_value
            };
            // assume it's selected because it's being incremented/decremented
            self.text.fragments_mut()[1].text =
                format!("<{}>", self.value + self.value_show_increase);
        }
    }

    pub fn set_keycode(&mut self, keycode: Option<KeyCode>) {
        self.keycode = keycode;
        match self.keycode {
            Some(key) => self.text.fragments_mut()[1].text = format!("{:?}", key),
            None => self.text.fragments_mut()[1].text = "None".to_string(),
        };
    }

    pub fn resize(&mut self, window_height: f32) {
        self.text.set_font(
            Font::default(),
            Scale::uniform(window_height / self.text_scale_down),
        );
    }
}

pub struct MenuGameOptions {
    pub num_players: u8,
    pub starting_level: u8,
    pub arr_controls: Vec<(KeyboardControlScheme, bool)>,
}

impl MenuGameOptions {
    pub fn new() -> Self {
        let arr_controls: Vec<(KeyboardControlScheme, bool)> =
            vec![(KeyboardControlScheme::default(), false); MAX_NUM_PLAYERS as usize];
        Self {
            num_players: 1,
            starting_level: 0,
            arr_controls,
        }
    }
}
