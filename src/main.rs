use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::nalgebra as na;

// this constant is for the two unseen columns above the board so that when an I piece is rotated
// right after spawning, the two tiles that go above the board are kept track of
const BOARD_HEIGHT_BUFFER_U: u8 = 2;
// the amount of columns that should be on either side of the board to account for next pieces, score, etc
const BOARD_WIDTH_EXTENSION_LR: u8 = 6;
// these constants always point to the lower right corner of the window no matter what size the window actually is
const RELATIVE_WINDOW_WIDTH: f32 = 800.0;
const RELATIVE_WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    // make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Rustrisn-t", "Catcow").build().unwrap();
    // set window size
    graphics::set_resizable(&mut ctx, true).expect("Failed to set window to resizable");
    graphics::set_drawable_size(&mut ctx, 100.0, 100.0).expect("Failed to resize window");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut rustrisnt = Rustrisnt::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut rustrisnt) {
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
    pub fn new() -> Self {
        Self {
            empty: true,
            active: false,
            player: 0xffu8,
        }
    }

    pub fn get_tile_size(board_width: u8, board_height: u8) -> f32 {
        std::cmp::min(board_height as u32 / RELATIVE_WINDOW_HEIGHT as u32, board_width as u32 / RELATIVE_WINDOW_WIDTH as u32) as f32
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
            board: vec![vec![Tile::new(); (board_height + BOARD_HEIGHT_BUFFER_U) as usize]; board_width as usize],
        }
    }
}

struct Rustrisnt {
    num_players: u8,
    board: Board,
}

impl Rustrisnt {
    pub fn new(_ctx: &mut Context) -> Rustrisnt {
        // Load/create resources here: images, fonts, sounds, etc.
        Self {
            num_players: 2,
            board: Board::new(14u8, 20u8)
        }
    }
}

impl EventHandler for Rustrisnt {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Draw code here...
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            RELATIVE_WINDOW_WIDTH,
            2.0,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &circle, (na::Point2::new(800.0, 600.0),))?;

        graphics::present(ctx)
    }
}