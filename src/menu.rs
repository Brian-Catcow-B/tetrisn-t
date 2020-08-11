use ggez::Context;
use ggez::event::KeyCode;

use crate::control::ProgramState;

pub struct Menu {
    // state: MenuState
    selection: u8,
}

impl Menu {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            selection: 0,
        }
    }

    pub fn update(&mut self) -> ProgramState {
        ProgramState::Menu
    }

    pub fn key_down_event(&mut self, keycode: KeyCode, repeat: bool) {
        println!("hey we are in the keydown for menu. wow!");
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        println!("hey we are in the keyup for menu. wow!");
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        println!("hey wer drawing the menu. wow!");
    }
}