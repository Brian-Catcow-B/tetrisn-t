
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
use player::Player;

mod tile;
use tile::NUM_PIXEL_ROWS_PER_TILEGRAPHIC;
use tile::{Tile, TileGraphic};

mod piece;
use piece::{Shapes, Movement};

mod board;
use board::BOARD_HEIGHT_BUFFER_U;
use board::Board;

mod controls;
use controls::ControlScheme;

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
    let mut rustrisnt = Rustrisnt::new(ctx, 18u8);

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
    // control_scheme: ControlScheme,
    // input: controls::Input,
    // spawn_piece_flag: bool,
    // active_piece: piece::Piece,
    // drawing
    text: graphics::Text,
    tile_size: f32,
    batch_empty_tile: spritebatch::SpriteBatch,
    vec_batch_player_tile: Vec<spritebatch::SpriteBatch>,
}

impl Rustrisnt {
    pub fn new(ctx: &mut Context, num_players: u8) -> Rustrisnt {
        // Load/create resources here: images, fonts, sounds, etc.
        let mut vec_players: Vec<Player> = vec![];
        for player in 0..num_players {
            vec_players.push(Player::new(player, ControlScheme::new(KeyCode::Left, KeyCode::Right, KeyCode::Down, KeyCode::X, KeyCode::Z)));
        }
        let image = TileGraphic::new_empty(ctx).image;
        let batch_empty_tile = spritebatch::SpriteBatch::new(image);
        let mut vec_batch_player_tile: Vec<spritebatch::SpriteBatch> = vec![];
        for player in 0..num_players {
            vec_batch_player_tile.push(spritebatch::SpriteBatch::new(TileGraphic::new_player(ctx, player).image));
        }
        Self {
            board: Board::new(6 + 4 * num_players, 20u8),
            num_players,
            vec_players,
            // control_scheme: ControlScheme::new(KeyCode::Left, KeyCode::Right, KeyCode::Down, KeyCode::X, KeyCode::Z),
            // input: controls::Input::new(),
            // spawn_piece_flag: true,
            // active_piece: piece::Piece::new(Shapes::None, 0u8),
            text: graphics::Text::new(("Hello world!", graphics::Font::default(), 24.0)),
            tile_size: 0.0,
            batch_empty_tile,
            vec_batch_player_tile,
        }
    }
}

// draw and update are done every frame
impl EventHandler for Rustrisnt {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Debug stuff...

        // draws all the active players' tiles on the top row
        // for player in 0..self.num_players {
        //     self.board.matrix[player as usize][BOARD_HEIGHT_BUFFER_U] = Tile::new(false, true, player.player_num);
        // }

        // Update code here...

        for player in self.vec_players.iter() {
            // piece spawning
            if player.spawn_piece_flag {
                player.spawn_piece_flag = false;
                player.active_piece = piece::Piece::new(Shapes::L, player.player_num); // TODO: make random sometime
                player.active_piece.spawn(3u8 + player.player_num);
                self.board.playerify_piece(player.player_num, &player.active_piece.positions);
            }

            // piece movement
            // CW / CCW
            if player.input.keydown_rotate_cw.1 {
                if self.board.is_valid_piece_pos(&player.active_piece.piece_pos(Movement::RotateCw), player.player_num) {
                    self.board.emptify_piece(&player.active_piece.positions);
                    player.active_piece.positions = player.active_piece.piece_pos(Movement::RotateCw);
                    self.board.playerify_piece(player.player_num, &player.active_piece.positions);
                }
            }
            if player.input.keydown_rotate_ccw.1 {
                if self.board.is_valid_piece_pos(&player.active_piece.piece_pos(Movement::RotateCcw), player.player_num) {
                    self.board.emptify_piece(&player.active_piece.positions);
                    player.active_piece.positions = player.active_piece.piece_pos(Movement::RotateCcw);
                    self.board.playerify_piece(player.player_num, &player.active_piece.positions);
                }
            }
            // LEFT / RIGHT
            if player.input.keydown_left.1 {
                if self.board.is_valid_piece_pos(&player.active_piece.piece_pos(Movement::Left), player.player_num) {
                    self.board.emptify_piece(&player.active_piece.positions);
                    player.active_piece.positions = player.active_piece.piece_pos(Movement::Left);
                    self.board.playerify_piece(player.player_num, &player.active_piece.positions);
                }
            }
            if player.input.keydown_right.1 {
                if self.board.is_valid_piece_pos(&player.active_piece.piece_pos(Movement::Right), player.player_num) {
                    self.board.emptify_piece(&player.active_piece.positions);
                    player.active_piece.positions = player.active_piece.piece_pos(Movement::Right);
                    self.board.playerify_piece(player.player_num, &player.active_piece.positions);
                }
            }
            // DOWN
            // down is interesting because every time the downwards position is false we have to check if it's running into the bottom or an inactive tile so we know if we should lock it
            if player.input.keydown_down.1 {
                if self.board.is_valid_piece_pos(&player.active_piece.piece_pos(Movement::Down), player.player_num) {
                    self.board.emptify_piece(&player.active_piece.positions);
                    player.active_piece.positions = player.active_piece.piece_pos(Movement::Down);
                    self.board.playerify_piece(player.player_num, &player.active_piece.positions);
                } else if self.board.should_lock(&player.active_piece.positions) {
                    self.board.lock_piece(&player.active_piece.positions, player.player_num);
                    player.spawn_piece_flag = true;
                }
            }

            // update controls (always do last in update for each player)
            player.input.was_unpressed_previous_frame_setfalse();
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
            for mut player in self.vec_players {
                // POTENTIAL OPTIMIZATION: have a check elsewhere that makes sure no two input overlap and then just return after it finds what an input goes to; also in key_up_event()
                match player.control_scheme.find_move(keycode) {
                    Movement::Left => player.input.keydown_left = (true, true),
                    Movement::Right => player.input.keydown_right = (true, true),
                    Movement::Down => player.input.keydown_down = (true, true),
                    Movement::RotateCw => player.input.keydown_rotate_cw = (true, true),
                    Movement::RotateCcw => player.input.keydown_rotate_ccw = (true, true),
                    Movement::None => return,
                }
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        for mut player in self.vec_players {
            match player.control_scheme.find_move(keycode) {
                Movement::Left => player.input.keydown_left = (false, false),
                Movement::Right => player.input.keydown_right = (false, false),
                Movement::Down => player.input.keydown_down = (false, false),
                Movement::RotateCw => player.input.keydown_rotate_cw = (false, false),
                Movement::RotateCcw => player.input.keydown_rotate_ccw = (false, false),
                Movement::None => return,
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (window_width, window_height) = graphics::size(ctx);
        graphics::clear(ctx, graphics::BLACK);
        self.tile_size = TileGraphic::get_size(ctx, self.board.width, self.board.height);

        for x in 0..self.board.width {
            for y in 0..self.board.height {
                // empty tiles
                if self.board.matrix[x as usize][(y + BOARD_HEIGHT_BUFFER_U) as usize].empty {
                    let x = x as f32;
                    let y = (y + BOARD_HEIGHT_BUFFER_U) as f32;
                    let empty_tile = graphics::DrawParam::new().dest(Point2::new(x * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, (y - BOARD_HEIGHT_BUFFER_U as f32) * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32));
                    self.batch_empty_tile.add(empty_tile);
                } else {
                    // player tiles
                    for player in 0..self.num_players {
                        if self.board.matrix[x as usize][(y + BOARD_HEIGHT_BUFFER_U) as usize].player == player {
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

        // clear sprite batches; this is a bit inefficient and should maybe be changed to using sprite indices
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