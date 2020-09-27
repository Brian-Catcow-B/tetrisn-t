use ggez::ContextBuilder;
use ggez::event;
use ggez::graphics;

// file systems stuff
use std::path;
use std::env;
use std::fs::File;
use ggez::filesystem::resources_dir;

// rustrisnt files
mod control;
use control::Control;

mod menu;
mod game;

mod inputs;

use gilrs;
use ggez::input::gamepad::GilrsGamepadContext;

fn main() {
    let mut context = ContextBuilder::new("Rustrisn-t", "Catcow")
        .window_setup(ggez::conf::WindowSetup::default().title("Rustrisn-t"));

    // file systems stuff
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        context = context.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut context.build().expect("[!] Failed to build context");

    // custom controller setup stuffs
    let mut gilrs_builder = gilrs::GilrsBuilder::new();

    match resources_dir(&ctx).join("gamecontrollerdb.txt").as_path().to_str() {
        Some(path) => {
            match std::fs::read_to_string(path) {
                Ok(string) => {
                    for line in string.split_ascii_whitespace() {
                        gilrs_builder = gilrs_builder.add_mappings(line);
                    }
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
    graphics::set_resizable(ctx, true).expect("[!] Failed to set window to resizable");
    graphics::set_drawable_size(ctx, 800.0, 600.0).expect("[!] Failed to resize window");

    // make it not blurry
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    // create an instance of the event handler
    let mut control = Control::new(ctx);

    // loop that controls the ProgramState
    match event::run(ctx, event_loop, &mut control) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("[!] Error occured: {}", e)
    }
}