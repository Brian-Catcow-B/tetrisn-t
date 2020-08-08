use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods};
use ggez::graphics::{self, DrawParam, spritebatch};
use ggez::nalgebra as na;
use na::Point2;
use na::Vector2;

// file systems stuff
use std::path;
use std::env;

mod game;
use game::Game;

mod controls;
use controls::ControlScheme;

pub const CLEAR_DELAY: i8 = 60i8;

pub const SCORE_SINGLE_BASE: u8 = 40u8;
pub const SCORE_DOUBLE_BASE: u8 = 100u8;
pub const SCORE_TRIPLE_BASE: u16 = 300u16;
pub const SCORE_QUADRUPLE_BASE: u16 = 1200u16;

const BOARD_HEIGHT: u8 = 20u8;

// space up of the board that is not the board in tiles
pub const NON_BOARD_SPACE_U: u8 = 4u8;

fn main() {
    let mut context = ContextBuilder::new("Rustrisn-t", "Catcow");

    // file systems stuff
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        context = context.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut context.build().expect("Failed to build context");

    // set window size
    graphics::set_resizable(ctx, true).expect("Failed to set window to resizable");
    graphics::set_drawable_size(ctx, 800.0, 600.0).expect("Failed to resize window");

    // make it not blurry
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut game = Game::new(ctx, 2u8, 0u8);

    // Run!
    match event::run(ctx, event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}