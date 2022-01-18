use ggez::event::{Axis, Button, GamepadId, KeyCode};
use ggez::graphics::{self, spritebatch, DrawParam};
use ggez::graphics::{PxScale, Text, TextFragment};
use ggez::Context;

use ggez::mint::{Point2, Vector2};

use rand::random;

use crate::control::ProgramState;
use crate::movement::Movement;
use crate::movement::CONVERSION_FAILED_MOVEMENT_FROM_U8;

use std::convert::TryFrom;

mod player;
use crate::game::player::{Player, SPAWN_DELAY};

mod tile;
use crate::game::tile::TileGraphic;
use crate::game::tile::NUM_PIXEL_ROWS_PER_TILEGRAPHIC;

mod piece;
use crate::game::piece::{NextPiece, Shapes};

mod board;
use crate::game::board::BoardHandler;

use crate::inputs::KeyboardControlScheme;
use crate::menu::menuhelpers::MenuGameOptions;

const BOARD_HEIGHT: u8 = 20u8;

const ROTATRIS_BOARD_SIDE_LENGTH: u8 = 20u8;

pub const CLEAR_DELAY_CLASSIC: i8 = 30i8;

pub const SCORE_SINGLE_BASE: u8 = 40u8;
pub const SCORE_DOUBLE_BASE: u8 = 100u8;
pub const SCORE_TRIPLE_BASE: u16 = 300u16;
pub const SCORE_QUADRUPLE_BASE: u16 = 1200u16;

const GAME_OVER_DELAY: i8 = 60i8;

// space up of the board that is not the board in tiles
const NON_BOARD_SPACE_U: u8 = 4u8;
// space between the top of the board and the next piece in tiles
const BOARD_NEXT_PIECE_SPACING: u8 = 3;
// space down of the board that is not the board in tiles
const NON_BOARD_SPACE_D: u8 = 3u8;
// each tile is actually 8x8 pixels, so we scale down by 8 and then some because with 8.0, window resizing can clip off the bottom of the board
const TILE_SIZE_DOWN_SCALE: f32 = 8.5;

const LITTLE_TEXT_SCALE: f32 = 20.0;

// for each level (as the index), the number of frames it takes for a piece to move down one row (everything after 29 is also 0)
// it's actually 1 less than the number of frames it takes the piece to fall because the game logic works out better that way
const FALL_DELAY_VALUES_CLASSIC: [u8; 30] = [
    47, 42, 37, 32, 27, 22, 17, 12, 7, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0,
];
// rotatris is much harder, so slower fall rates than classic
const FALL_DELAY_VALUES_ROTATRIS: [u8; 30] = [
    60, 50, 40, 35, 30, 25, 20, 15, 10, 7, 6, 5, 4, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0,
];

// number of frames between downward movements when holding down
pub const FORCE_FALL_DELAY: u8 = 2;

// first das threshold (eg left is pressed; how many frames to until the piece auto-shifts left?)
const DAS_THRESHOLD_BIG: u8 = 14;
// second das threshold (eg left is pressed and it auto shifts once; how many frames until it auto-shifts again?)
const DAS_THRESHOLD_LITTLE: u8 = 5;

// how long the pieces don't move down at the start
pub const INITIAL_HANG_FRAMES: u8 = 180;

// gamepad joystick thresholds
pub const DETECT_GAMEPAD_AXIS_THRESHOLD: f32 = 0.5;
pub const UNDETECT_GAMEPAD_AXIS_THRESHOLD: f32 = 0.2;

static INVALID_MENU_CONTROLS: &str = "[!] last used controls was Some() but invalid data";
static GAME_MODE_NONE: &str = "[!] GameMode unexpectedly None";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GameMode {
    None,
    Classic,
    Rotatris,
}

impl GameMode {
    pub fn num_required_inputs(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Classic => 5,
            Self::Rotatris => 7,
        }
    }
}

impl From<usize> for GameMode {
    fn from(num: usize) -> Self {
        match num {
            0 => Self::Classic,
            1 => Self::Rotatris,
            _ => Self::Classic,
        }
    }
}

#[derive(Copy, Clone)]
pub struct GameSettings {
    pub ghost_pieces_state: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            ghost_pieces_state: true,
        }
    }
}

// this struct is for the Menu class so that it can return what game options to start the game with
pub struct GameOptions {
    pub num_players: u8,
    pub starting_level: u8,
    pub game_mode: GameMode,
    pub vec_controls: Vec<(Option<KeyboardControlScheme>, bool)>,
    pub settings: GameSettings,
}

