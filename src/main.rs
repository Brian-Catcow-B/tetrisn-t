use ggez;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, DrawParam};
use ggez::nalgebra as na;
use na::Point2;
use na::Vector2;

// file systems stuff
use std::path;
use std::env;

mod tile;
use tile::NUM_PIXEL_ROWS_PER_TILEGRAPHIC;
use tile::{Tile, TileGraphic};

mod piece;

mod board;
use board::BOARD_HEIGHT_BUFFER_U;
use board::Board;

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
    let mut rustrisnt = Rustrisnt::new(ctx);

    // Run!
    match event::run(ctx, event_loop, &mut rustrisnt) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Rustrisnt {
    num_players: u8,
    board: Board,
    text: graphics::Text,
    tile_size: f32,
    batch_empty_tile: graphics::spritebatch::SpriteBatch,
}

impl Rustrisnt {
    pub fn new(mut ctx: &mut Context) -> Rustrisnt {
        // Load/create resources here: images, fonts, sounds, etc.
        let image = TileGraphic::new_empty(ctx).image;
        let batch_empty_tile = graphics::spritebatch::SpriteBatch::new(image);
        Self {
            num_players: 2,
            board: Board::new(14u8, 20u8),
            text: graphics::Text::new(("Hello world!", graphics::Font::default(), 24.0)),
            tile_size: 0.0,
            batch_empty_tile: batch_empty_tile,
        }
    }
}

// draw and update are done every frame
impl EventHandler for Rustrisnt {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (window_width, window_height) = graphics::size(ctx);
        graphics::clear(ctx, graphics::BLACK);
        // empty tiles as the first layer
        self.tile_size = TileGraphic::get_size(ctx, self.board.board_width, self.board.board_height);
        for x in 0..self.board.board_width {
            for y in 0..self.board.board_height {
                let x = x as f32;
                let y = y as f32;
                let empty_tile = graphics::DrawParam::new().dest(Point2::new(x * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32, y * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32)); // TODO: see if scaling these up to the right size here as opposed to when the spritebatch is drawn fixes blurriness
                self.batch_empty_tile.add(empty_tile);
            }
        }
        graphics::draw(ctx, &self.batch_empty_tile, DrawParam::new().dest(Point2::new(window_width / 2.0 - (self.tile_size * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32 * self.board.board_width as f32 / (2.0 * 8.5)), 0.0)).scale(Vector2::new(self.tile_size / 8.5, self.tile_size / 8.5)))?;

        graphics::present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}