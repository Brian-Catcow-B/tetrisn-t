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
    vec_full_lines: Vec<FullLine>,
}

impl Board {
    pub fn new(board_width: u8, board_height: u8, num_players: u8) -> Self {
        let mut vec_active_piece: Vec<Piece> = Vec::with_capacity(num_players);
        for _ in 0..num_players {
            vec_active_piece.push(Piece::new(Shapes::None));
        }
        Self {
            width: board_width,
            height: board_height,
            matrix: vec![vec![Tile::new_empty(); board_width as usize]; (board_height + BOARD_HEIGHT_BUFFER_U) as usize],
            vec_active_piece,
            vec_full_lines: vec![],
        }
    }

    fn emptify_piece(&mut self, player: u8) {
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

    // returns bool based on if (piece is locked && filled some line)
    pub fn attempt_piece_movement(&mut self, movement: Movement, player: u8) -> bool {
        let mut cant_move_flag = false;
        // determine if it can move
        for position in self.vec_active_piece[player as usize].piece_pos(movement).iter().take(4) {
            // due to integer underflow (u8 board width and u8 board height), we must only check the positive side of x and y positions
            if position.0 >= self.height + BOARD_HEIGHT_BUFFER_U {
                cant_move_flag = true;
                break;
            }
            if position.1 >= self.width {
                cant_move_flag = true;
                break;
            }
            // make sure the position is empty or is part of the piece being moved
            if !self.matrix[position.0 as usize][position.1 as usize].empty
                && !(self.matrix[position.0 as usize][position.1 as usize].active
                && self.matrix[position.0 as usize][position.1 as usize].player == player) {
                cant_move_flag = true;
                break;
            }
        }

        if cant_move_flag {
            println!("(movement == Movement::Down) = {}, self.should_lock(player) == {}", movement == Movement::Down, self.should_lock(player));
            if movement == Movement::Down && self.should_lock(player) {
                // lock piece and push any full lines to vec_full_lines
                self.vec_active_piece[player as usize].shape = Shapes::None;
                let mut is_full_line = false;
                for row in self.lock_piece(player).iter() {
                    if self.is_row_full(*row) {
                        is_full_line = true;
                        self.vec_full_lines.push(FullLine::new(*row, player));
                        println!("pushed a thing to the thing with row {}, player {}", *row, player);
                    }
                }
                if is_full_line {
                    self.vec_full_lines.sort();
                }

                return is_full_line;
            }

            return false;
        }

        // move it
        self.emptify_piece(player);
        self.vec_active_piece[player as usize].positions = self.vec_active_piece[player as usize].piece_pos(movement);
        self.playerify_piece(player);

        // update self.piece.rotation if it was a rotate
        if movement == Movement::RotateCw {
            self.vec_active_piece[player as usize].rotation = (self.vec_active_piece[player as usize].rotation + 1) % 4;
        }
        if movement == Movement::RotateCcw {
            self.vec_active_piece[player as usize].rotation = (self.vec_active_piece[player as usize].rotation + 3) % 4;
        }

        false
    }

    fn should_lock(&self, player: u8) -> bool {
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

    // returns y position(s) of the locked piece to test if it filled a line
    fn lock_piece(&mut self, player: u8) -> Vec<u8> {
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

    fn is_row_full(&self, row: u8) -> bool {
        for tile in self.matrix[row as usize].iter() {
            if tile.empty || tile.active {
                return false;
            }
        }

        true
    }

    pub fn attempt_clear_lines(&mut self, num_players: u8) {
        // go through the clear delays and dec if > 0, push index to vec_clearing_now_indices if <= 0
        let mut vec_clearing_now_indices: Vec<usize> = vec![];
        for (full_line_index, full_line) in self.vec_full_lines.iter_mut().enumerate() {
            if full_line.clear_delay > 0 {
                // you must construct additional pylons
                full_line.clear_delay -= 1;
            } else {
                // you have constructed additional pylons
                vec_clearing_now_indices.push(full_line_index);
            }
        }

        // it is very helpful that we sort self.vec_full_lines because then the lines that are waiting to be cleared are easy to find
        // (just iterate backwards from where we are in self.vec_full_lines because we are removing clearing_now_lines by iterating forward,
        // so any line being cleared now will have been cleared up to the offset we are at in self.vec_full_lines);
        // we do have to worry about getting the right index as we do this, since we are removing from a vector as we parse it, so
        // we simply have a variable that increments each time we remove an element from the vector, and then subtract that variable
        // from whatever index we are trying to access, since each index beyond what we removed will be incremented
        let mut indices_destroyed = 0;
        for index in vec_clearing_now_indices.iter() {
            self.matrix.remove(self.vec_full_lines[index - indices_destroyed].row as usize);
            self.matrix.insert(0, vec![Tile::new_empty(); self.width as usize]);
            self.vec_full_lines.remove(index - indices_destroyed);
            indices_destroyed += 1;
            // now is when we step backwards through the self.vec_full_lines vector,
            // incrementing the row value of each element so when it gets cleared it lines up correctly
            let mut backwards_inc_row_index = 0;
            // help this feels like magic
            while (*index - indices_destroyed) as isize >= 0 && (vec_clearing_now_indices[*index - indices_destroyed] - backwards_inc_row_index - 1) as isize >= 0 {
                self.vec_full_lines[vec_clearing_now_indices[*index - indices_destroyed] - backwards_inc_row_index - 1].row += 1;
                backwards_inc_row_index += 1;
            }
        }
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq)]
struct FullLine {
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
            clear_delay: crate::CLEAR_DELAY,
            remove_flag: false,
        }
    }
}
