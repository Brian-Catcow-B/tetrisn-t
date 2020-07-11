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

    pub fn emptify_piece(&mut self, positions: &[(u8, u8); 4]) {
        for pos_index in 0..4 {
            if positions[pos_index] != (0xffu8, 0xffu8) {
                self.matrix[positions[pos_index].0 as usize][positions[pos_index].1 as usize] = Tile::new_empty();
            } else {
                println!("[!] tried to emptify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }

    pub fn playerify_piece(&mut self, player: u8, positions: &[(u8, u8); 4]) {
        for pos_index in 0..4 {
            if positions[pos_index] != (0xffu8, 0xffu8) {
                self.matrix[positions[pos_index].0 as usize][positions[pos_index].1 as usize] = Tile::new(false, true, player);
            } else {
                println!("[!] tried to playerify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }
}