impl From<&MenuGameOptions> for GameOptions {
    fn from(menu_game_options: &MenuGameOptions) -> Self {
        let mut vec_controls: Vec<(Option<KeyboardControlScheme>, bool)> =
            Vec::with_capacity(menu_game_options.arr_controls.len());
        let mut counted_active_controls: u8 = 0;
        match menu_game_options.game_mode {
            GameMode::None => unreachable!("{}", GAME_MODE_NONE),
            GameMode::Classic => {
                for ctrls in menu_game_options.arr_controls.iter() {
                    if !(ctrls.0).is_empty() {
                        vec_controls.push((
                            Some(KeyboardControlScheme::new_classic(
                                (ctrls.0)
                                    .keycode_from_movement(Movement::Left)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::Right)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::Down)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::RotateCw)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::RotateCcw)
                                    .expect(INVALID_MENU_CONTROLS),
                            )),
                            false,
                        ));
                        counted_active_controls += 1;
                    } else if ctrls.1 {
                        vec_controls.push((None, true));
                        counted_active_controls += 1;
                    }
                    if counted_active_controls == menu_game_options.num_players {
                        break;
                    }
                }
            }
            GameMode::Rotatris => {
                for ctrls in menu_game_options.arr_controls.iter() {
                    if !(ctrls.0).is_empty() {
                        vec_controls.push((
                            Some(KeyboardControlScheme::new_rotatris(
                                (ctrls.0)
                                    .keycode_from_movement(Movement::Left)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::Right)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::Down)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::RotateCw)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::RotateCcw)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::BoardCw)
                                    .expect(INVALID_MENU_CONTROLS),
                                (ctrls.0)
                                    .keycode_from_movement(Movement::BoardCcw)
                                    .expect(INVALID_MENU_CONTROLS),
                            )),
                            false,
                        ));
                        counted_active_controls += 1;
                    } else if ctrls.1 {
                        vec_controls.push((None, true));
                        counted_active_controls += 1;
                    }
                    if counted_active_controls == menu_game_options.num_players {
                        break;
                    }
                }
            }
        }

        Self {
            num_players: menu_game_options.num_players,
            starting_level: menu_game_options.starting_level,
            game_mode: menu_game_options.game_mode,
            vec_controls,
            settings: menu_game_options.settings,
        }
    }
}

pub struct Game {
    // GAME STUFF
    // logic (mostly)
    bh: BoardHandler,
    num_players: u8,
    vec_players: Vec<Player>,
    vec_next_piece: Vec<NextPiece>,
    vec_gamepad_id_map_to_player: Vec<(Option<GamepadId>, u8)>,
    num_gamepads_to_initialize: u8,
    level: u8,
    starting_level: u8,
    num_cleared_lines: u16,
    score: u64,
    keycode_down_flags: (bool, bool),
    keycode_escape_flags: (bool, bool),
    pause_flags: (bool, bool),
    gravity_direction: Movement,
    game_over_flag: bool,
    game_over_delay: i8,
    determine_ghost_tile_locations: bool,
    // drawing
    tile_size: f32,
    batch_empty_tile: spritebatch::SpriteBatch,
    batch_highlight_active_tile: spritebatch::SpriteBatch,
    batch_highlight_clearing_standard_tile: spritebatch::SpriteBatch,
    batch_highlight_clearing_tetrisnt_tile: spritebatch::SpriteBatch,
    batch_highlight_ghost_tile: spritebatch::SpriteBatch,
    vec_batch_player_piece: Vec<spritebatch::SpriteBatch>,
    vec_batch_next_piece: Vec<spritebatch::SpriteBatch>,
    game_info_text: Text,
    pause_text: Text,
    game_over_text: Text,
}

