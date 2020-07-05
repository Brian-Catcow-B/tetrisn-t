use ggez;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra as na;
use na::Point2;
use na::Vector2;

// file systems stuff
use std::path;
use std::env;
use std::io::{Read, Write};

// this constant is for the two unseen columns above the board so that when an I piece is rotated
// right after spawning, the two tiles that go above the board are kept track of
const BOARD_HEIGHT_BUFFER_U: u8 = 2;
// the amount of columns that should be on either side of the board to account for next pieces, score, etc
const BOARD_WIDTH_EXTENSION_LR: u8 = 6;
// these constants always point to the lower right corner of the window no matter what size the window actually is
const RELATIVE_WINDOW_WIDTH: f32 = 800.0;
const RELATIVE_WINDOW_HEIGHT: f32 = 600.0;

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

#[derive(Clone)]
struct Tile {
    empty: bool,
    active: bool,
    player: u8,
}

impl Tile {
    pub fn new_empty() -> Self {
        Self {
            empty: true,
            active: false,
            player: 0xffu8,
        }
    }

    // TODO: figure out how to fill the rest of the `Self` arguments as what they are and optimize modify_fill and modify_empty
    pub fn modify_fill(&mut self, active: bool, player: u8) -> Self {
        Self {
            empty: false,
            active: active,
            player: player,
        }
    }

    pub fn modify_empty(&mut self) -> Self {
        Self {
            empty: false,
            active: false,
            player: self.player,
        }
    }
}

struct TileGraphic {
    image: graphics::Image,
}

const NUM_PIXEL_ROWS: u16 = 8;
const GRAY: (u8, u8, u8) = (150u8, 150u8, 150u8);
const DARK_GRAY: (u8, u8, u8) = (84u8, 84u8, 84u8);

impl TileGraphic {
    pub fn new_empty(ctx: &mut Context) -> Self {
        // create a pixel buffer big enough to hold 4 u8's for each pixel because rgba
        let mut pixel_buf: [u8; 4 * (NUM_PIXEL_ROWS as usize) * (NUM_PIXEL_ROWS as usize)] = [0u8; 4 * (NUM_PIXEL_ROWS as usize) * (NUM_PIXEL_ROWS as usize)];
        for row_index in 0..NUM_PIXEL_ROWS {
            for col_index in 0..NUM_PIXEL_ROWS {
                if row_index == 0 || row_index == NUM_PIXEL_ROWS - 1 || col_index == 0 || col_index == NUM_PIXEL_ROWS - 1 {
                    pixel_buf[(row_index * NUM_PIXEL_ROWS * 4 + col_index * 4 + 0) as usize] = DARK_GRAY.0;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS * 4 + col_index * 4 + 1) as usize] = DARK_GRAY.1;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS * 4 + col_index * 4 + 2) as usize] = DARK_GRAY.2;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS * 4 + col_index * 4 + 3) as usize] = 0xff;
                } else {
                    pixel_buf[(row_index * NUM_PIXEL_ROWS * 4 + col_index * 4 + 0) as usize] = GRAY.0;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS * 4 + col_index * 4 + 1) as usize] = GRAY.1;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS * 4 + col_index * 4 + 2) as usize] = GRAY.2;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS * 4 + col_index * 4 + 3) as usize] = 0xff;
                }
            }
        }
        Self{
            image: graphics::Image::from_rgba8(ctx, NUM_PIXEL_ROWS, NUM_PIXEL_ROWS, &pixel_buf).expect("Failed to create background tile image"),
        }
    }

    pub fn get_size(ctx: &mut Context, board_width: u8, board_height: u8) -> f32 {
        std::cmp::min(graphics::size(ctx).1 as u32 / board_height as u32, graphics::size(ctx).0 as u32 / board_width as u32) as f32
    }

    pub fn _print_image_buf(self, ctx: &mut Context) {
        let image_buf: Vec<u8> = self.image.to_rgba8(ctx).expect("Failed to create image buffer");
        for index in 0..image_buf.len() {
            if index % 4 == 0 {
                if index % 32 == 0 {
                    print!("\n");
                } else {
                    if index != 0 {
                        print!(" ");
                    }
                }
            }
            print!("{:02x}", image_buf[index]);
        }
        print!("\n");
    }
}

struct Board {
    board_width: u8,
    board_height: u8,
    board: Vec<Vec<Tile>>,
}

impl Board {
    pub fn new(board_width: u8, board_height: u8) -> Self {
        Self {
            board_width: board_width,
            board_height: board_height,
            board: vec![vec![Tile::new_empty(); (board_height + BOARD_HEIGHT_BUFFER_U) as usize]; board_width as usize],
        }
    }
}

struct Rustrisnt {
    num_players: u8,
    board: Board,
    // canvas: graphics::Canvas,
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
            board: Board::new(100u8, 20u8),
            // canvas: graphics::Canvas::with_window_size(ctx).expect("Failed to create canvas based on window size"),
            text: graphics::Text::new(("Hello world!", graphics::Font::default(), 24.0)),
            tile_size: TileGraphic::get_size(ctx, 14u8, 20u8),
            batch_empty_tile: batch_empty_tile,
        }
    }
}

// draw and update are done every frame
impl EventHandler for Rustrisnt {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
                // println!("Added empty tile");

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // empty tiles as the first layer
        self.tile_size = TileGraphic::get_size(ctx, self.board.board_width, self.board.board_height);
        for x in 0..self.board.board_width {
            for y in 0..self.board.board_height {
                let x = x as f32;
                let y = y as f32;
                let empty_tile = graphics::DrawParam::new().dest(Point2::new(x * NUM_PIXEL_ROWS as f32, y * NUM_PIXEL_ROWS as f32));
                self.batch_empty_tile.add(empty_tile);
            }
        }
        graphics::draw(ctx, &self.batch_empty_tile, DrawParam::new().dest(Point2::new(RELATIVE_WINDOW_WIDTH / 2.0 - (self.tile_size * NUM_PIXEL_ROWS as f32 * self.board.board_width as f32 / (2.0 * 10.0)), 0.0)).scale(Vector2::new(self.tile_size / 10.0, self.tile_size / 10.0)))?;

        graphics::present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}