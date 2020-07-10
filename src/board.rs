use crate::tile::Tile;

// this constant is for the two unseen columns above the board so that when an I piece is rotated
// right after spawning, the two tiles that go above the board are kept track of
pub const BOARD_HEIGHT_BUFFER_U: u8 = 2;

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub matrix: Vec<Vec<Tile>>,
}

impl Board {
    pub fn new(board_width: u8, board_height: u8) -> Self {
        Self {
            width: board_width,
            height: board_height,
            matrix: vec![vec![Tile::new_empty(); (board_height + BOARD_HEIGHT_BUFFER_U) as usize]; board_width as usize],
        }
    }
}