impl Game {
    pub fn new(ctx: &mut Context, game_options: &GameOptions) -> Game {
        let mode = game_options.game_mode;
        let board_width = match mode {
            GameMode::None => unreachable!("{}", GAME_MODE_NONE),
            GameMode::Classic => 6 + 4 * game_options.num_players,
            GameMode::Rotatris => ROTATRIS_BOARD_SIDE_LENGTH,
        };
        let board_height = match mode {
            GameMode::None => unreachable!("{}", GAME_MODE_NONE),
            GameMode::Classic => BOARD_HEIGHT,
            GameMode::Rotatris => ROTATRIS_BOARD_SIDE_LENGTH,
        };
        let mut vec_players: Vec<Player> = Vec::with_capacity(game_options.num_players as usize);
        for player in 0..game_options.num_players {
            // spawn_column
            let spawn_column: u8 = if player < game_options.num_players / 2 {
                // first half, not including middle player if there's an odd number of players
                (player as f32 * (board_width as f32 / game_options.num_players as f32)
                    + board_width as f32 / (2.0 * game_options.num_players as f32))
                    as u8
                    + 1
            } else if player == game_options.num_players / 2 && game_options.num_players % 2 == 1 {
                // middle player, for an odd number of players
                board_width / 2
            } else {
                // second half, not including the middle player if there's an odd number of players
                board_width
                    - 1
                    - ((game_options.num_players - 1 - player) as f32
                        * (board_width as f32 / game_options.num_players as f32)
                        + board_width as f32 / (2.0 * game_options.num_players as f32))
                        as u8
            };
            // control_scheme; we need to create a copy of game_options.vec_controls, but to do that, we must "manually" copy the keyboard controls for the player if they exist (since that has a vector)
            let control_scheme = match &game_options.vec_controls[player as usize].0 {
                Some(k_ctrl_scheme) => (Some(k_ctrl_scheme.copy()), false),
                None => (None, true),
            };

            vec_players.push(Player::new(player, control_scheme, spawn_column));
        }
        let mut batch_empty_tile = spritebatch::SpriteBatch::new(TileGraphic::new_empty(ctx).image);
        // the emtpy tile batch will be constant once the game starts with the player tile batches drawing on top of it, so just set that up here
        for x in 0..board_width {
            for y in 0..board_height as usize {
                // empty tiles
                let empty_tile = graphics::DrawParam::new().dest(Point2::from_slice(&[
                    x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                ]));
                batch_empty_tile.add(empty_tile);
            }
        }
        let mut vec_next_piece: Vec<NextPiece> =
            Vec::with_capacity(game_options.num_players as usize);
        let mut vec_gamepad_id_map_to_player: Vec<(Option<GamepadId>, u8)>;
        let mut temp_vec: Vec<(Option<GamepadId>, u8)> = vec![];
        let mut num_gamepads_to_initialize: u8 = 0;
        for (idx, controls) in game_options.vec_controls.iter().enumerate() {
            if controls.1 {
                temp_vec.push((None, idx as u8));
                num_gamepads_to_initialize += 1;
            }
        }
        if !temp_vec.is_empty() {
            vec_gamepad_id_map_to_player = Vec::with_capacity(temp_vec.len());
            vec_gamepad_id_map_to_player.append(&mut temp_vec);
        } else {
            vec_gamepad_id_map_to_player = Vec::with_capacity(1);
        }
        // for 1 player we have 3 sprite batches for player pieces because we have different color pieces
        let mut vec_batch_player_piece: Vec<spritebatch::SpriteBatch> =
            Vec::with_capacity(std::cmp::max(game_options.num_players as usize, 3));
        let mut vec_batch_next_piece: Vec<spritebatch::SpriteBatch> =
            Vec::with_capacity(std::cmp::max(game_options.num_players as usize, 3));
        for player in 0..std::cmp::max(game_options.num_players as usize, 3) {
            vec_next_piece.push(NextPiece::new(Shapes::None));
            vec_batch_player_piece.push(spritebatch::SpriteBatch::new(
                TileGraphic::new_player(ctx, player as u8).image,
            ));
            vec_batch_next_piece.push(spritebatch::SpriteBatch::new(
                TileGraphic::new_player(ctx, player as u8).image,
            ));
        }
        let little_text_scale = PxScale::from(LITTLE_TEXT_SCALE);
        let mut game_info_text = match mode {
            GameMode::None => unreachable!("{}", GAME_MODE_NONE),
            GameMode::Classic => Text::new(
                TextFragment::new("Lines: ")
                    .color(graphics::Color::WHITE)
                    .scale(little_text_scale),
            ),
            GameMode::Rotatris => Text::new(
                TextFragment::new("Rings: ")
                    .color(graphics::Color::WHITE)
                    .scale(little_text_scale),
            ),
        };
        game_info_text.add(
            TextFragment::new("000")
                .color(graphics::Color::WHITE)
                .scale(little_text_scale),
        );
        game_info_text.add(
            TextFragment::new("   Score: ")
                .color(graphics::Color::WHITE)
                .scale(little_text_scale),
        );
        game_info_text.add(
            TextFragment::new("0000000")
                .color(graphics::Color::WHITE)
                .scale(little_text_scale),
        );
        game_info_text.add(
            TextFragment::new("   Level: ")
                .color(graphics::Color::WHITE)
                .scale(little_text_scale),
        );
        game_info_text.add(
            TextFragment::new(format!("{:02}", game_options.starting_level))
                .color(graphics::Color::WHITE)
                .scale(little_text_scale),
        );
        let pause_text = Text::new(
            TextFragment::new("PAUSED\n\nDown + ESC/Start to quit")
                .color(graphics::Color::WHITE)
                .scale(little_text_scale),
        );
        let game_over_text = Text::new(
            TextFragment::new("Game Over!")
                .color(graphics::Color::WHITE)
                .scale(PxScale::from(LITTLE_TEXT_SCALE * 2.0)),
        );

        let (window_width, window_height) = graphics::size(ctx);

        Self {
            bh: BoardHandler::new(board_width, board_height, game_options.num_players, mode),
            num_players: game_options.num_players,
            vec_players,
            vec_next_piece,
            vec_gamepad_id_map_to_player,
            num_gamepads_to_initialize,
            level: game_options.starting_level,
            starting_level: game_options.starting_level,
            num_cleared_lines: 0u16,
            score: 0u64,
            keycode_down_flags: (false, false),
            keycode_escape_flags: (false, false),
            pause_flags: (false, false),
            gravity_direction: Movement::Down,
            game_over_flag: false,
            game_over_delay: GAME_OVER_DELAY,
            determine_ghost_tile_locations: game_options.settings.ghost_pieces_state,
            tile_size: TileGraphic::get_size(
                window_width,
                window_height,
                board_width,
                board_height + NON_BOARD_SPACE_U + NON_BOARD_SPACE_D,
            ),
            batch_empty_tile,
            batch_highlight_active_tile: spritebatch::SpriteBatch::new(
                TileGraphic::new_active_highlight(ctx).image,
            ),
            batch_highlight_clearing_standard_tile: spritebatch::SpriteBatch::new(
                TileGraphic::new_clear_standard_highlight(ctx).image,
            ),
            batch_highlight_clearing_tetrisnt_tile: spritebatch::SpriteBatch::new(
                TileGraphic::new_clear_tetrisnt_highlight(ctx).image,
            ),
            batch_highlight_ghost_tile: spritebatch::SpriteBatch::new(
                TileGraphic::new_ghost_highlight(ctx).image,
            ),
            vec_batch_player_piece,
            vec_batch_next_piece,
            game_info_text,
            pause_text,
            game_over_text,
        }
    }

