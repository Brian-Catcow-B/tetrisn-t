use ggez::ContextBuilder;
use ggez::event;
use ggez::graphics;

// file systems stuff
use std::path;
use std::env;

// game and controls
mod game;
use game::Game;

mod controls;

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

    // create an instance of the event handler
    let mut game = Game::new(ctx, 2u8, 0u8);

    // menu/game loop
    while ctx.continuing {
        match event::run(ctx, event_loop, &mut game) {
            Ok(_) => println!("Exited cleanly."),
            Err(e) => println!("Error occured: {}", e)
        }
    }
}