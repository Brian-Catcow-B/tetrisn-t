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
            matrix: vec![vec![Tile::new_empty(); board_width as usize]; (board_height + BOARD_HEIGHT_BUFFER_U) as usize],
        }
    }

    pub fn emptify_piece(&mut self, positions: &[(u8, u8); 4]) {
        for position in positions.iter().take(4) {
            if position != &(0xffu8, 0xffu8) {
                self.matrix[position.0 as usize][position.1 as usize] = Tile::new_empty();
            } else {
                println!("[!] tried to emptify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }

    pub fn playerify_piece(&mut self, player: u8, positions: &[(u8, u8); 4]) {
        for position in positions.iter().take(4) {
            if position != &(0xffu8, 0xffu8) {
                self.matrix[position.0 as usize][position.1 as usize] = Tile::new(false, true, player);
            } else {
                println!("[!] tried to playerify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }

    // TODO: perhaps here I should pass in &Piece, but not sure exactly how to do that rn. Too bad
    pub fn is_valid_piece_pos(&self, positions: &[(u8, u8); 4], player: u8) -> bool {
        for position in positions.iter().take(4) {
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

    pub fn should_lock(&self, positions: &[(u8, u8); 4]) -> bool {
        for position in positions.iter().take(4) {
            // we just want to know if moving down by 1 will run the piece into the bottom of the board or an inactive tile
            if position.1 as usize + 1 >= BOARD_HEIGHT_BUFFER_U as usize {
                return true;
            }
            if !self.matrix[position.0 as usize][position.1 as usize + 1].active && !self.matrix[position.0 as usize][position.1 as usize + 1].empty {
                return true;
            }
        }

        false
    }

    // TODO: perhaps here I should pass in &Piece, but not sure exactly how to do that rn. Too bad
    // returns y position(s) of the locked piece to test if it filled a line
    pub fn lock_piece(&mut self, positions: &[(u8, u8); 4], player: u8) -> Vec<u8> {
        for position in positions.iter().take(4) {
            self.matrix[position.0 as usize][position.1 as usize] = Tile::new(false, false, player);
        }

        let mut y_vals: Vec<u8> = vec![positions[0].1];
        if positions[1].1 != positions[0].1 {
            y_vals.push(positions[1].1);
        }
        if positions[2].1 != positions[1].1 && positions[2].1 != positions[0].1 {
            y_vals.push(positions[2].1);
        }
        if positions[3].1 != positions[2].1 && positions[3].1 != positions[1].1 && positions[3].1 != positions[0].1 {
            y_vals.push(positions[3].1);
        }

        y_vals
    }

    pub fn is_row_full(&self, row: u8) -> bool {
        for col in 0..self.width {
            if self.matrix[col as usize][row as usize].empty {
                return false;
            }
        }

        true
    }

    // pub fn clear_line(&mut self, row: u8) {
    //     self.matrix
    //     for col in 0..self.width {
    //         self.matrix[col as usize][self.height + BOARD_HEIGHT_BUFFER_U] = Tile::new_empty();
    //     }
    // }
}

pub struct FullLine {
    pub row_index: u8,
    pub player: u8,
    pub clear_delay: i8
}

impl FullLine {
    pub fn new(row_index: u8, player: u8) -> Self {
        Self {
            row_index,
            player,
            clear_delay: 20,
        }
    }
}