    pub fn update(&mut self) -> ProgramState {
        if self.game_over_flag {
            if self.game_over_delay == 0 {
                // GAME OVER LOGIC
                if self.keycode_escape_flags.1 {
                    return ProgramState::Menu;
                }
                for player in &mut self.vec_players {
                    // should we quit to main menu?
                    if player.input.keydown_start.1 {
                        return ProgramState::Menu;
                    }
                }
                self.was_just_pressed_setfalse_all_players();
            } else {
                self.game_over_delay -= 1;
            }
        } else if self.pause_flags.0 {
            // PAUSE LOGIC
            if self.pause_flags.1 {
                // if the pause flag was just set, reset all inputs to false in case focus was lost or keyboard hardware is acting up somehow or another
                self.pause_flags.1 = false;
                for player in &mut self.vec_players {
                    player.input.reset_all();
                }
            } else {
                // down arrow key held and escape key results in quit to menu
                if self.keycode_escape_flags.1 && self.keycode_down_flags.0 {
                    return ProgramState::Menu;
                }
                // this loop is mostly due to gamepad/keyboard controls meshing together weirdly
                for player in &mut self.vec_players {
                    // should we quit to main menu? (down and start, but start on keyboard is Escape and not specific to a player, so check if players holding down are using keyboard)
                    if player.input.keydown_down.0
                        && (player.input.keydown_start.1
                            || ((player.control_scheme.0).is_some() && self.keycode_escape_flags.1))
                    {
                        return ProgramState::Menu;
                    }
                    // should we resume?
                    if player.input.keydown_start.1 {
                        self.pause_flags = (false, false);
                        break;
                    }
                }
                // should we resume because of escape press?
                if self.keycode_escape_flags.1 {
                    self.pause_flags = (false, false);
                }
                self.was_just_pressed_setfalse_all_players();
            }
        } else {
            // GAME LOGIC
            for player in &mut self.vec_players {
                if !player.spawn_piece_flag
                    && self.bh.get_shape_from_player(player.player_num) == Shapes::None
                {
                    player.input.was_just_pressed_setfalse();
                    continue;
                }

                // piece spawning
                if player.spawn_piece_flag {
                    if player.spawn_delay <= 0 {
                        // (blocked, blocked by some !active tile); if .1, game over sequence, if .0 and !.1, only blocked by other players, wait until they move, then carry on
                        let blocked: (bool, bool) = self.bh.attempt_piece_spawn(
                            player.player_num,
                            player.spawn_column,
                            player.next_piece_shape,
                        );
                        if blocked.0 {
                            if blocked.1 {
                                self.game_over_flag = true;
                            }
                            continue;
                        } else {
                            self.bh.playerify_piece(player.player_num);
                            player.spawn_delay = SPAWN_DELAY;
                            player.spawn_piece_flag = false;
                            // set das_countdown to the smaller das value if input left or right is pressed as the piece spawns in
                            if player.input.keydown_left.0 || player.input.keydown_right.0 {
                                player.das_countdown = DAS_THRESHOLD_LITTLE;
                            }
                            // set next piece to random; reroll once if it chooses the same piece as it just was
                            let mut rand: u8;
                            loop {
                                rand = random::<u8>();
                                if rand < 252 {
                                    break;
                                }
                            }
                            let random_shape =
                                Shapes::try_from(rand % 7).expect("Unable to get random piece");
                            if self.bh.get_shape_from_player(player.player_num) != random_shape {
                                player.next_piece_shape = random_shape;
                            } else {
                                let mut rand: u8;
                                loop {
                                    rand = random::<u8>();
                                    if rand < 252 {
                                        break;
                                    }
                                }
                                player.next_piece_shape =
                                    Shapes::try_from(rand % 7).expect("Unable to get random piece");
                            }
                            self.vec_next_piece[player.player_num as usize] =
                                NextPiece::new(player.next_piece_shape);
                            player.redraw_next_piece_flag = true;
                        }
                    } else {
                        player.spawn_delay -= 1;
                    }
                    continue;
                }

                // rotatris specific
                // board rotations
                if player.input.keydown_board_cw.1 {
                    // this is flipped because singleplayer and multiplayer will be different someday; TODO
                    if self.bh.attempt_rotate_board(Movement::RotateCcw) {
                        self.gravity_direction =
                            Movement::try_from(((self.gravity_direction as u8) + 3) % 4)
                                .expect(CONVERSION_FAILED_MOVEMENT_FROM_U8);
                    }
                }

                if player.input.keydown_board_ccw.1 {
                    // this is flipped because singleplayer and multiplayer will be different someday; TODO
                    if self.bh.attempt_rotate_board(Movement::RotateCw) {
                        self.gravity_direction =
                            Movement::try_from(((self.gravity_direction as u8) + 1) % 4)
                                .expect(CONVERSION_FAILED_MOVEMENT_FROM_U8);
                    }
                }
                // rotatris specific end

                // piece movement
                // LEFT / RIGHT
                if player.input.keydown_left.1 {
                    // if it didn't move on the initial input, set waiting_to_shift to true
                    player.waiting_to_shift = !self
                        .bh
                        .attempt_piece_movement(
                            Movement::try_from(
                                (Movement::Left as u8 + self.gravity_direction as u8) % 4,
                            )
                            .expect(CONVERSION_FAILED_MOVEMENT_FROM_U8),
                            player.player_num,
                        )
                        .0;
                    player.das_countdown = DAS_THRESHOLD_BIG;
                }
                if player.input.keydown_right.1 {
                    // if it didn't move on the initial input, set waiting_to_shift to true
                    player.waiting_to_shift = !self
                        .bh
                        .attempt_piece_movement(
                            Movement::try_from(
                                (Movement::Right as u8 + self.gravity_direction as u8) % 4,
                            )
                            .expect(CONVERSION_FAILED_MOVEMENT_FROM_U8),
                            player.player_num,
                        )
                        .0;
                    player.das_countdown = DAS_THRESHOLD_BIG;
                }
                if (player.input.keydown_left.0 && !player.input.keydown_left.1)
                    || (player.input.keydown_right.0 && !player.input.keydown_right.1)
                {
                    let movement: Movement = if player.input.keydown_left.0 {
                        Movement::Left
                    } else {
                        Movement::Right
                    };
                    if player.tick_das_countdown() {
                        // if the das countdown hit zero, we try to move the piece
                        if self
                            .bh
                            .attempt_piece_movement(
                                Movement::try_from(
                                    (movement as u8 + self.gravity_direction as u8) % 4,
                                )
                                .expect(CONVERSION_FAILED_MOVEMENT_FROM_U8),
                                player.player_num,
                            )
                            .0
                        {
                            // if the piece moved, set variables accordingly
                            player.das_countdown =
                                std::cmp::max(DAS_THRESHOLD_LITTLE, player.das_countdown);
                            player.waiting_to_shift = false;
                        } else {
                            // failed to move piece, so we are waiting to shift the piece
                            player.waiting_to_shift = true;
                        };
                    }
                }
                // CW / CCW
                if player.input.keydown_rotate_cw.1 {
                    self.bh
                        .attempt_piece_movement(Movement::RotateCw, player.player_num);
                }
                if player.input.keydown_rotate_ccw.1 {
                    self.bh
                        .attempt_piece_movement(Movement::RotateCcw, player.player_num);
                }
                // DOWN
                // down is interesting because every time the downwards position is false we have to check if it's running into the bottom or an inactive tile so we know if we should lock it
                if player.input.keydown_down.1
                    || (player.input.keydown_down.0 && player.force_fall_countdown == 0)
                    || player.fall_countdown == 0
                {
                    let (moved_flag, caused_full_line_flag): (bool, bool) =
                        self.bh.attempt_piece_movement(
                            Movement::try_from(
                                (Movement::Down as u8 + self.gravity_direction as u8) % 4,
                            )
                            .expect(CONVERSION_FAILED_MOVEMENT_FROM_U8),
                            player.player_num,
                        );
                    // if the piece got locked, piece.shape gets set to Shapes::None, so set the spawn piece flag
                    if self.bh.get_shape_from_player(player.player_num) == Shapes::None {
                        player.spawn_piece_flag = true;
                        player.fall_countdown = if self.level < 30 {
                            self.bh.get_fall_delay_from_level(self.level)
                        } else {
                            0
                        };
                        player.force_fall_countdown = FORCE_FALL_DELAY;
                        // add more spawn delay if locking the piece caused a line clear
                        if caused_full_line_flag {
                            player.spawn_delay += CLEAR_DELAY_CLASSIC as i16;
                        }
                    }
                    if moved_flag {
                        player.fall_countdown = if self.level < 30 {
                            self.bh.get_fall_delay_from_level(self.level)
                        } else {
                            0
                        };
                        player.force_fall_countdown = FORCE_FALL_DELAY;
                    }
                } else if player.input.keydown_down.0 {
                    player.force_fall_countdown -= 1;
                    player.fall_countdown -= 1;
                } else {
                    player.fall_countdown -= 1;
                }

                if player.input.keydown_start.1 {
                    self.pause_flags = (true, true);
                }

                // reset player controls in memory for next frame consistency
                player.input.was_just_pressed_setfalse();
            }

            // update controls so that the logic realizes next frame that the button inputs made were run through the logic
            if self.keycode_escape_flags.1 {
                self.pause_flags = (true, true);
            }
            self.was_just_pressed_setfalse_common();

            // attempt to line clear (go through the vector of FullLine's and decrement clear_delay if > 0, clear and return (lines_cleared, score) for <= 0)
            let (returned_lines, returned_score) = self.bh.attempt_clear(self.level);
            if returned_lines > 0 {
                self.num_cleared_lines += returned_lines as u16;
                self.game_info_text.fragments_mut()[1].text =
                    format!("{:03}", self.num_cleared_lines);
                self.score += returned_score as u64;
                self.game_info_text.fragments_mut()[3].text = format!("{:07}", self.score);
                let first_level_up_lines_amount: u16 = (self.starting_level as u16 + 1) * 10;
                let not_first_level_up_lines_amount: u16 = 10;
                if self.level == self.starting_level {
                    if self.num_cleared_lines >= first_level_up_lines_amount {
                        self.level += 1;
                    }
                } else if self.num_cleared_lines
                    >= first_level_up_lines_amount
                        + (self.level as u16 - self.starting_level as u16)
                            * not_first_level_up_lines_amount
                {
                    self.level += 1;
                }
                self.game_info_text.fragments_mut()[5].text = format!("{:02}", self.level);
            }
        }

        ProgramState::Game
    }

