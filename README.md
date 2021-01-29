# Tetrisn-t
Tetrisn-t (python version) rewritten in Rust, better.

Binaries for Windows and Linux are included in each tagged release.

# Build
Download and install cargo, then set rustup default to 1.47.0 to avoid an error in the ggez library introduced with rust 1.48.0 by running
```
rustup default 1.47.0
```
then build with
```
cargo build --release
```
and find the resulting binary in `./target/release/`

Google any errors that occur, if they do.

# Gamepads
In the "Controls" menu, keyboard control schemes and setting players to use gamepads are both possible and mostly self explanatory.
It is possible to connect multiple keyboards to one PC and use both separately, but the inputs show up as the same, so the keyboard control schemes are not allowed to overlap, even across separate keyboards.
There is a check in place, which will give a notice when a key is attempted to be re-used.
Unfortunately, these settings are not saved yet, so they will have to be reconfigured each time the program is opened; eventually, saved control schemes will be implemented.

Setting a player to use a gamepad is easily done in the "Controls" menu by pressing 'G', as it states.
The game decides which specific gamepad controls which player by assigning a gamepad's id to the first player that doesn't yet have an assigned gamepad id as soon as an unassigned gamepad has a button/axis pressed during gameplay.
Because of weird coding stuff, this is done each game no matter what, but might eventually be extended to be the entire instance of the program.

## Custom Layouts and Obscure Compatibility
### Windows
Because ggez only supports xinput, a program like rewasd is required for non-xinput controllers, which allows remapping of non-xinput controllers to act as an xinput controller.
rewasd does cost money (unless you download an earlier version from 3rd party), but here's an alternative: http://ds4windows.com/

### Linux
For linux, this program uses SDL2 gamepad configurations for controllers.
A list of automatically-detected controllers can be found in the `resources` folder in `gamecontrollerdb.txt`, which is based on this file: https://github.com/gabomdq/SDL_GameControllerDB/blob/master/gamecontrollerdb.txt
The `resources` folder with its contents must be in the same location as the binary on launch in order for gamepads to be detected; create a shortcut to the binary if you want to open it from elsewhere.

In case a gamepad is not detected or a different layout is desired, here is a cross-platform SDL2 configurer: https://generalarcade.com/gamepadtool/. A quick guide on how to use it is as follows:
1. Download, install, open
2. Select desired gamepad in the dropdown on the top
3. Select "Create A New Mapping"
4. Create the desired mapping using input on your gamepad
5. Select "Copy Mapping String" and paste the string into a newline of `gamecontrollerdb.txt`, deleting the line of the controller with the same Gamepad GUID (the first really long number) if it exists

Then the gamepad should be recognized when the program is opened again. When creating a gamepad mapping, consider which buttons the program has set to do which action:
```
Axis::LeftAxisX- and Button::DPadLeft -> Left
Axis::LeftAxisX+ and Button::DPadRight -> Right
Axis::LeftAxisY- and Button::DPadDown -> Down
Button::West -> RotateCw
Button::South -> RotateCcw
Button::Start -> Start
```
where, in the graphic, `Button::Start` is the small button just to the right of the circle button in the middle, and the compass directions refer to the four buttons on the right.
