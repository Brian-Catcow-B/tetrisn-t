use ggez::event::KeyCode;
use ggez::graphics::{self, Color, Font, PxScale, Text, TextFragment};

use crate::game::{GameMode, GameSettings};
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

pub static GAME_MODE_UNEXPECTEDLY_NONE: &str = "[!] GameMode unexpectedly None";

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MenuState {
    ChooseMode,
    Start,
    Settings,
    InputConfig,
}

#[repr(u8)]
#[derive(Eq, PartialEq)]
pub enum MenuItemValueType {
    None,
    OnOff,
    Numerical,
    KeyCode,
    Custom,
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MenuItemTrigger {
    None,
    StartGame,
    SubMenu(MenuState),
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
    // data
    pub text: Text,
    pub id: u8,
    pub on: bool,
    pub value: u8,
    min_value: u8,
    num_values: u8,
    value_show_increase: u8,
    pub keycode: Option<KeyCode>,
    pub trigger: MenuItemTrigger,
    selected: bool,
    value_type: MenuItemValueType,
    // draw
    text_scale_down: f32,
}

impl MenuItem {
    fn onoffstr(on: bool) -> &'static str {
        match on {
            true => "on",
            false => "off",
        }
    }

    pub fn new_novalue(
        title: &str,
        id: u8,
        trigger: MenuItemTrigger,
        window_height: f32,
        text_scale_down: f32,
    ) -> Self {
        let mut text = Text::new(TextFragment::new(title).color(graphics::Color::BLACK));
        text.set_font(
            Font::default(),
            PxScale::from(window_height / text_scale_down),
        );
        Self {
            text,
            id,
            on: true,
            value: 0u8,
            min_value: 0u8,
            num_values: 0u8,
            value_show_increase: 0u8,
            keycode: None,
            trigger,
            selected: false,
            value_type: MenuItemValueType::None,
            text_scale_down,
        }
    }

    pub fn new_onoffvalue(
        title: &str,
        id: u8,
        start_on: bool,
        trigger: MenuItemTrigger,
        window_height: f32,
        text_scale_down: f32,
    ) -> Self {
        let mut text = Text::new(TextFragment::new(title).color(graphics::Color::BLACK));
        text.add(TextFragment::new(Self::onoffstr(start_on)).color(graphics::Color::BLACK));
        text.set_font(
            Font::default(),
            PxScale::from(window_height / text_scale_down),
        );
        Self {
            text,
            id,
            on: start_on,
            value: 0u8,
            min_value: 0u8,
            num_values: 0u8,
            value_show_increase: 0u8,
            keycode: None,
            trigger,
            selected: false,
            value_type: MenuItemValueType::OnOff,
            text_scale_down,
        }
    }

    pub fn new_numericalvalue(
        title: &str,
        id: u8,
        start_value: u8,
        min_value: u8,
        num_values: u8,
        value_show_increase: u8,
        trigger: MenuItemTrigger,
        window_height: f32,
        text_scale_down: f32,
    ) -> Self {
        let mut text = Text::new(TextFragment::new(title).color(graphics::Color::BLACK));
        text.add(
            TextFragment::new(format!("{}", start_value + value_show_increase))
                .color(graphics::Color::BLACK),
        );
        text.set_font(
            Font::default(),
            PxScale::from(window_height / text_scale_down),
        );
        Self {
            text,
            id,
            on: true,
            value: start_value,
            min_value,
            num_values,
            value_show_increase,
            keycode: None,
            trigger,
            selected: false,
            value_type: MenuItemValueType::Numerical,
            text_scale_down,
        }
    }

    pub fn new_keycodevalue(
        title: &str,
        id: u8,
        opt_start_keycode: Option<KeyCode>,
        trigger: MenuItemTrigger,
        window_height: f32,
        text_scale_down: f32,
    ) -> Self {
        let mut text = Text::new(TextFragment::new(title).color(graphics::Color::BLACK));
        text.add(match opt_start_keycode {
            Some(key) => TextFragment::new(format!("{:?}", key)).color(graphics::Color::BLACK),
            None => TextFragment::new("None").color(graphics::Color::BLACK),
        });
        text.set_font(
            Font::default(),
            PxScale::from(window_height / text_scale_down),
        );
        Self {
            text,
            id,
            on: true,
            value: 0u8,
            min_value: 0u8,
            num_values: 0u8,
            value_show_increase: 0u8,
            keycode: opt_start_keycode,
            trigger,
            selected: false,
            value_type: MenuItemValueType::KeyCode,
            text_scale_down,
        }
    }