    fn was_just_pressed_setfalse_all_players(&mut self) {
        for player in self.vec_players.iter_mut() {
            player.input.was_just_pressed_setfalse();
        }
        self.was_just_pressed_setfalse_common();
    }

    fn was_just_pressed_setfalse_common(&mut self) {
        self.keycode_down_flags.1 = false;
        self.keycode_escape_flags.1 = false;
    }

    pub fn key_down_event(&mut self, keycode: KeyCode, repeat: bool) {
        if !repeat {
            if keycode == KeyCode::Escape {
                self.keycode_escape_flags = (true, true);
                return;
            } else if keycode == KeyCode::Down {
                self.keycode_down_flags = (true, true);
            }
            for player in &mut self.vec_players {
                if player.update_input_keydown(keycode) {
                    return;
                }
            }
        }
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        if keycode == KeyCode::Escape {
            self.keycode_escape_flags = (false, false);
            return;
        } else if keycode == KeyCode::Down {
            self.keycode_down_flags = (false, false);
            return;
        }
        for player in &mut self.vec_players {
            if player.update_input_keyup(keycode) {
                return;
            }
        }
    }

    pub fn gamepad_button_down_event(&mut self, btn: Button, id: GamepadId) {
        for map in self.vec_gamepad_id_map_to_player.iter() {
            if Some(id) == map.0 {
                self.vec_players[map.1 as usize].update_input_buttondown(btn);
                return;
            }
        }
        if self.num_gamepads_to_initialize > 0 {
            for map in self.vec_gamepad_id_map_to_player.iter_mut() {
                if None == map.0 {
                    map.0 = Some(id);
                    self.vec_players[map.1 as usize].update_input_buttondown(btn);
                    if self.vec_gamepad_id_map_to_player.len()
                        == self.vec_gamepad_id_map_to_player.capacity()
                    {
                        self.num_gamepads_to_initialize -= 1;
                    }
                    return;
                }
            }
        }
    }

