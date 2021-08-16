use ggez::event::EventHandler;
use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use crate::game::{Game, GameOptions};
use crate::menu::{menuhelpers::MenuGameOptions, Menu};

static STATE_MENU_BUT_MENU_NONE: &str =
    "[!] control.state == ProgramState::Menu but control.menu == None";
static STATE_GAME_BUT_GAME_NONE: &str =
    "[!] control.state == ProgramState::Game but control.game == None";

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ProgramState {
    Menu,
    Game,
}

pub struct Control {
    state: ProgramState,
    menu: Option<Menu>,
    game: Option<Game>,
    game_options: MenuGameOptions,
}

impl Control {
    pub fn new(ctx: &mut Context) -> Control {
        let menu_game_options = MenuGameOptions::default();
        Self {
            state: ProgramState::Menu,
            menu: Some(Menu::new(ctx, &menu_game_options)),
            game: None,
            game_options: menu_game_options,
        }
    }

    fn change_state(&mut self, ctx: &mut Context, new_state: ProgramState) {
        self.state = match new_state {
            ProgramState::Menu => {
                self.menu = Some(Menu::new(ctx, &self.game_options));
                ProgramState::Menu
            }
            ProgramState::Game => {
                self.game = Some(Game::new(ctx, &GameOptions::from(&self.game_options)));
                ProgramState::Game
            }
        };
    }
}

// this is run once every frame and passes control off to whichever state the game is in
impl EventHandler for Control {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            match self.state {
                ProgramState::Menu => {
                    // update the menu and get the state with GameOptions if ProgramState is changing
                    if let Some(new_state) = self
                        .menu
                        .as_mut()
                        .expect(STATE_MENU_BUT_MENU_NONE)
                        .update(&mut self.game_options)
                    {
                        self.menu = None;
                        self.change_state(ctx, new_state);
                    }
                }
                ProgramState::Game => {
                    // update the game and get the state that the program should be in
                    let state_returned =
                        self.game.as_mut().expect(STATE_GAME_BUT_GAME_NONE).update();
                    // should we change states?
                    if self.state != state_returned {
                        self.game = None;
                        self.change_state(ctx, state_returned);
                    }
                }
            };
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        match self.state {
            ProgramState::Menu => self
                .menu
                .as_mut()
                .expect(STATE_MENU_BUT_MENU_NONE)
                .key_down_event(keycode, repeat),
            ProgramState::Game => self
                .game
                .as_mut()
                .expect(STATE_GAME_BUT_GAME_NONE)
                .key_down_event(keycode, repeat),
        };
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match self.state {
            ProgramState::Menu => self
                .menu
                .as_mut()
                .expect(STATE_MENU_BUT_MENU_NONE)
                .key_up_event(keycode),
            ProgramState::Game => self
                .game
                .as_mut()
                .expect(STATE_GAME_BUT_GAME_NONE)
                .key_up_event(keycode),
        };
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, btn: Button, id: GamepadId) {
        match self.state {
            ProgramState::Menu => (),
            ProgramState::Game => self
                .game
                .as_mut()
                .expect(STATE_GAME_BUT_GAME_NONE)
                .gamepad_button_down_event(btn, id),
        };
    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, btn: Button, id: GamepadId) {
        match self.state {
            ProgramState::Menu => (),
            ProgramState::Game => self
                .game
                .as_mut()
                .expect(STATE_GAME_BUT_GAME_NONE)
                .gamepad_button_up_event(btn, id),
        };
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, id: GamepadId) {
        match self.state {
            ProgramState::Menu => (),
            ProgramState::Game => self
                .game
                .as_mut()
                .expect(STATE_GAME_BUT_GAME_NONE)
                .gamepad_axis_event(axis, value, id),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            ProgramState::Menu => self
                .menu
                .as_mut()
                .expect(STATE_MENU_BUT_MENU_NONE)
                .draw(ctx, &self.game_options),
            ProgramState::Game => self
                .game
                .as_mut()
                .expect(STATE_GAME_BUT_GAME_NONE)
                .draw(ctx),
        };

        graphics::present(ctx)
    }

    // this seems unused but is called somewhere in ggez to ultimately make things scale and get placed correctly when changing window size
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();

        match self.state {
            ProgramState::Menu => self
                .menu
                .as_mut()
                .expect(STATE_MENU_BUT_MENU_NONE)
                .resize_event((width, height)),
            ProgramState::Game => self
                .game
                .as_mut()
                .expect(STATE_GAME_BUT_GAME_NONE)
                .resize_event(width, height),
        };
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if self.state == ProgramState::Game {
            self.game
                .as_mut()
                .expect(STATE_GAME_BUT_GAME_NONE)
                .focus_event(gained);
        }
    }
}
