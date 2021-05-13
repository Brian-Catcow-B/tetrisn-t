use ggez::event::KeyCode;
use ggez::graphics::{self, Color, Font, Scale, Text, TextFragment};
use ggez::Context;

use crate::control::ProgramState;
use crate::inputs::{Input, KeyboardControlScheme};
// use crate::movement::Movement;

pub const MAX_STARTING_LEVEL: u8 = 29; // this is just the fastest speed, so yeah
pub const MAX_NUM_PLAYERS: u8 = 62; // currently held back by board width being a u8 equal to 6 + 4 * num_players

pub const TEXT_SCALE_DOWN: f32 = 15.0;
pub const SUB_TEXT_SCALE_DOWN: f32 = 25.0;

const GRAY: Color = Color::new(0.4, 0.4, 0.4, 1.0);
pub const DARK_GRAY: Color = Color::new(0.3, 0.3, 0.3, 1.0);
pub const LIGHT_GRAY: Color = Color::new(0.6, 0.6, 0.6, 1.0);
pub const SELECT_GREEN: Color = Color::new(0.153, 0.839, 0.075, 1.0);
pub const HELP_RED: Color = Color::new(0.9, 0.11, 0.11, 1.0);

mod inputconfig;
mod start;
use inputconfig::InputConfigMenu;
use start::StartMenu;

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

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum MenuState {
    Start,
    InputConfig,
}

// we just have all the menu stuffs loaded into here because they're all connected and it's not much memory anyways
pub struct Menu {
    // logic
    input: Input,
    num_required_keycode_movement_pairs: usize,
    // states
    state: MenuState,
    start_menu: start::StartMenu,
    input_config_menu: inputconfig::InputConfigMenu,
}

impl Menu {
    pub fn new(ctx: &mut Context, game_options: &MenuGameOptions) -> Self {
        let window_dimensions = graphics::size(ctx);
        Self {
            input: Input::new(),
            num_required_keycode_movement_pairs: 5, // TODO
            state: MenuState::Start,
            start_menu: StartMenu::new(
                window_dimensions,
                game_options.num_players,
                game_options.starting_level,
            ),
            input_config_menu: InputConfigMenu::new(window_dimensions, game_options),
        }
    }

    pub fn update(&mut self, game_options: &mut MenuGameOptions) -> Option<ProgramState> {
        match self.state {
            MenuState::Start => {
                let trigger: MenuItemTrigger = self.start_menu.update(&self.input, game_options);
                match trigger {
                    MenuItemTrigger::StartGame => {
                        if self.ensure_enough_controls(game_options) {
                            return Some(ProgramState::Game);
                        } else {
                            self.start_menu.not_enough_controls_flag = true;
                        }
                    }
                    MenuItemTrigger::SubMenu1 => {
                        // InputConfig menu
                        self.start_menu.not_enough_controls_flag = false;
                        self.state = MenuState::InputConfig;
                    }
                    MenuItemTrigger::Back => {
                        println!("[!] what? 1");
                    }
                    MenuItemTrigger::None => {}
                    _ => println!("[!] Wrong menu?"),
                }
            }
            MenuState::InputConfig => {
                if self.input_config_menu.update(&self.input, game_options) {
                    self.state = MenuState::Start;
                }
            }
        }

        self.input.was_just_pressed_setfalse();
        None
    }

    fn ensure_enough_controls(&self, game_options: &MenuGameOptions) -> bool {
        let mut ctrls_count = 0;
        for ctrls in game_options.arr_controls.iter() {
            if ctrls.1 {
                ctrls_count += 1;
            } else if (ctrls.0).len() >= self.num_required_keycode_movement_pairs {
                ctrls_count += 1;
            }
        }
        ctrls_count >= game_options.num_players
    }

    pub fn key_down_event(&mut self, keycode: KeyCode, _repeat: bool) {
        self.input_config_menu.most_recently_pressed_key = Some(keycode);
        if keycode == KeyCode::Left {
            if !self.input.keydown_left.0 {
                self.input.keydown_left = (true, true);
            }
        } else if keycode == KeyCode::Right {
            if !self.input.keydown_right.0 {
                self.input.keydown_right = (true, true);
            }
        } else if keycode == KeyCode::Down {
            if !self.input.keydown_down.0 {
                self.input.keydown_down = (true, true);
            }
        } else if keycode == KeyCode::Up {
            if !self.input.keydown_up.0 {
                self.input.keydown_up = (true, true);
            }
        } else if keycode == KeyCode::G {
            if !self.input.keydown_rotate_cw.0 {
                self.input.keydown_rotate_cw = (true, true);
            }
        } else if keycode == KeyCode::Escape {
            if !self.input.keydown_rotate_ccw.0 {
                self.input.keydown_rotate_ccw = (true, true);
            }
        } else if (keycode == KeyCode::Space
            || keycode == KeyCode::Return
            || keycode == KeyCode::NumpadEnter)
            && !self.input.keydown_start.0
        {
            self.input.keydown_start = (true, true);
        }
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        if keycode == KeyCode::Left {
            self.input.keydown_left = (false, false);
        } else if keycode == KeyCode::Right {
            self.input.keydown_right = (false, false);
        } else if keycode == KeyCode::Down {
            self.input.keydown_down = (false, false);
        } else if keycode == KeyCode::Up {
            self.input.keydown_up = (false, false);
        } else if keycode == KeyCode::G {
            self.input.keydown_rotate_cw = (false, false);
        } else if keycode == KeyCode::Escape {
            self.input.keydown_rotate_ccw = (false, false);
        } else if keycode == KeyCode::Space
            || keycode == KeyCode::Return
            || keycode == KeyCode::NumpadEnter
        {
            self.input.keydown_start = (false, false);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, game_options: &MenuGameOptions) {
        graphics::clear(ctx, GRAY);

        match self.state {
            MenuState::Start => self.start_menu.draw(ctx),
            MenuState::InputConfig => self.input_config_menu.draw(ctx, game_options),
        }
    }

    pub fn resize_event(&mut self, height: f32) {
        match self.state {
            MenuState::Start => self.start_menu.resize_event(height),
            MenuState::InputConfig => {}
        }
    }
}