    pub fn gamepad_button_up_event(&mut self, btn: Button, id: GamepadId) {
        for map in self.vec_gamepad_id_map_to_player.iter() {
            if Some(id) == map.0 {
                self.vec_players[map.1 as usize].update_input_buttonup(btn);
                return;
            }
        }
        if self.num_gamepads_to_initialize > 0 {
            for map in self.vec_gamepad_id_map_to_player.iter_mut() {
                if None == map.0 {
                    map.0 = Some(id);
                    self.vec_players[map.1 as usize].update_input_buttonup(btn);
                    if self.vec_gamepad_id_map_to_player.len()
                        == self.vec_gamepad_id_map_to_player.capacity()
                    {
                        self.num_gamepads_to_initialize -= 1;
                    }
                    return;
                }
            }
        }
    }

    pub fn gamepad_axis_event(&mut self, axis: Axis, value: f32, id: GamepadId) {
        for map in self.vec_gamepad_id_map_to_player.iter() {
            if Some(id) == map.0 {
                self.vec_players[map.1 as usize].update_input_axis(axis, value);
                return;
            }
        }
        if self.num_gamepads_to_initialize > 0
            && !(-DETECT_GAMEPAD_AXIS_THRESHOLD..=DETECT_GAMEPAD_AXIS_THRESHOLD).contains(&value)
        {
            for map in self.vec_gamepad_id_map_to_player.iter_mut() {
                if None == map.0 {
                    map.0 = Some(id);
                    self.vec_players[map.1 as usize].update_input_axis(axis, value);
                    if self.vec_gamepad_id_map_to_player.len()
                        == self.vec_gamepad_id_map_to_player.capacity()
                    {
                        self.num_gamepads_to_initialize -= 1;
                    }
                    return;
                }
            }
        }
    }

