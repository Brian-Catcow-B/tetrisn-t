
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods};
use ggez::graphics::{self, DrawParam, spritebatch};
use ggez::nalgebra as na;
use na::Point2;
use na::Vector2;

// file systems stuff
use std::path;
use std::env;

mod player;
use player::{Player, SPAWN_DELAY};

mod tile;
use tile::NUM_PIXEL_ROWS_PER_TILEGRAPHIC;
use tile::TileGraphic;

mod piece;
use piece::{Shapes, Movement, NextPiece};

mod board;
use board::BOARD_HEIGHT_BUFFER_U;
use board::Board;

mod controls;
use controls::ControlScheme;

pub const CLEAR_DELAY: i8 = 60i8;

pub const SCORE_SINGLE_BASE: u8 = 40u8;
pub const SCORE_DOUBLE_BASE: u8 = 100u8;
pub const SCORE_TRIPLE_BASE: u16 = 300u16;
pub const SCORE_QUADRUPLE_BASE: u16 = 1200u16;

const BOARD_HEIGHT: u8 = 20u8;

// space up of the board that is not the board in tiles
pub const NON_BOARD_SPACE_U: u8 = 4u8;

fn main() {
    let mut context = ContextBuilder::new("Rustrisn-t", "Catcow");

    // file systems stuff
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        context = context.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut context.build().expect("Failed to build context");

    // set window size
    graphics::set_resizable(ctx, true).expect("Failed to set window to resizable");
    graphics::set_drawable_size(ctx, 800.0, 600.0).expect("Failed to resize window");

    // make it not blurry
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut game = Game::new(ctx, 2u8, 0u8);

    // Run!
    match event::run(ctx, event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Game {
    // logic (mostly)
    board: Board,
    num_players: u8,
    vec_players: Vec<Player>,
    vec_next_piece: Vec<NextPiece>,
    level: u8,
    num_cleared_lines: u16,
    score: u64,
    // drawing
    tile_size: f32,
    batch_empty_tile: spritebatch::SpriteBatch,
    vec_batch_player_piece: Vec<spritebatch::SpriteBatch>,
    vec_batch_next_piece: Vec<spritebatch::SpriteBatch>,
    sprite_index_matrix: Vec<Vec<(spritebatch::SpriteIdx, u8)>>, // (sprite_index, player_num) with player_num = 0xff for empty tiles
    rebuild_sprite_index_matrix_flag: bool,
    rebuild_sprite_index_matrix_counter: i16,
}

impl Game {
    pub fn new(ctx: &mut Context, num_players: u8, starting_level: u8) -> Game {
        // Load/create resources here: images, fonts, sounds, etc.
        let board_width = 6 + 4 * num_players;
        let mut vec_players: Vec<Player> = Vec::with_capacity(num_players as usize);
        // implementing this later when a config file with saved ControlScheme's for each player is added with menu UI (rip)
        // for player in 0..(num_players + 1) / 2 {
        //     vec_players.push(Player::new(player, control_scheme[player], (player as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8 + 1));
        // }
        // for player in (num_players + 1) / 2..num_players {
        //     vec_players.push(Player::new(player, control_scheme[player], board_width - 1 - ((num_players - 1 - player) as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8));
        // }
        // ...and will get rid of the following two...
        vec_players.push(Player::new(0, ControlScheme::new(KeyCode::A, KeyCode::D, KeyCode::S, KeyCode::E, KeyCode::Q), (0 as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8 + 1));
        vec_players.push(Player::new(1, ControlScheme::new(KeyCode::J, KeyCode::L, KeyCode::K, KeyCode::O, KeyCode::U), board_width - 1 - ((num_players - 1 - 1) as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8));
        let mut batch_empty_tile = spritebatch::SpriteBatch::new(TileGraphic::new_empty(ctx).image);
        let mut vec_next_piece: Vec<NextPiece> = Vec::with_capacity(num_players as usize);
        let mut vec_batch_player_piece: Vec<spritebatch::SpriteBatch> = Vec::with_capacity(num_players as usize);
        let mut vec_batch_next_piece: Vec<spritebatch::SpriteBatch> = Vec::with_capacity(num_players as usize);
        for player in 0..num_players {
            vec_next_piece.push(NextPiece::new(Shapes::None));
            vec_batch_player_piece.push(spritebatch::SpriteBatch::new(TileGraphic::new_player(ctx, player).image));
            vec_batch_next_piece.push(spritebatch::SpriteBatch::new(TileGraphic::new_player(ctx, player).image));
        }

        // create empty tile sprite index matrix
        let mut sprite_index_matrix: Vec<Vec<(spritebatch::SpriteIdx, u8)>> = Vec::with_capacity((BOARD_HEIGHT) as usize);
        for x in 0..board_width {
            for y in 0..BOARD_HEIGHT {
                sprite_index_matrix.push(Vec::with_capacity(board_width as usize));
                let empty_tile = graphics::DrawParam::new().dest(Point2::new(x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                // if y < BOARD_HEIGHT_BUFFER_U {
                //     empty_tile.scale(Vector2::new(0.0, 0.0));
                // }
                sprite_index_matrix[y as usize].push((batch_empty_tile.add(empty_tile), 0xff));
            }
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
            tile_size: 0f32,
            batch_empty_tile,
            vec_batch_player_piece,
            vec_batch_next_piece,
            sprite_index_matrix,
            rebuild_sprite_index_matrix_flag: true,
            rebuild_sprite_index_matrix_counter: 0i16,
        }
    }

    // when a piece is moved successfully, we swap sprite indices of the start and end tiles of the piece movement so we don't have to fully rebuild the sprites each draw() call
    fn swap_sprite_indices_on_move(&mut self, start_positions: &[(u8, u8); 4], end_positions: &[(u8, u8); 4]) {
        let mut start_spritebatch_copy: Vec<Option<(spritebatch::SpriteIdx, u8)>> = Vec::with_capacity(4);
        let mut end_spritebatch_copy: Vec<Option<(spritebatch::SpriteIdx, u8)>> = Vec::with_capacity(4);
        for index in 0..4 {
            if start_positions[index].0 < BOARD_HEIGHT_BUFFER_U {
                start_spritebatch_copy.push(None);
            } else {
                start_spritebatch_copy.push(Some(self.sprite_index_matrix[(start_positions[index].0 - BOARD_HEIGHT_BUFFER_U) as usize][start_positions[index].1 as usize]));
            }
            if end_positions[index].0 < BOARD_HEIGHT_BUFFER_U {
                end_spritebatch_copy.push(None);
            } else {
                end_spritebatch_copy.push(Some(self.sprite_index_matrix[(start_positions[index].0 - BOARD_HEIGHT_BUFFER_U) as usize][start_positions[index].1 as usize]));
            }
        }

        // it'd be really nice if these worked instead :(
        // let start_spritebatch_copy: [Option<(spritebatch::SpriteIdx, u8)>; 4] = start_positions.iter().take(4).map(|(y, x)| {
        //     if y < &BOARD_HEIGHT_BUFFER_U {
        //         return None;
        //     } else {
        //         return Some(self.sprite_index_matrix[*y as usize - &BOARD_HEIGHT_BUFFER][*x as usize]);
        //     }
        // }).collect();
        // let end_spritebatch_copy: [Option<(spritebatch::SpriteIdx, u8)>; 4] = start_positions.iter().take(4).map(|(y, x)| {
        //     if y < &BOARD_HEIGHT_BUFFER_U {
        //         return None;
        //     } else {
        //         return Some(self.sprite_index_matrix[*y as usize - &BOARD_HEIGHT_BUFFER][*x as usize]);
        //     }
        // }).collect();

        // this could also be a closure probs; basically just making an array of (start_bool, end_bool) singifying if any position in the corresponding opposite start_positions or end_positions is the same
        let mut start_end_same: [(bool, bool); 4] = [(false, false); 4];
        for num_checks in (1..4 + 1).rev() {
            for check in 0..num_checks {
                if start_positions[4 - num_checks] == end_positions[check] {
                    start_end_same[4 - num_checks].0 = true;
                    start_end_same[check].1 = true;
                }
            }
        }
        for index in 0..4 {
            // this testing stuff can be more efficient probably; each start and end position we check to see if
            if start_spritebatch_copy[index] != None && start_end_same[index].0 {
                self.sprite_index_matrix[(start_positions[index].0 - BOARD_HEIGHT_BUFFER_U) as usize][start_positions[index].1 as usize] = start_spritebatch_copy[index].unwrap();
            }
            if start_spritebatch_copy[index] != None && start_end_same[index].1 {
                self.sprite_index_matrix[(start_positions[index].0 - BOARD_HEIGHT_BUFFER_U) as usize][start_positions[index].1 as usize] = end_spritebatch_copy[index].unwrap();
            }
        }
    }
}

// draw and update are done every frame
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Debug stuff...

        // draws all the active players' tiles on the top row
        // for player in 0..self.num_players {
        //     self.board.matrix[player as usize][BOARD_HEIGHT_BUFFER_U] = Tile::new(false, true, player.player_num);
        // }

        // Update code...
        for player in self.vec_players.iter_mut() {
            if !player.spawn_piece_flag && self.board.vec_active_piece[player.player_num as usize].shape == Shapes::None {
                player.input.was_unpressed_previous_frame_setfalse();
                continue;
            }

            // piece spawning
            if player.spawn_piece_flag {
                if player.spawn_delay <= 0 {
                    // TODO: check if spawning collides with anything (if it overlaps board, self.game_over_flag = true;, if it only collides with active tiles, wait)
                    player.spawn_piece_flag = false;
                    if self.vec_next_piece[player.player_num as usize].shape == Shapes::None {
                        player.next_piece = piece::Piece::new(Shapes::L); // TODO: make random sometime
                    }
                    self.board.vec_active_piece[player.player_num as usize] = piece::Piece::new(player.next_piece.shape);
                    player.next_piece = piece::Piece::new(Shapes::J); // TODO: make random sometime
                    self.vec_next_piece[player.player_num as usize] = NextPiece::new(player.next_piece.shape); // TODO: is there a way to make this not need Copy?
                    self.board.vec_active_piece[player.player_num as usize].spawn(player.spawn_column);
                    self.board.playerify_piece(player.player_num);
                    player.spawn_delay = SPAWN_DELAY;
                } else {
                    player.spawn_delay -=1;
                }
                continue;
            }

            // piece movement
            // CW / CCW
            if player.input.keydown_rotate_cw.1 {
                // if it did move the piece, do spritebatch stuff so the sprite batches don't have to be recalculated
                if self.board.attempt_piece_movement(Movement::RotateCw, player.player_num).0 {

                }
            }
            if player.input.keydown_rotate_ccw.1 {
                // if it did move the piece, do spritebatch stuff so the sprite batches don't have to be recalculated
                if self.board.attempt_piece_movement(Movement::RotateCcw, player.player_num).0 {

                }
            }
            // LEFT / RIGHT
            if player.input.keydown_left.1 {
                // if it did move the piece, do spritebatch stuff so the sprite batches don't have to be recalculated
                if self.board.attempt_piece_movement(Movement::Left, player.player_num).0 {

                }
            }
            if player.input.keydown_right.1 {
                // if it did move the piece, do spritebatch stuff so the sprite batches don't have to be recalculated
                if self.board.attempt_piece_movement(Movement::Right, player.player_num).0 {

                }
            }
            // DOWN
            // down is interesting because every time the downwards position is false we have to check if it's running into the bottom or an inactive tile so we know if we should lock it
            if player.input.keydown_down.1 {
                let (moved_flag, caused_full_line_flag): (bool, bool) = self.board.attempt_piece_movement(Movement::Down, player.player_num);
                // if the piece got locked, piece.shape gets set to Shapes::None, so set the spawn piece flag
                if self.board.vec_active_piece[player.player_num as usize].shape == Shapes::None {
                    println!("locking piece");
                    player.spawn_piece_flag = true;
                    // add more spawn delay if locking the piece caused a line clear
                    if caused_full_line_flag {
                        player.spawn_delay += CLEAR_DELAY as i16;
                    }
                }
            }

            // debug player 0 inputs
            // if player.player_num == 0 {
            //     println!("inputs player {} before", player.player_num);
            //     player.input._print_inputs();
            // }

            // update controls (always do after all player player input for each player)
            player.input.was_unpressed_previous_frame_setfalse();
        }

        // attempt to line clear (go through the vector of FullLine's and decrement clear_delay if > 0, clear and return (lines_cleared, score) for <= 0)
        let (returned_lines, returned_score) = self.board.attempt_clear_lines(self.level);
        if returned_lines > 0 {
            self.num_cleared_lines += returned_lines as u16;
            self.score += returned_score as u64;
            println!("[+] lines: {}; score: {}", self.num_cleared_lines, self.score);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            for player in self.vec_players.iter_mut() {
                // POTENTIAL OPTIMIZATION: have a check elsewhere that makes sure no two input overlap and then just return after it finds what an input goes to; also in key_up_event()
                // the extra `if` statements in each case are to fix a weird bug that happens because when multiple keys are pressed and then one is unpressed, it triggers a keydown event for all the others
                match player.control_scheme.find_move(keycode) {
                    Movement::Left => {
                        if !player.input.keydown_left.0 {
                            player.input.keydown_left = (true, true);
                        }
                    },
                    Movement::Right => {
                        if !player.input.keydown_right.0 {
                            player.input.keydown_right = (true, true);
                        }
                    },
                    Movement::Down => {
                        if !player.input.keydown_down.0 {
                            player.input.keydown_down = (true, true);
                        }
                    },
                    Movement::RotateCw => {
                        if !player.input.keydown_rotate_cw.0 {
                            player.input.keydown_rotate_cw = (true, true);
                        }
                    },
                    Movement::RotateCcw => {
                        if !player.input.keydown_rotate_ccw.0 {
                            player.input.keydown_rotate_ccw = (true, true);
                        }
                    },
                    _ => {},
                }
                // debug inputs
                // println!("keydown");
                // println!("inputs:");
                // player.input._print_inputs();
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        for player in self.vec_players.iter_mut() {
            match player.control_scheme.find_move(keycode) {
                Movement::Left => player.input.keydown_left = (false, false),
                Movement::Right => player.input.keydown_right = (false, false),
                Movement::Down => player.input.keydown_down = (false, false),
                Movement::RotateCw => player.input.keydown_rotate_cw = (false, false),
                Movement::RotateCcw => player.input.keydown_rotate_ccw = (false, false),
                _ => return,
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (window_width, _window_height) = graphics::size(ctx);
        graphics::clear(ctx, graphics::BLACK);
        self.tile_size = TileGraphic::get_size(ctx, self.board.width, self.board.height + NON_BOARD_SPACE_U);

        // self.rebuild_sprite_index_matrix_flag is set to true every time a line is completed, since it's easier (and most of the time more efficient) to rebuild everything when lines are cleared
        if self.rebuild_sprite_index_matrix_flag {
            self.rebuild_sprite_index_matrix_counter -= 1;
            if self.rebuild_sprite_index_matrix_counter <= 0 {
                self.rebuild_sprite_index_matrix_flag = false;
                self.batch_empty_tile.clear();
                for batch in self.vec_batch_player_piece.iter_mut().take(self.num_players as usize) {
                    batch.clear();
                }
                // create correct SpriteBatch for each
                for x in 0..self.board.width {
                    for y in 0..self.board.height {
                        // empty tiles
                        if self.board.matrix[(y + BOARD_HEIGHT_BUFFER_U) as usize][x as usize].empty {
                            let empty_tile = graphics::DrawParam::new().dest(Point2::new(x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                            self.sprite_index_matrix[y as usize][x as usize].0 = self.batch_empty_tile.add(empty_tile);
                        } else {
                            // player tiles
                            let player = self.board.matrix[(y + BOARD_HEIGHT_BUFFER_U) as usize][x as usize].player;
                            let player_tile = graphics::DrawParam::new().dest(Point2::new(x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                            self.vec_batch_player_piece[player as usize].add(player_tile);
                        }
                    }
                }
            }
        }

        // next pieces
        for player in 0..self.num_players {
            for x in 0..4 {
                for y in 0..2 {
                    if self.vec_next_piece[player as usize].matrix[y][x] {
                        let x = x as f32;
                        let y = y as f32;
                        let next_tile = graphics::DrawParam::new().dest(Point2::new(x * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, y * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                        self.vec_batch_next_piece[player as usize].add(next_tile);
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
            .scale(Vector2::new(scaled_tile_size, scaled_tile_size)))?;
        // player tiles
        for player in 0..self.num_players {
            graphics::draw(ctx, &self.vec_batch_player_piece[player as usize], DrawParam::new()
                .dest(Point2::new(board_top_left_corner, NON_BOARD_SPACE_U as f32 * self.tile_size))
                .scale(Vector2::new(scaled_tile_size, scaled_tile_size)))?;
        }
        // next piece tiles
        for player in self.vec_players.iter() {
            graphics::draw(ctx, &self.vec_batch_next_piece[player.player_num as usize], DrawParam::new()
                .dest(Point2::new(board_top_left_corner + (player.spawn_column - 2) as f32 * scaled_tile_size * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, 1f32 * scaled_tile_size))
                .scale(Vector2::new(scaled_tile_size, scaled_tile_size)))?;
        }

        // clear sprite batches
        // self.batch_empty_tile.clear();
        // for player in 0..self.num_players {
        //     self.vec_batch_player_piece[player as usize].clear();
        // }

        graphics::present(ctx)
    }

    // this seems unused but is called somewhere in ggez to ultimately make things scale and get placed correctly when changing window size
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}