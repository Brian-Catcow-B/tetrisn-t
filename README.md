# Tetrisn-t
Tetrisn-t (python version) rewritten in Rust, better.

Binaries for Windows and Linux are included in each tagged release.

# Build
Download and install cargo, then build with
```
cargo build --release
```
and find the resulting binary in `./target/release/`

Google any errors that occur, if they do (there are a few necessary libraries).

# Controls
In the "Controls" menu, keyboard control schemes and setting players to use gamepads are both possible and mostly self explanatory.
It is possible to connect multiple keyboards to one PC and use both separately, but the inputs show up as the same, so the keyboard control schemes are not allowed to overlap, even across separate keyboards.
There is a check in place, which will give a notice when a key is attempted to be re-used.
Unfortunately, these settings are not saved yet, so they will have to be reconfigured each time the program is opened.

Setting a player to use a gamepad is easily done in the "Controls" menu by pressing 'G', as it states.
The game decides which gamepad controls which player by assigning gamepads to players as inputs are made.
Due to easier coding, this is done each game no matter what.

## Custom Gamepad Layouts and Obscure Compatibility
### Windows
Because ggez only supports xinput for Windows, a program like rewasd is required for non-xinput controllers, which allows remapping of non-xinput controllers to act as an xinput controller.

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
Axis::LeftAxisX-/Button::DPadLeft  -> Left
Axis::LeftAxisX+/Button::DPadRight -> Right
Axis::LeftAxisY-/Button::DPadDown  -> Down
Button::East  -> RotateCw
Button::South -> RotateCcw
Button::North -> RotateBoardCw
Button::West  -> RotateBoardCcw
Button::Start -> Start
```
where, in the graphic, `Button::Start` is the small button just to the right of the circle button in the middle, and the compass directions refer to the four buttons on the right.
