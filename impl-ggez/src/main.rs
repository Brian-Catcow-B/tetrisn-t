use ggez::graphics;
use ggez::input::gamepad::gilrs;
use ggez::ContextBuilder;
use ggez::input::gamepad::GilrsGamepadContext;

// file systems stuff
use ggez::filesystem::resources_dir;
use std::env;
use std::fs::File;
use std::path;

// tetrisn-t files
mod control;
use control::Control;
mod menu;

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

    match resources_dir(&ctx).join("gamecontrollerdb.txt").as_path().to_str() {
        Some(path) => {
            match std::fs::read_to_string(path) {
                Ok(string) => {
                    gilrs_builder = gilrs_builder.add_mappings(&string);
                },
                Err(_) => {
                    println!("[!] couldn't read contents of {} to string; creating file; no custom controller layouts will function", path);
                    if let Err(e) = File::create(resources_dir(&ctx).join("gamecontrollerdb.txt")) {
                        println!("[!] failed to create file {}: {}", resources_dir(&ctx).join("gamecontrollerdb.txt").display(), e);
                    }
                },
            }
        },
        None => println!("[!] couldn't obtain path for gamecontrollerdb.txt; no custom controller layouts will function"),
    }
    ctx.gamepad_context = Box::new(GilrsGamepadContext::from(gilrs_builder.build().unwrap()));

    // set window size
    graphics::set_resizable(&mut ctx, true).expect("[!] Failed to set window to resizable");
    graphics::set_drawable_size(&mut ctx, 800.0, 600.0).expect("[!] Failed to resize window");

    // make it not blurry
    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    // create an instance of the event handler
    let control = Control::new(&mut ctx);

    // loop that controls the ProgramState
    ggez::event::run(ctx, event_loop, control)
}
