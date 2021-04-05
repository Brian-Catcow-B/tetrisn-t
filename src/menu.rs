use ggez::event::KeyCode;
use ggez::graphics::{self, Color, Text, TextFragment};
use ggez::Context;

use crate::control::ProgramState;
use crate::inputs::{Input, KeyboardControlScheme};

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
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum MenuItemTrigger {
    None,
    StartGame,
    SubMenu1,
    SubMenu2,
    Back,
}

pub struct MenuItem {
    pub text: Text,
    pub value_type: MenuItemValueType,
    max_value: u8,
    pub value: u8,
    value_show_increase: u8,
    pub trigger: MenuItemTrigger,
    selected: bool,
}

impl MenuItem {
    pub fn new(
        item_start_str: &str,
        value_type: MenuItemValueType,
        value: u8,
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
        }
        Self {
            text,
            value_type,
            max_value,
            value,
            value_show_increase,
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
            self.text.fragments_mut()[1].text = if select {
                format!(" <{}>", self.value + self.value_show_increase)
            } else {
                format!(" {}", self.value + self.value_show_increase)
            };
        }
    }

    pub fn inc_or_dec(&mut self, inc: bool) {
        if self.value_type != MenuItemValueType::None {
            self.value = if inc {
                (self.value + 1) % self.max_value
            } else {
                (self.value - 1 + self.max_value) % self.max_value
            };
            // assume it's selected because it's being incremented/decremented
            self.text.fragments_mut()[1].text =
                format!(" <{}>", self.value + self.value_show_increase);
        }
    }
}

pub struct MenuGameOptions {
    pub num_players: u8,
    pub starting_level: u8,
    pub arr_controls: [(Option<KeyboardControlScheme>, bool); MAX_NUM_PLAYERS as usize],
}

impl MenuGameOptions {
    fn new(
        num_players: u8,
        starting_level: u8,
        arr_split_controls: [(
            Option<(
                Option<KeyCode>,
                Option<KeyCode>,
                Option<KeyCode>,
                Option<KeyCode>,
                Option<KeyCode>,
            )>,
            bool,
        ); MAX_NUM_PLAYERS as usize],
    ) -> Self {
        let mut arr_controls: [(Option<KeyboardControlScheme>, bool); MAX_NUM_PLAYERS as usize] =
            [(None, false); MAX_NUM_PLAYERS as usize];
        for (idx, ctrls) in arr_split_controls.iter().enumerate() {
            if let Some(k_ctrls) = ctrls.0 {
                arr_controls[idx] = (
                    Some(KeyboardControlScheme::new(
                        k_ctrls.0.expect(
                            "[!] attempted to create KeyboardControlScheme with Left == None",
                        ),
                        k_ctrls.1.expect(
                            "[!] attempted to create KeyboardControlScheme with Right == None",
                        ),
                        k_ctrls.2.expect(
                            "[!] attempted to create KeyboardControlScheme with Down == None",
                        ),
                        k_ctrls.3.expect(
                            "[!] attempted to create KeyboardControlScheme with RotateCw == None",
                        ),
                        k_ctrls.4.expect(
                            "[!] attempted to create KeyboardControlScheme with RotateCcw == None",
                        ),
                        KeyCode::Escape,
                    )),
                    false,
                );
            } else if ctrls.1 {
                arr_controls[idx] = (None, true);
            }
        }
        Self {
            num_players,
            starting_level,
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
    // states
    state: MenuState,
    start_menu: start::StartMenu,
    input_config_menu: inputconfig::InputConfigMenu,
}

impl Menu {
    pub fn new(ctx: &mut Context, last_used_game_options: &Option<MenuGameOptions>) -> Self {
        let window_dimensions = graphics::size(ctx);
        if let Some(menu_game_options) = last_used_game_options {
            // previously used
            Self {
                input: Input::new(),
                state: MenuState::Start,
                start_menu: StartMenu::new(
                    menu_game_options.num_players,
                    menu_game_options.starting_level,
                ),
                input_config_menu: InputConfigMenu::new(
                    window_dimensions,
                    menu_game_options.arr_controls,
                ),
            }
        } else {
            // defaults
            Self {
                input: Input::new(),
                state: MenuState::Start,
                start_menu: StartMenu::new(1, 0),
                input_config_menu: InputConfigMenu::new(
                    window_dimensions,
                    [(None, false); MAX_NUM_PLAYERS as usize],
                ),
            }
        }
    }

    pub fn update(&mut self) -> Option<(ProgramState, MenuGameOptions)> {
        match self.state {
            MenuState::Start => {
                let trigger: MenuItemTrigger = self.start_menu.update(&self.input);
                match trigger {
                    MenuItemTrigger::StartGame => {
                        if self.ensure_enough_controls() {
                            let (num_players, starting_level) =
                                self.start_menu.find_important_values();
                            return Some((
                                ProgramState::Game,
                                MenuGameOptions::new(
                                    num_players,
                                    starting_level,
                                    self.input_config_menu.arr_split_controls,
                                ),
                            ));
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
                if self.input_config_menu.update(&self.input) {
                    self.state = MenuState::Start;
                }
            }
        }

        self.input.was_just_pressed_setfalse();
        None
    }

    fn ensure_enough_controls(&self) -> bool {
        let mut ctrls_count = 0;
        for ctrls in self.input_config_menu.arr_split_controls.iter() {
            if ctrls.0.is_some() || ctrls.1 {
                ctrls_count += 1;
            }
        }
        ctrls_count >= self.start_menu.find_important_values().0
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

    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, GRAY);

        match self.state {
            MenuState::Start => self.start_menu.draw(ctx),
            MenuState::InputConfig => self.input_config_menu.draw(ctx),
        }
    }
}