    // when drawing, we make an unscaled board with the tiles (so the pixel dimensions are 8 * num_tiles_wide by 8 * num_tiles_tall)
    // with the top left of the board at (0, 0), which is the top left corner of the screen;
    // then when we actually draw the board, we scale it to the appropriate size and place the top left corner of the board at the appropriate place;
    // there's a sprite batch for each players' tiles and one more for the empty tiles, which is constant, and the player tiles are drawn after so they are on top
    pub fn draw(&mut self, ctx: &mut Context) {
        // constants used throughout draw
        let height_buffer = self.bh.get_height_buffer();
        let width = self.bh.get_width();
        let height = self.bh.get_height();

        // start doing drawing stuff
        graphics::clear(ctx, graphics::Color::BLACK);
        let (window_width, window_height) = graphics::size(ctx);
        if self.game_over_flag && self.game_over_delay == 0 {
            // DRAW GAME OVER
            self.draw_text(
                ctx,
                &self.game_over_text,
                0.4,
                &(window_width, window_height),
            );
            self.draw_text(
                ctx,
                &self.game_info_text,
                0.55,
                &(window_width, window_height),
            );
        } else if self.pause_flags.0 {
            // DRAW PAUSE
            self.draw_text(ctx, &self.pause_text, 0.4, &(window_width, window_height));
        } else {
            // DRAW GAME

            // ghost tile highlights
            if self.determine_ghost_tile_locations {
                self.batch_highlight_ghost_tile.clear();
                for piece_positions in self.bh.get_ghost_highlight_positions().iter() {
                    for pos in piece_positions.iter().take(4) {
                        let center = width / 2;
                        let is_center_even: u8 = (center + 1) % 2;
                        let (y_draw_pos, x_draw_pos) = match self.gravity_direction {
                            // account for the gravity direction in how to draw it (rotatris)
                            Movement::Down => (pos.0, pos.1),
                            Movement::Left => (center * 2 - pos.1 - is_center_even, pos.0),
                            Movement::Up => (
                                center * 2 - pos.0 - is_center_even,
                                center * 2 - pos.1 - is_center_even,
                            ),
                            Movement::Right => (pos.1, center * 2 - pos.0 - is_center_even),
                            _ => unreachable!(
                                "[!] Error: self.gravity_direction is {}",
                                self.gravity_direction as u8
                            ),
                        };
                        self.batch_highlight_ghost_tile
                            .add(graphics::DrawParam::new().dest(Point2::from_slice(&[
                                x_draw_pos as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                (y_draw_pos - height_buffer) as f32
                                    * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                            ])));
                    }
                }
            }

            // add each non-empty tile to the correct SpriteBatch
            for x in 0..width {
                for y in 0..height {
                    // actually go through and add tiles to a spritebatch
                    if !self.bh.get_empty_from_pos(y + height_buffer, x) {
                        // account for the gravity direction in how to draw it (rotatris)
                        let center = width / 2;
                        let is_center_even: u8 = (center + 1) % 2;
                        let (y_draw_pos, x_draw_pos) = match self.gravity_direction {
                            Movement::Down => (y, x),
                            Movement::Left => (center * 2 - x - is_center_even, y),
                            Movement::Up => (
                                center * 2 - y - is_center_even,
                                center * 2 - x - is_center_even,
                            ),
                            Movement::Right => (x, center * 2 - y - is_center_even),
                            _ => unreachable!(
                                "[!] Error: self.gravity_direction is {}",
                                self.gravity_direction as u8
                            ),
                        };
                        // create the proper DrawParam and add to the spritebatch
                        let player_tile = graphics::DrawParam::new().dest(Point2::from_slice(&[
                            x_draw_pos as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                            y_draw_pos as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                        ]));
                        if self.num_players > 1 {
                            let player = self.bh.get_player_from_pos(y + height_buffer, x);
                            self.vec_batch_player_piece[player as usize].add(player_tile);
                        } else {
                            let shape: Shapes = self.bh.get_shape_from_pos(y + height_buffer, x);
                            if shape == Shapes::J || shape == Shapes::S {
                                self.vec_batch_player_piece[0].add(player_tile);
                            } else if shape == Shapes::L || shape == Shapes::Z {
                                self.vec_batch_player_piece[1].add(player_tile);
                            } else if shape == Shapes::I || shape == Shapes::O || shape == Shapes::T
                            {
                                self.vec_batch_player_piece[2].add(player_tile);
                            }
                        }
                        // highlight if active
                        if self.bh.get_active_from_pos(y + height_buffer, x) {
                            self.batch_highlight_active_tile.add(player_tile);
                        }
                    }
                }
            }

            // line clear highlights
            if let Some(classic) = &self.bh.classic {
                for full_line in classic.vec_full_lines.iter() {
                    if full_line.lines_cleared_together < 4 {
                        // standard clear animation

                        let y = (full_line.row - height_buffer) as usize;
                        let board_max_index_remainder_2 = (width - 1) % 2;
                        // go from the middle to the outside and reach the end right before full_line.clear_delay reaches 0
                        for x in (width / 2)..width {
                            let highlight_pos_right =
                                graphics::DrawParam::new().dest(Point2::from_slice(&[
                                    x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                ]));
                            let highlight_pos_left =
                                graphics::DrawParam::new().dest(Point2::from_slice(&[
                                    (width as f32 - (x + board_max_index_remainder_2) as f32)
                                        * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                ]));

                            self.batch_highlight_clearing_standard_tile
                                .add(highlight_pos_right);
                            self.batch_highlight_clearing_standard_tile
                                .add(highlight_pos_left);

                            if ((x as f32) / (width as f32) - 0.5) * 2.0
                                > 1.0 - (full_line.clear_delay as f32 / CLEAR_DELAY_CLASSIC as f32)
                            {
                                break;
                            }
                        }
                    } else {
                        // tetrisnt clear animation

                        let y = (full_line.row - height_buffer) as usize;
                        let board_max_index_remainder_2 = (width - 1) % 2;
                        // go from the middle to the outside and reach the end right before full_line.clear_delay reaches 0
                        for x in (width / 2)..width {
                            let highlight_pos_right =
                                graphics::DrawParam::new().dest(Point2::from_slice(&[
                                    x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                ]));
                            let highlight_pos_left =
                                graphics::DrawParam::new().dest(Point2::from_slice(&[
                                    (width as f32 - (x + board_max_index_remainder_2) as f32)
                                        * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                ]));

                            self.batch_highlight_clearing_tetrisnt_tile
                                .add(highlight_pos_right);
                            self.batch_highlight_clearing_tetrisnt_tile
                                .add(highlight_pos_left);

                            if ((x as f32) / (width as f32) - 0.5) * 2.0
                                > 1.0 - (full_line.clear_delay as f32 / CLEAR_DELAY_CLASSIC as f32)
                            {
                                break;
                            }
                        }
                    }
                }
            }

            // next pieces
            let mut color_number_singleplayer = 2;
            let next_piece = self.vec_next_piece[0].shape;
            if next_piece == Shapes::J || next_piece == Shapes::S {
                color_number_singleplayer = 0;
            } else if next_piece == Shapes::L || next_piece == Shapes::Z {
                color_number_singleplayer = 1;
            }
            for player in &mut self.vec_players {
                if player.redraw_next_piece_flag {
                    // if we need to redraw, clear the next piece sprite batch and rebuild it
                    player.redraw_next_piece_flag = false;
                    if self.num_players > 1 {
                        self.vec_batch_next_piece[player.player_num as usize].clear();
                        for x in 0u8..4u8 {
                            for y in 0u8..2u8 {
                                if self.vec_next_piece[player.player_num as usize].matrix
                                    [y as usize][x as usize]
                                {
                                    let next_tile =
                                        graphics::DrawParam::new().dest(Point2::from_slice(&[
                                            x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                            y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                        ]));
                                    self.vec_batch_next_piece[player.player_num as usize]
                                        .add(next_tile);
                                }
                            }
                        }
                    } else {
                        for x in 0..3 {
                            self.vec_batch_next_piece[x].clear();
                        }
                        for x in 0u8..4u8 {
                            for y in 0u8..2u8 {
                                if self.vec_next_piece[player.player_num as usize].matrix
                                    [y as usize][x as usize]
                                {
                                    let next_tile =
                                        graphics::DrawParam::new().dest(Point2::from_slice(&[
                                            x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                            y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                        ]));
                                    self.vec_batch_next_piece[color_number_singleplayer]
                                        .add(next_tile);
                                }
                            }
                        }
                    }
                }
            }

            let scaled_tile_size = self.tile_size / TILE_SIZE_DOWN_SCALE;

            // draw each SpriteBatch
            let board_top_left_corner = window_width / 2.0
                - (scaled_tile_size * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32 * width as f32 / 2.0);
            // empty tiles
            graphics::draw(
                ctx,
                &self.batch_empty_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // ghost pieces
            graphics::draw(
                ctx,
                &self.batch_highlight_ghost_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // player tiles
            for player in 0..std::cmp::max(self.num_players, 3) {
                graphics::draw(
                    ctx,
                    &self.vec_batch_player_piece[player as usize],
                    DrawParam::new()
                        .dest(Point2::from_slice(&[
                            board_top_left_corner,
                            NON_BOARD_SPACE_U as f32 * self.tile_size,
                        ]))
                        .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
                )
                .unwrap();
            }
            // active tile highlights
            graphics::draw(
                ctx,
                &self.batch_highlight_active_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // clearing tile standard highlights
            graphics::draw(
                ctx,
                &self.batch_highlight_clearing_standard_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // clearing tile tetrisnt highlights
            graphics::draw(
                ctx,
                &self.batch_highlight_clearing_tetrisnt_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // next piece tiles
            for player in self.vec_players.iter() {
                if self.num_players > 1 {
                    graphics::draw(
                        ctx,
                        &self.vec_batch_next_piece[player.player_num as usize],
                        DrawParam::new()
                            .dest(Point2::from_slice(&[
                                board_top_left_corner
                                    + (player.spawn_column - 2) as f32
                                        * scaled_tile_size
                                        * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                (NON_BOARD_SPACE_U - BOARD_NEXT_PIECE_SPACING) as f32
                                    * self.tile_size,
                            ]))
                            .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
                    )
                    .unwrap();
                } else {
                    let spawn_column = player.spawn_column;
                    graphics::draw(
                        ctx,
                        &self.vec_batch_next_piece[color_number_singleplayer],
                        DrawParam::new()
                            .dest(Point2::from_slice(&[
                                board_top_left_corner
                                    + (spawn_column - 2) as f32
                                        * scaled_tile_size
                                        * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                (NON_BOARD_SPACE_U - BOARD_NEXT_PIECE_SPACING) as f32
                                    * self.tile_size,
                            ]))
                            .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
                    )
                    .unwrap();
                }
            }
            // score text; TODO: perhaps make a separate function for something based on the bottom,
            // or just figure out how to do this better so we don't divide out by the window_height
            self.draw_text(
                ctx,
                &self.game_info_text,
                1.0 - ((NON_BOARD_SPACE_D as f32 * self.tile_size) / window_height),
                &(window_width, window_height),
            );

            // clear player sprite batches
            for player in 0..std::cmp::max(self.num_players, 3) {
                self.vec_batch_player_piece[player as usize].clear();
            }
            // clear highlight active tile sprite batch
            self.batch_highlight_active_tile.clear();
            // clear highlight clearing tile standard sprite batch
            self.batch_highlight_clearing_standard_tile.clear();
            // clear highlight clearing tile tetrisnt sprite batch
            self.batch_highlight_clearing_tetrisnt_tile.clear();
        }
    }

    fn draw_text(
        &self,
        ctx: &mut Context,
        text_var: &Text,
        vertical_position: f32,
        window_dimensions: &(f32, f32),
    ) {
        let text_var_dimensions = text_var.dimensions(ctx);
        graphics::draw(
            ctx,
            text_var,
            DrawParam::new().dest(Point2::from_slice(&[
                (window_dimensions.0 - text_var_dimensions.w as f32) / 2.0,
                (window_dimensions.1 - text_var_dimensions.h as f32) * vertical_position,
            ])),
        )
        .unwrap();
    }

    pub fn resize_event(&mut self, width: f32, height: f32) {
        self.tile_size = TileGraphic::get_size(
            width,
            height,
            self.bh.get_width(),
            self.bh.get_height() + NON_BOARD_SPACE_U + NON_BOARD_SPACE_D,
        );
    }

    pub fn focus_event(&mut self, gained: bool) {
        if !gained {
            self.pause_flags = (true, true);
        }
    }
}
