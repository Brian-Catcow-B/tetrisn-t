# Rustrisn-t
Tetrisn-t rewritten in Rust, better

# Build
```
cargo build --release
```
If that fails, giving the following error: `error: failed to run custom build command for 'alsa-sys v0.1.2'`, try
```
sudo apt-get install libsdl2-dev
```
and try again

# Controllers
There's a fancy Controls menu that allows for keyboard mappings

<!-- Rustrisn-t uses SDL2 gamepad configurations for controllers.  
A cross-platform SDL2 configurer can be found here: https://generalarcade.com/gamepadtool/

For mapping buttons and joysticks, the GUI of that gamepad tool will look like an Xbox 360 controller. When looking at an Xbox 360 controller,  
A -> RotateCcw  
B -> RotateCw  
Start -> Start  
DPAD_LEFT and AXIS_1_LEFT -> Left  
DPAD_RIGHT and AXIS_1_RIGHT -> Right  
DPAD_DOWN and AXIS_1_DOWN -> Down  
are the mappings into Rustrisnt controls. You will need to choose in the "Controls" menu which player is controlled by which controller.

Also, you can set keyboard controls in the "Controls" menu of the game.  
There is no guarantee any of this will work, and a restart may be required. -->