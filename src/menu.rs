use ggez::event::KeyCode;
use ggez::graphics;
use ggez::Context;

use crate::control::ProgramState;
use crate::inputs::Input;

mod choosemode;
mod inputconfig;
pub mod menuhelpers;
mod start;
use choosemode::ChooseModeMenu;
use inputconfig::InputConfigMenu;
use menuhelpers::GRAY;
use menuhelpers::{MenuGameOptions, MenuItemTrigger};
use start::StartMenu;

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum MenuState {
    ChooseMode,
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
    choose_mode_menu: ChooseModeMenu,
    start_menu: StartMenu,
    input_config_menu: InputConfigMenu,
    // window size
    window_dimensions: (f32, f32),
}

impl Menu {
    pub fn new(ctx: &mut Context, game_options: &MenuGameOptions) -> Self {
        let window_dimensions = graphics::size(ctx);
        Self {
            input: Input::new(),
            num_required_keycode_movement_pairs: game_options.game_mode.num_required_inputs(),
            state: MenuState::ChooseMode,
            choose_mode_menu: ChooseModeMenu::new(game_options.game_mode, window_dimensions),
            start_menu: StartMenu::new(
                window_dimensions,
                game_options.num_players,
                game_options.starting_level,
                game_options.game_mode,
            ),
            input_config_menu: InputConfigMenu::new(game_options, window_dimensions),
            window_dimensions,
        }
    }

    pub fn update(&mut self, game_options: &mut MenuGameOptions) -> Option<ProgramState> {
        match self.state {
            MenuState::ChooseMode => {
                if self.choose_mode_menu.update(&self.input) == MenuItemTrigger::SubMenu {
                    self.state = MenuState::Start;
                    if game_options.game_mode != self.choose_mode_menu.game_mode {
                        game_options.game_mode = self.choose_mode_menu.game_mode;
                        self.num_required_keycode_movement_pairs =
                            game_options.game_mode.num_required_inputs();
                        self.start_menu.set_game_mode(
                            self.choose_mode_menu.game_mode,
                            game_options,
                            self.window_dimensions,
                        );
                        self.input_config_menu
                            .update_game_mode(self.window_dimensions, game_options);
                    }
                }
            }
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
                    MenuItemTrigger::SubMenu => {
                        // InputConfig menu
                        self.start_menu.not_enough_controls_flag = false;
                        self.state = MenuState::InputConfig;
                    }
                    MenuItemTrigger::Back => {
                        self.state = MenuState::ChooseMode;
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
            MenuState::ChooseMode => self.choose_mode_menu.draw(ctx),
            MenuState::Start => self.start_menu.draw(ctx),
            MenuState::InputConfig => self.input_config_menu.draw(ctx, game_options),
        }
    }

    pub fn resize_event(&mut self, window_dims: (f32, f32)) {
        self.window_dimensions = window_dims;
        match self.state {
            MenuState::ChooseMode => self.choose_mode_menu.resize_event(window_dims.1),
            MenuState::Start => self.start_menu.resize_event(window_dims.1),
            MenuState::InputConfig => {}
        }
    }
}
