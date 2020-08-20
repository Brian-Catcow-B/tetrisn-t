// use ggez::{Context, GameResult};
use ggez::Context;
// use ggez::event::EventHandler;
use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods};
use ggez::graphics::{self, DrawParam, spritebatch};
use ggez::nalgebra::{Point2, Vector2};
// use na::Point2;
// use na::Vector2;
use ggez::graphics::{Align, Color, Scale, Text, TextFragment};

use crate::control::ProgramState;

mod player;
use crate::game::player::{Player, SPAWN_DELAY};

mod tile;
use crate::game::tile::NUM_PIXEL_ROWS_PER_TILEGRAPHIC;
use crate::game::tile::TileGraphic;

mod piece;
use crate::game::piece::{Shapes, Movement, NextPiece};

mod board;
use crate::game::board::BOARD_HEIGHT_BUFFER_U;
use crate::game::board::Board;

use crate::inputs::ControlScheme;


const BOARD_HEIGHT: u8 = 20u8;

pub const CLEAR_DELAY: i8 = 60i8;

pub const SCORE_SINGLE_BASE: u8 = 40u8;
pub const SCORE_DOUBLE_BASE: u8 = 100u8;
pub const SCORE_TRIPLE_BASE: u16 = 300u16;
pub const SCORE_QUADRUPLE_BASE: u16 = 1200u16;

const GAME_OVER_DELAY: i8 = 60i8;

// space up of the board that is not the board in tiles
pub const NON_BOARD_SPACE_U: u8 = 4u8;

// this struct is for when the Menu struct returns what game options to start with
pub struct GameOptions {
    pub num_players: u8,
    pub starting_level: u8,
    pub vec_keyboard_inputs: Vec<ControlScheme>,
}

impl GameOptions {
    pub fn new(num_players: u8, starting_level: u8, vec_keyboard_inputs: Vec<ControlScheme>) -> Self {
        Self {
            num_players,
            starting_level,
            vec_keyboard_inputs,
        }
    }
}

pub struct Game {
    // GAME STUFF
    // logic (mostly)
    board: Board,
    num_players: u8,
    vec_players: Vec<Player>,
    vec_next_piece: Vec<NextPiece>,
    level: u8,
    num_cleared_lines: u16,
    score: u64,
    pause_flag: bool,
    game_over_flag: bool,
    game_over_delay: i8,
    // drawing
    tile_size: f32,
    batch_empty_tile: spritebatch::SpriteBatch,
    vec_batch_player_piece: Vec<spritebatch::SpriteBatch>,
    vec_batch_next_piece: Vec<spritebatch::SpriteBatch>,
}

