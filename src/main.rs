
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
use piece::{Shapes, Movement};

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
    let mut rustrisnt = Rustrisnt::new(ctx, 2u8, 0u8);

    // Run!
    match event::run(ctx, event_loop, &mut rustrisnt) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Rustrisnt {
    // logic (mostly)
    board: Board,
    num_players: u8,
    vec_players: Vec<Player>,
    level: u8,
    num_cleared_lines: u16,
    score: u64,
    // drawing
    tile_size: f32,
    batch_empty_tile: spritebatch::SpriteBatch,
    vec_batch_player_tile: Vec<spritebatch::SpriteBatch>,
}

impl Rustrisnt {
    pub fn new(ctx: &mut Context, num_players: u8, level: u8) -> Rustrisnt {
        // Load/create resources here: images, fonts, sounds, etc.
        let board_width = 6 + 4 * num_players;
        let mut vec_players: Vec<Player> = vec![];
        // implementing this later when a config file with saved ControlScheme's for each player is added with menu UI (rip)
        // for player in 0..(num_players + 1) / 2 {
        //     vec_players.push(Player::new(player, control_scheme[player], (player as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8 + 1));
        // }
        // for player in (num_players + 1) / 2..num_players {
        //     vec_players.push(Player::new(player, control_scheme[player], board_width - 1 - ((num_players - 1 - player) as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8));
        // }
        // ...and will get rid of the following two...
        vec_players.push(Player::new(0, ControlScheme::new(KeyCode::A, KeyCode::D, KeyCode::S, KeyCode::E, KeyCode::Q), (0 as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8 + 1));
        vec_players.push(Player::new(1, ControlScheme::new(KeyCode::J, KeyCode::L, KeyCode::K, KeyCode::U, KeyCode::O), board_width - 1 - ((num_players - 1 - 1) as f32 * (board_width as f32 / num_players as f32) + board_width as f32 / (2.0 * num_players as f32)) as u8));
        let batch_empty_tile = spritebatch::SpriteBatch::new(TileGraphic::new_empty(ctx).image);
        let mut vec_batch_player_tile: Vec<spritebatch::SpriteBatch> = vec![];
        for player in 0..num_players {
            vec_batch_player_tile.push(spritebatch::SpriteBatch::new(TileGraphic::new_player(ctx, player).image));
        }

        println!("[+] starting game with {} players and at level {}", num_players, level);
        Self {
            board: Board::new(board_width, 20u8, num_players),
            num_players,
            vec_players,
            level: level,
            num_cleared_lines: 0u16,
            score: 0u64,
            tile_size: 0f32,
            batch_empty_tile,
            vec_batch_player_tile,
        }
    }
}

// draw and update are done every frame
impl EventHandler for Rustrisnt {
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
                    self.board.vec_active_piece[player.player_num as usize] = piece::Piece::new(Shapes::I); // TODO: make random sometime
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
                let caused_full_line_flag: bool = self.board.attempt_piece_movement(Movement::Down, player.player_num);
                // if the piece got locked, piece.shape gets set to Shapes::None, so set the spawn piece flag
                if self.board.vec_active_piece[player.player_num as usize].shape == Shapes::None {
                    player.spawn_piece_flag = true;
                    // add more spawn delay if locking the piece caused a line clear
                    if caused_full_line_flag {
                        player.spawn_delay += CLEAR_DELAY as i16;
                    }
                }
            }

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
                match player.control_scheme.find_move(keycode) {
                    Movement::Left => player.input.keydown_left = (true, true),
                    Movement::Right => player.input.keydown_right = (true, true),
                    Movement::Down => player.input.keydown_down = (true, true),
                    Movement::RotateCw => player.input.keydown_rotate_cw = (true, true),
                    Movement::RotateCcw => player.input.keydown_rotate_ccw = (true, true),
                    _ => {},
                }
                // debug inputs
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
        self.tile_size = TileGraphic::get_size(ctx, self.board.width, self.board.height);

        for x in 0..self.board.width {
            for y in 0..self.board.height {
                // empty tiles
                if self.board.matrix[(y + BOARD_HEIGHT_BUFFER_U) as usize][x as usize].empty {
                    let x = x as f32;
                    let y = (y + BOARD_HEIGHT_BUFFER_U) as f32;
                    let empty_tile = graphics::DrawParam::new().dest(Point2::new(x * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, (y - BOARD_HEIGHT_BUFFER_U as f32) * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                    self.batch_empty_tile.add(empty_tile);
                } else {
                    // player tiles
                    for player in 0..self.num_players {
                        if self.board.matrix[(y + BOARD_HEIGHT_BUFFER_U) as usize][x as usize].player == player {
                            let x = x as f32;
                            let y = (y + BOARD_HEIGHT_BUFFER_U) as f32;
                            let player_tile = graphics::DrawParam::new().dest(Point2::new(x * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, (y - BOARD_HEIGHT_BUFFER_U as f32) * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                            self.vec_batch_player_tile[player as usize].add(player_tile);
                        }
                    }
                }
            }
        }

        // empty tiles
        graphics::draw(ctx, &self.batch_empty_tile, DrawParam::new().dest(Point2::new(window_width / 2.0 - (self.tile_size * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32 * self.board.width as f32 / (2.0 * 8.5)), 0.0)).scale(Vector2::new(self.tile_size / 8.5, self.tile_size / 8.5)))?;
        // player tiles
        for player in 0..self.num_players {
            graphics::draw(ctx, &self.vec_batch_player_tile[player as usize], DrawParam::new().dest(Point2::new(window_width / 2.0 - (self.tile_size * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32 * self.board.width as f32 / (2.0 * 8.5)), 0.0)).scale(Vector2::new(self.tile_size / 8.5, self.tile_size / 8.5)))?;           
        }

        // clear sprite batches; this is a bit inefficient and should maybe be changed to using sprite indices; TODO
        self.batch_empty_tile.clear();
        for player in 0..self.num_players {
            self.vec_batch_player_tile[player as usize].clear();
        }

        graphics::present(ctx)
    }

    // this seems unused but is called somewhere in ggez to ultimately make things scale and get placed correctly
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}