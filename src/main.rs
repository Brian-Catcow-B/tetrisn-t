use ggez::event;
use ggez::graphics;
use ggez::ContextBuilder;

// file systems stuff
use ggez::filesystem::resources_dir;
use std::env;
use std::fs::File;
use std::path;

// tetrisn-t files
mod control;
use control::Control;

mod game;
mod menu;

mod inputs;

use ggez::input::gamepad::GilrsGamepadContext;

fn main() {
    let mut context = ContextBuilder::new("Rotatris", "Catcow")
        .window_setup(ggez::conf::WindowSetup::default().title("Rotatris"));

    // file systems stuff
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("[+] Adding path {:?}", path);
        context = context.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut context.build().expect("[!] Failed to build context");

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
    graphics::set_resizable(ctx, true).expect("[!] Failed to set window to resizable");
    graphics::set_drawable_size(ctx, 800.0, 600.0).expect("[!] Failed to resize window");

    // make it not blurry
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    // create an instance of the event handler
    let mut control = Control::new(ctx);

    // loop that controls the ProgramState
    if let Err(e) = event::run(ctx, event_loop, &mut control) {
        println!("[!] Error occured: {}", e);
    }
}