impl Game {
    pub fn new(ctx: &mut Context, num_players: u8, starting_level: u8, vec_keyboard_inputs: Vec<ControlScheme>) -> Game {
        // Load/create resources here: images, fonts, sounds, etc.
        // let board_width = 6 + 4 * num_players;
        let board_width = 6;
        let mut vec_players: Vec<Player> = Vec::with_capacity(num_players as usize);
        // spawn columns
        // first half, not including middle player if there's an odd number of players
        for player in 0..(num_players) / 2 {
            vec_players.push(Player::new(player, vec_keyboard_inputs[player as usize], (player as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8 + 1));
        }
        // middle player, for an odd number of players
        if num_players % 2 == 1 {
            let player = num_players / 2;
            vec_players.push(Player::new(player, vec_keyboard_inputs[player as usize], board_width / 2));
        }
        // second half, not including the middle player if there's an odd number of players
        for player in (num_players + 1) / 2..num_players {
            vec_players.push(Player::new(player, vec_keyboard_inputs[player as usize], board_width - 1 - ((num_players - 1 - player) as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8));
        }
        let mut batch_empty_tile = spritebatch::SpriteBatch::new(TileGraphic::new_empty(ctx).image);
        // the emtpy tile batch will be constant once the game starts with the player tile batches drawing on top of it, so just set that up here
        for x in 0..board_width {
            for y in 0..BOARD_HEIGHT as usize {
                // empty tiles
                let empty_tile = graphics::DrawParam::new().dest(Point2::new(x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                batch_empty_tile.add(empty_tile);
            }
        }
        let mut vec_next_piece: Vec<NextPiece> = Vec::with_capacity(num_players as usize);
        let mut vec_batch_player_piece: Vec<spritebatch::SpriteBatch> = Vec::with_capacity(num_players as usize);
        let mut vec_batch_next_piece: Vec<spritebatch::SpriteBatch> = Vec::with_capacity(num_players as usize);
        for player in 0..num_players {
            vec_next_piece.push(NextPiece::new(Shapes::None));
            vec_batch_player_piece.push(spritebatch::SpriteBatch::new(TileGraphic::new_player(ctx, player).image));
            vec_batch_next_piece.push(spritebatch::SpriteBatch::new(TileGraphic::new_player(ctx, player).image));
        }

        println!("[+] starting game with {} players and at level {}", num_players, starting_level);
        Self {
            board: Board::new(board_width, BOARD_HEIGHT, num_players),
            num_players,
            vec_players,
            vec_next_piece,
            level: starting_level,
            num_cleared_lines: 0u16,
            score: 0u64,
            pause_flag: false,
            game_over_flag: false,
            game_over_delay: GAME_OVER_DELAY,
            tile_size: 0f32,
            batch_empty_tile,
            vec_batch_player_piece,
            vec_batch_next_piece,
        }
    }

    pub fn update(&mut self) -> ProgramState {
        if self.pause_flag {
            for player in self.vec_players.iter_mut() {
                // should we quit to main menu?
                if     player.input.keydown_down.0
                    && player.input.keydown_rotate_ccw.0
                    && player.input.keydown_rotate_cw.0
                    && player.input.keydown_start.1
                {
                    return ProgramState::Menu;
                }
                // should we resume?
                if player.input.keydown_start.1 {
                    self.pause_flag = false;
                    player.input.was_just_pressed_setfalse();
                }
            }
        } else {
            // Debug stuff...

            // draws all the active players' tiles on the top row
            // for player in 0..self.num_players {
            //     self.board.matrix[player as usize][BOARD_HEIGHT_BUFFER_U] = Tile::new(false, true, player.player_num);
            // }

            // Update code...
            for player in self.vec_players.iter_mut() {
                if !player.spawn_piece_flag && self.board.vec_active_piece[player.player_num as usize].shape == Shapes::None {
                    player.input.was_just_pressed_setfalse();
                    continue;
                }

                // piece spawning
                if player.spawn_piece_flag {
                    if player.spawn_delay <= 0 {
                        // (blocked, blocked by some !active tile); if .1, game over sequence, if .0 and !.1, only blocked by other players, wait until they move, then carry on
                        let blocked: (bool, bool) = self.board.attempt_piece_spawn(player.player_num, player.spawn_column, player.next_piece_shape);
                        if blocked.0 {
                            if blocked.1 {
                                self.game_over_flag = true;
                            }
                            continue;
                        }
                        self.board.playerify_piece(player.player_num);
                        player.spawn_delay = SPAWN_DELAY;
                        player.spawn_piece_flag = false;
                        player.next_piece_shape = Shapes::I; // TODO: make random sometime
                        self.vec_next_piece[player.player_num as usize] = NextPiece::new(player.next_piece_shape);
                        player.redraw_next_piece_flag = true;
                    } else {
                        player.spawn_delay -= 1;
                    }
                    continue;
                }

                // piece movement
                // CW / CCW
                if player.input.keydown_rotate_cw.1 {
                    self.board.attempt_piece_movement(Movement::RotateCw, player.player_num);
                }
                if player.input.keydown_rotate_ccw.1 {
                    self.board.attempt_piece_movement(Movement::RotateCcw, player.player_num);
                }
                // LEFT / RIGHT
                if player.input.keydown_left.1 {
                    self.board.attempt_piece_movement(Movement::Left, player.player_num);
                }
                if player.input.keydown_right.1 {
                    self.board.attempt_piece_movement(Movement::Right, player.player_num);
                }
                // DOWN
                // down is interesting because every time the downwards position is false we have to check if it's running into the bottom or an inactive tile so we know if we should lock it
                if player.input.keydown_down.1 {
                    let caused_full_line_flag: bool = self.board.attempt_piece_movement(Movement::Down, player.player_num).1;
                    // if the piece got locked, piece.shape gets set to Shapes::None, so set the spawn piece flag
                    if self.board.vec_active_piece[player.player_num as usize].shape == Shapes::None {
                        player.spawn_piece_flag = true;
                        // add more spawn delay if locking the piece caused a line clear
                        if caused_full_line_flag {
                            player.spawn_delay += CLEAR_DELAY as i16;
                        }
                    }
                }

                if player.input.keydown_start.1 {
                    self.pause_flag = true;
                    player.input.was_just_pressed_setfalse();
                }

                // debug player 0 inputs
                // if player.player_num == 0 {
                //     println!("inputs player {} before", player.player_num);
                //     player.input._print_inputs();
                // }

                // update controls (always do after all player player input for each player)
                player.input.was_just_pressed_setfalse();
            }

            // attempt to line clear (go through the vector of FullLine's and decrement clear_delay if > 0, clear and return (lines_cleared, score) for <= 0)
            let (returned_lines, returned_score) = self.board.attempt_clear_lines(self.level);
            if returned_lines > 0 {
                self.num_cleared_lines += returned_lines as u16;
                self.score += returned_score as u64;
                println!("[+] lines: {}; score: {}", self.num_cleared_lines, self.score);
            }
        }

        ProgramState::Game
    }

    pub fn key_down_event(
        &mut self,
        // _ctx: &mut Context,
        keycode: KeyCode,
        // _keymod: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            for player in self.vec_players.iter_mut() {
                if player.update_input_keydown(keycode) {break;}
            }
        }
    }

    pub fn key_up_event(
        &mut self,
        // _ctx: &mut Context,
        keycode: KeyCode,
        // _keymod: KeyMods
    ) {
        // println!("Key released: {:?}, modifier {:?}", keycode, _keymod);
        for player in self.vec_players.iter_mut() {
            player.update_input_keyup(keycode);
        }
    }

    // when drawing, we make an unscaled board with the tiles (so the pixel dimensions are 8 * num_tiles_wide by 8 * num_tiles_tall)
    // with the top left of the board at (0, 0), which is the top left corner of the screen;
    // then when we actually draw the board, we scale it to the appropriate size and place the top left corner of the board at the appropriate place;
    // there's a sprite batch for each players' tiles and one more for the empty tiles, which is constant, and the player tiles are drawn after so they are on top
    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, graphics::BLACK);
        let (window_width, window_height) = graphics::size(ctx);
        self.tile_size = TileGraphic::get_size(ctx, self.board.width, self.board.height + NON_BOARD_SPACE_U);
        if !self.pause_flag {
            // add each non-empty tile to the correct SpriteBatch
            for x in 0..self.board.width {
                for y in 0..self.board.height {
                    if !self.board.matrix[(y + BOARD_HEIGHT_BUFFER_U) as usize][x as usize].empty {
                        let player = self.board.matrix[(y + BOARD_HEIGHT_BUFFER_U) as usize][x as usize].player;
                        let player_tile = graphics::DrawParam::new().dest(Point2::new(x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                        self.vec_batch_player_piece[player as usize].add(player_tile);
                    }
                }
            }
            // next pieces
            for player in self.vec_players.iter_mut() {
                if player.redraw_next_piece_flag {
                    player.redraw_next_piece_flag = false;
                    // if we need to redraw, clear the next piece sprite batch and rebuild it
                    self.vec_batch_next_piece[player.player_num as usize].clear();
                    for x in 0..4 {
                        for y in 0..2 {
                            if self.vec_next_piece[player.player_num as usize].matrix[y][x] {
                                let next_tile = graphics::DrawParam::new().dest(Point2::new(x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                                self.vec_batch_next_piece[player.player_num as usize].add(next_tile);
                            }
                        }
                    }
                }
            }

            const TILE_SIZE_DOWN_SCALE: f32 = 8.5; // each tile is actually 8x8 pixels, so we scale down by 8 and then some because with 8.0, window resizing can clip off the bottom of the board
            let scaled_tile_size = self.tile_size / TILE_SIZE_DOWN_SCALE;

            // draw each SpriteBatch
            let board_top_left_corner = window_width / 2.0 - (scaled_tile_size * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32 * self.board.width as f32 / 2.0);
            // empty tiles
            graphics::draw(ctx, &self.batch_empty_tile, DrawParam::new()
                .dest(Point2::new(board_top_left_corner, NON_BOARD_SPACE_U as f32 * self.tile_size))
                .scale(Vector2::new(scaled_tile_size, scaled_tile_size))).unwrap();
            // player tiles
            for player in 0..self.num_players {
                graphics::draw(ctx, &self.vec_batch_player_piece[player as usize], DrawParam::new()
                    .dest(Point2::new(board_top_left_corner, NON_BOARD_SPACE_U as f32 * self.tile_size))
                    .scale(Vector2::new(scaled_tile_size, scaled_tile_size))).unwrap();
            }
            // next piece tiles
            for player in self.vec_players.iter() {
                graphics::draw(ctx, &self.vec_batch_next_piece[player.player_num as usize], DrawParam::new()
                    .dest(Point2::new(board_top_left_corner + (player.spawn_column - 2) as f32 * scaled_tile_size * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, 1f32 * scaled_tile_size))
                    .scale(Vector2::new(scaled_tile_size, scaled_tile_size))).unwrap();
            }

            // clear player sprite batches
            for player in 0..self.num_players {
                self.vec_batch_player_piece[player as usize].clear();
            }
        } else {
            // display a pause screen
            let mut text = Text::new(TextFragment {
                // `TextFragment` stores a string, and optional parameters which will override those
                // of `Text` itself. This allows inlining differently formatted lines, words,
                // or even individual letters, into the same block of text.
                text: "PAUSED\n\nDown + RotateCw + RotateCcw then Start to quit".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                // `Font` is a handle to a loaded TTF, stored inside the `Context`.
                // `Font::default()` always exists and maps to DejaVuSerif.
                font: Some(graphics::Font::default()),
                scale: Some(Scale::uniform(self.tile_size)),
                ..Default::default()
            });
            let (paused_text_width, paused_text_height) = text.dimensions(ctx);
            text.set_bounds(Point2::new(paused_text_width as f32, paused_text_height as f32), Align::Center);

            graphics::draw(ctx, &text, DrawParam::new()
            .offset(Point2::new(0.5, 0.5))
            .dest(Point2::new((window_width - paused_text_width as f32) / 2.0, (window_height - paused_text_height as f32) / 2.0))
            ).unwrap();
        }
    }
}