use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods};
use ggez::graphics;

use crate::menu::Menu;
use crate::game::Game;

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum ProgramState {
    Menu,
    Game,
}

pub struct Control {
    state: ProgramState,
    menu: Option<Menu>,
    game: Option<Game>,
}

impl Control {
    pub fn new(ctx: &mut Context) -> Control {
        Self {
            state: ProgramState::Menu,
            menu: Some(Menu::new(ctx)),
            game: None,
        }
    }

    pub fn change_state(&mut self, ctx: &mut Context, new_state: ProgramState) {
        self.state = match new_state {
            ProgramState::Menu => {
                self.menu = Some(Menu::new(ctx));
                ProgramState::Menu
            },
            ProgramState::Game => {
                self.game = Some(Game::new(ctx, 2u8, 0u8));
                ProgramState::Game
            },
        };
    }
}

// this is run once every frame and passes control off to whichever state the game is in
impl EventHandler for Control {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            ProgramState::Menu => {
                // update the menu and get the state that the program should be in
                let state_returned = match self.menu.as_mut() {
                    Some(menu) => menu.update(),
                    None => {
                        println!("[!] control.state == ProgramState::Menu but control.menu == None");
                        panic!();
                    }
                };
                // should we change states?
                if self.state != state_returned {
                    self.menu = None;
                    self.change_state(ctx, state_returned);
                }
            },
            ProgramState::Game => {
                // update the game and get the state that the program should be in
                let state_returned = match self.game.as_mut() {
                    Some(game) => game.update(),
                    None => {
                        println!("[!] control.state == ProgramState::Game but control.game == None");
                        panic!();
                    }
                };
                // should we change states?
                if self.state != state_returned {
                    self.game = None;
                    self.change_state(ctx, state_returned);
                }
            },
        };

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
            ProgramState::Menu => match self.menu.as_mut() {
                Some(menu) => menu.key_down_event(keycode, repeat),
                None => {
                    println!("[!] control.state == ProgramState::Menu but control.menu == None");
                    panic!();
                }
            },
            ProgramState::Game => match self.game.as_mut() {
                Some(game) => game.key_down_event(keycode, repeat),
                None => {
                    println!("[!] control.state == ProgramState::Game but control.game == None");
                    panic!();
                }
            },
        };
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        match self.state {
            ProgramState::Menu => match self.menu.as_mut() {
                Some(menu) => menu.key_up_event(keycode),
                None => {
                    println!("[!] control.state == ProgramState::Menu but control.menu == None");
                    panic!();
                }
            },
            ProgramState::Game => match self.game.as_mut() {
                Some(game) => game.key_up_event(keycode),
                None => {
                    println!("[!] control.state == ProgramState::Game but control.game == None");
                    panic!();
                }
            },
        };
    }

    // fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    //     match self.state {
    //         ProgramState::Menu => match self.menu {
    //             Some(mut menu) => &menu.draw(ctx),
    //             None => {
    //                 println!("[!] control.state == ProgramState::Menu but control.menu == None");
    //                 panic!();
    //             }
    //         }
    //         ProgramState::Game => match self.game {
    //             Some(mut game) => &game.draw(ctx),
    //             None => {
    //                 println!("[!] control.state == ProgramState::Game but control.game == None");
    //                 panic!();
    //             }
    //         }
    //     };

    //     graphics::present(ctx)
    // }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::present(ctx)
    }

    // this seems unused but is called somewhere in ggez to ultimately make things scale and get placed correctly when changing window size
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}
