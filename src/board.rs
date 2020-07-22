use crate::tile::Tile;
use crate::piece::{Piece, Shapes, Movement};

// this constant is for the two unseen columns above the board so that when an I piece is rotated
// right after spawning, the two tiles that go above the board are kept track of
pub const BOARD_HEIGHT_BUFFER_U: u8 = 2;

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub matrix: Vec<Vec<Tile>>,
    pub vec_active_piece: Vec<Piece>,
}

impl Board {
    pub fn new(board_width: u8, board_height: u8, num_players: u8) -> Self {
        let mut vec_active_piece: Vec<Piece> = vec![];
        for player_index in 0..num_players {
            vec_active_piece.push(Piece::new(Shapes::None, player_index));
        }
        Self {
            width: board_width,
            height: board_height,
            matrix: vec![vec![Tile::new_empty(); board_width as usize]; (board_height + BOARD_HEIGHT_BUFFER_U) as usize],
            vec_active_piece,
        }
    }

    pub fn emptify_piece(&mut self, player: u8) {
        for position in self.vec_active_piece[player as usize].positions.iter().take(4) {
            if position != &(0xffu8, 0xffu8) {
                self.matrix[position.0 as usize][position.1 as usize] = Tile::new_empty();
            } else {
                println!("[!] tried to emptify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }

    pub fn playerify_piece(&mut self, player: u8) {
        for position in self.vec_active_piece[player as usize].positions.iter().take(4) {
            if position != &(0xffu8, 0xffu8) {
                self.matrix[position.0 as usize][position.1 as usize] = Tile::new(false, true, player);
            } else {
                println!("[!] tried to playerify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }

    // TODO: perhaps here I should pass in &Piece, but not sure exactly how to do that rn. Too bad
    pub fn attempt_piece_movement(&self, movement: Movement, player: u8) -> bool {
        for position in self.vec_active_piece[player as usize].piece_pos(movement).iter().take(4) {
            // due to integer underflow (u8 board width and u8 board height), we must only check the positive side of x and y positions
            if position.0 >= self.height + BOARD_HEIGHT_BUFFER_U {
                return false;
            }
            if position.1 >= self.width {
                return false;
            }
            // make sure the position is not empty and is not part of the piece being moved
            if !self.matrix[position.0 as usize][position.1 as usize].empty && !(self.matrix[position.0 as usize][position.1 as usize].active && self.matrix[position.0 as usize][position.1 as usize].player == player) {
                return false;
            }
        }

        true
    }

    pub fn should_lock(&self, player: u8) -> bool {
        for position in self.vec_active_piece[player as usize].positions.iter().take(4) {
            // we just want to know if moving down by 1 will run the piece into the bottom of the board or an inactive tile
            if position.0 as usize + 1 >= (self.height + BOARD_HEIGHT_BUFFER_U) as usize {
                return true;
            }
            if !self.matrix[position.0 as usize + 1][position.1 as usize].active && !self.matrix[position.0 as usize + 1][position.1 as usize].empty {
                return true;
            }
        }

        false
    }

    // TODO: perhaps here I should pass in &Piece, but not sure exactly how to do that rn. Too bad
    // returns y position(s) of the locked piece to test if it filled a line
    pub fn lock_piece(&mut self, player: u8) -> Vec<u8> {
        for position in self.vec_active_piece[player as usize].positions.iter().take(4) {
            self.matrix[position.0 as usize][position.1 as usize] = Tile::new(false, false, player);
        }

        let mut y_vals: Vec<u8> = vec![self.vec_active_piece[player as usize].positions[0].0];
        if self.vec_active_piece[player as usize].positions[1].0 != self.vec_active_piece[player as usize].positions[0].0 {
            y_vals.push(self.vec_active_piece[player as usize].positions[1].0);
        }
        if self.vec_active_piece[player as usize].positions[2].0 != self.vec_active_piece[player as usize].positions[1].0 && self.vec_active_piece[player as usize].positions[2].0 != self.vec_active_piece[player as usize].positions[0].0 {
            y_vals.push(self.vec_active_piece[player as usize].positions[2].0);
        }
        if self.vec_active_piece[player as usize].positions[3].0 != self.vec_active_piece[player as usize].positions[2].0 && self.vec_active_piece[player as usize].positions[3].0 != self.vec_active_piece[player as usize].positions[1].0 && self.vec_active_piece[player as usize].positions[3].0 != self.vec_active_piece[player as usize].positions[0].0 {
            y_vals.push(self.vec_active_piece[player as usize].positions[3].0);
        }

        y_vals
    }

    pub fn is_row_full(&self, row: u8) -> bool {
        for tile in self.matrix[row as usize].iter() {
            if tile.empty || tile.active {
                return false;
            }
        }

        true
    }

    pub fn clear_line(&mut self, row: u8) {
        self.matrix.remove(row as usize);
        self.matrix.insert(0, vec![Tile::new_empty(); self.width as usize]);
        // TODO: this is a bad way of doing this. it will cause a crash later
        // not actually though, it just pulls down active pieces as well
    }
}

pub struct FullLine {
    pub row: u8,
    pub player: u8,
    pub clear_delay: i8,
    pub remove_flag: bool,
}

impl FullLine {
    pub fn new(row: u8, player: u8) -> Self {
        Self {
            row,
            player,
            clear_delay: 20,
            remove_flag: false,
        }
    }
}
