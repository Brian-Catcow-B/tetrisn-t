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
        let mut vec_active_piece: Vec<Piece> = vec![];
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

    // returns bool based on if piece is locked
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
                // lock piece
                println!("locking piece");
                let mut lock_bool = false;
                for col_index in self.lock_piece(player).iter() {
                    if self.is_row_full(*col_index) {
                        lock_bool = true;
                        self.vec_active_piece[player as usize].shape = Shapes::None;
                        self.vec_full_lines.push(FullLine::new(*col_index, player));
                        println!("pushed a thing to the thing with row {}, player {}", *col_index, player);
                    }
                }
                return lock_bool;
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

    pub fn clear_line(&mut self, row: u8) {
        self.matrix.remove(row as usize);
        self.matrix.insert(0, vec![Tile::new_empty(); self.width as usize]);
        // TODO: this is a bad way of doing this. it will cause a crash later
        // not actually though, it just pulls down active pieces as well
        // it also sucks
    }
}

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
            clear_delay: 20,
            remove_flag: false,
        }
    }
}
