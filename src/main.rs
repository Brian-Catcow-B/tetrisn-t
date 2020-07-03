use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;

const BOARD_HEIGHT_BUFFER: u8 = 2;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("Rustrisn-t", "Catcow")
           .build()
           .unwrap();

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
            board: vec![vec![Tile::new(); board_height as usize]; board_width as usize],
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
        graphics::clear(ctx, graphics::WHITE);

        // Draw code here...

        graphics::present(ctx)
    }
}