    pub fn new_customvalue(
        title: &str,
        id: u8,
        start_custom_str: &str,
        value: u8,
        num_values: u8,
        trigger: MenuItemTrigger,
        window_height: f32,
        text_scale_down: f32,
    ) -> Self {
        let mut text = Text::new(TextFragment::new(title).color(graphics::Color::BLACK));
        text.add(TextFragment::new(start_custom_str).color(graphics::Color::BLACK));
        text.set_font(
            Font::default(),
            PxScale::from(window_height / text_scale_down),
        );
        Self {
            text,
            id,
            on: true,
            value,
            min_value: 0u8,
            num_values,
            value_show_increase: 0u8,
            keycode: None,
            trigger,
            selected: false,
            value_type: MenuItemValueType::Custom,
            text_scale_down,
        }
    }

    pub fn set_select(&mut self, select: bool) {
        self.selected = select;
        self.text.fragments_mut()[0].color = Some(if select {
            SELECT_GREEN
        } else {
            graphics::Color::BLACK
        });
        if self.value_type != MenuItemValueType::None {
            self.text.fragments_mut()[1].color = Some(if select {
                SELECT_GREEN
            } else {
                graphics::Color::BLACK
            });
            if self.value_type == MenuItemValueType::Numerical {
                self.text.fragments_mut()[1].text = if select {
                    format!("<{}>", self.value + self.value_show_increase)
                } else {
                    format!(" {}", self.value + self.value_show_increase)
                };
            } else if self.value_type == MenuItemValueType::OnOff {
                self.text.fragments_mut()[1].text = if select {
                    format!("<{}>", Self::onoffstr(self.on))
                } else {
                    Self::onoffstr(self.on).to_string()
                }
            }
        }
    }

    pub fn change_val(&mut self, rightward_press: bool) {
        if self.value_type == MenuItemValueType::Numerical
            || self.value_type == MenuItemValueType::Custom
        {
            self.value = if rightward_press {
                // increment 1 with looping value
                ((self.value + 1 - self.min_value) % self.num_values) + self.min_value
            } else {
                // decrement 1 with looping value
                ((self.value - 1 + self.num_values - self.min_value) % self.num_values)
                    + self.min_value
            };
            if self.value_type != MenuItemValueType::Custom {
                // assume it's selected because it's being incremented/decremented
                self.text.fragments_mut()[1].text =
                    format!("<{}>", self.value + self.value_show_increase);
            }
        } else if self.value_type == MenuItemValueType::OnOff {
            self.on = !self.on;
            // assume it's selected because it's being swapped
            self.text.fragments_mut()[1].text = format!("<{}>", Self::onoffstr(self.on));
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
            PxScale::from(window_height / self.text_scale_down),
        );
    }
}

pub struct MenuGameOptions {
    pub num_players: u8,
    pub starting_level: u8,
    pub game_mode: GameMode,
    pub arr_controls: Vec<(KeyboardControlScheme, bool)>,
    pub settings: GameSettings,
}

impl Default for MenuGameOptions {
    fn default() -> Self {
        let arr_controls: Vec<(KeyboardControlScheme, bool)> =
            vec![(KeyboardControlScheme::default(), false); MAX_NUM_PLAYERS as usize];
        Self {
            num_players: 1,
            starting_level: 0,
            game_mode: GameMode::None,
            arr_controls,
            settings: GameSettings::default(),
        }
    }
}

impl MenuGameOptions {
    pub fn reset_controls(&mut self) {
        for ctrls in self.arr_controls.iter_mut() {
            ctrls.0.clear();
            ctrls.1 = false;
        }
    }
}
