use ggez::input::gamepad::gilrs;
use ggez::ContextBuilder;

// file systems stuff
use std::env;
use std::path;

// tetrisn-t files
mod control;
use control::Control;

mod game;
mod menu;

mod inputs;
mod movement;

use ggez::input::gamepad::GamepadContext;

fn main() {
    let mut context = ContextBuilder::new("Tetrisn-t", "Catcow")
        .window_setup(ggez::conf::WindowSetup::default().title("Tetrisn't"));

    // file systems stuff
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        context = context.add_resource_path(path);
    }

    let (mut ctx, event_loop) = context.build().expect("[!] Failed to build context");

    // custom controller setup stuffs
    let mut gilrs_builder = gilrs::GilrsBuilder::new().add_included_mappings(false);

    ctx.fs.read_dir("/").expect("[!] Couldn't read dir \"/\"");

    match ctx.fs.read_to_string("gamecontrollerdb.txt") {
        Ok(contents) => {
            gilrs_builder = gilrs_builder.add_mappings(&contents);
        }
        Err(e) => println!(
            "[!] file 'resources/gamecontrollerdb.txt' could not be read: {}",
            e
        ),
    }
    ctx.gamepad = GamepadContext::from(gilrs_builder.build().unwrap());

    // set window size
    ctx.gfx
        .set_resizable(true)
        .expect("[!] Failed to set window to resizable");
    ctx.gfx
        .set_drawable_size(800.0, 600.0)
        .expect("[!] Failed to resize window");

    // make it not blurry ???
    //ctx.gfx.set_default_filter(graphics::FilterMode::Nearest);

    // create an instance of the event handler
    let control = Control::new(&mut ctx);

    // loop that controls the ProgramState
    ggez::event::run(ctx, event_loop, control).expect("[!] game error");
}
