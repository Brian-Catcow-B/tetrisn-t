# Rustrisn-t
Tetrisn-t rewritten in Rust, better.

`rustrisn-t.exe` is included on each tagged `master` branch commit so that Windows users don't have to compile the code.

Running the binary will create a folder where the binary is, called `resources`, which will eventually be used to hold config files and such.

Make sure your monitor's refresh rate is set to 60FPS; otherwise the game runs at a different speed than intended.

# Build
Download and install cargo, then
```
cargo build --release
```
Google any errors that occur, if they do.

# Controllers
Controllers are not yet supported, but eventually they will be. There's a fancy Controls menu that allows for keyboard mappings, though.
Unfortunately, saving keyboard control schemes to a file hasn't been implemented, so each time you open the program you must reconfigure keyboard controls.

<!-- Rustrisn-t uses SDL2 gamepad configurations for controllers.  
A cross-platform SDL2 configurer can be found here: https://generalarcade.com/gamepadtool/ -->