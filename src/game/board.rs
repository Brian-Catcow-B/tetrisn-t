use crate::game::tile::Tile;
use crate::game::piece::{Piece, Shapes, Movement};
use crate::game::{CLEAR_DELAY, SCORE_SINGLE_BASE, SCORE_DOUBLE_BASE, SCORE_TRIPLE_BASE, SCORE_QUADRUPLE_BASE};

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

// example Board coordinates system (2 width, 2 height)
// [(0, 0)][(0, 1)]
// [(1, 0)][(1, 1)]

impl Board {
    pub fn new(board_width: u8, board_height: u8, num_players: u8) -> Self {
        let mut vec_active_piece: Vec<Piece> = Vec::with_capacity(num_players as usize);
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

    // returns (bool, bool) based on (if piece moved successfully, if (piece is locked && filled some line))
    // sets the shape of the piece to Shapes::None if it locks
    pub fn attempt_piece_movement(&mut self, movement: Movement, player: u8) -> (bool, bool) {
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
            if movement == Movement::Down && self.should_lock(player) {
                // lock piece and push any full lines to vec_full_lines
                self.vec_active_piece[player as usize].shape = Shapes::None;
                let mut is_full_line = false;
                for row in &self.lock_piece(player) {
                    if self.is_row_full(*row) {
                        is_full_line = true;
                        self.vec_full_lines.push(FullLine::new(*row, player));
                    }
                }
                if is_full_line {
                    self.vec_full_lines.sort();
                }

                return (false, is_full_line);
            }

            return (false, false);
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

        (true, false)
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
        for tile in &self.matrix[row as usize] {
            if tile.empty || tile.active {
                return false;
            }
        }

        true
    }

    // returns (num_lines_cleared, score_from_cleared_lines)
    pub fn attempt_clear_lines(&mut self, level: u8) -> (u8, u32) {
        if self.vec_full_lines.len() == 0 {
            // nothing to see here
            return (0, 0);
        }

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

        if vec_clearing_now_indices.len() == 0 {
            // not much to see here
            return (0, 0);
        }

        // for the return value, we need to know how many lines are being cleared (the first return in the tuple)
        // and we need to know the amount of points from the lines clearing right now (the second in the tuple)
        // but we only want to score based on how many lines each individual player cleared, so for the scoring,
        // we must find a player who is clearing now, then go through the vector finding all the full lines which
        // that specific player filled, and count accordingly until we get the number of lines that player filled
        // at once; we then repeat until we have covered every player that has lines that are clearing this frame;
        // the nice thing here is, self.vec_full_lines is sorted by row every time a new FullLine is added, and it
        // is not possible in classic tetris to have one player clear 2+ lines while another player clears 1+ lines
        // inbetween the first player's lines (on the same frame (or at all?)); therefore, we can just start at whatever
        // our `checked_lines_for_scoring` variable is as the index of the `vec_clearing_now_indices` vector;
        // this is fine
        let lines_cleared = vec_clearing_now_indices.len();
        let mut score = 0;
        let mut checked_lines_for_scoring = 0;
        while checked_lines_for_scoring < lines_cleared {
            // find player number in question
            let player_num = self.vec_full_lines[vec_clearing_now_indices[checked_lines_for_scoring]].player;
            let mut lines_player_cleared = 1;
            // go through and determine how many lines this player cleared this time
            while checked_lines_for_scoring + lines_player_cleared < lines_cleared {
                if self.vec_full_lines[vec_clearing_now_indices[checked_lines_for_scoring + lines_player_cleared]].player == player_num {
                    lines_player_cleared += 1;
                } else {
                    break;
                }
            }
            checked_lines_for_scoring += lines_player_cleared;
            score += match lines_player_cleared {
                1 => SCORE_SINGLE_BASE as u32 * (level as u32 + 1),
                2 => SCORE_DOUBLE_BASE as u32 * (level as u32 + 1),
                3 => SCORE_TRIPLE_BASE as u32 * (level as u32 + 1),
                4 => SCORE_QUADRUPLE_BASE as u32 * (level as u32 + 1),
                _ => {
                    println!("[!] player was attributed a number of lines too large maybe, what the heck? lines_player_cleared: {}", lines_player_cleared);
                    0u32
                },
            };
        }

        // it is very helpful that we sort self.vec_full_lines because then the lines that are waiting to be cleared are easy to find
        // (just iterate backwards from where we are in self.vec_full_lines because we are removing clearing_now_lines by iterating forward,
        // so any line being cleared now will have been cleared up to the offset we are at in self.vec_full_lines);
        // we do have to worry about getting the right index as we do this, since we are removing from a vector as we parse it, so
        // we simply have a variable that increments each time we remove an element from the vector, and then subtract that variable
        // from whatever index we are trying to access, since each index beyond what we removed will be incremented
        let mut indices_destroyed = 0;
        for index in &vec_clearing_now_indices {
            self.matrix.remove(self.vec_full_lines[index - indices_destroyed].row as usize);
            self.matrix.insert(0, vec![Tile::new_empty(); self.width as usize]);
            self.vec_full_lines.remove(index - indices_destroyed);
            indices_destroyed += 1;
            // now is when we step backwards through the self.vec_full_lines vector,
            // incrementing the row value of each element so when it gets cleared it lines up correctly
            let mut backwards_inc_row_index = 0;
            // help this feels like magic
            while *index as isize - indices_destroyed as isize >= 0 && *index as isize - backwards_inc_row_index as isize - 1 >= 0 {
                self.vec_full_lines[*index - backwards_inc_row_index - 1].row += 1;
                backwards_inc_row_index += 1;
            }
        }

        println!("[+] cleared {} lines, scored {} points", lines_cleared, score);
        (lines_cleared as u8, score)
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
            clear_delay: CLEAR_DELAY,
            remove_flag: false,
        }
    }
}

// do `cargo test --release` because Rust doesn't like underflow, but that's how the board width works :(
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(0, 0);
    }

    #[test]
    fn clearing_and_scoring() {
        let mut test: bool = true;
        let mut score: u64 = 0;
        let mut num_cleared_lines: u16 = 0;
        let mut board = Board::new(5, 20, 3);
        for player_0_place_count in 0..8 {
            board.vec_active_piece[0] = Piece::new(Shapes::I);
            board.vec_active_piece[0].spawn(2);
            for _ in 0..20 - player_0_place_count {
                board.attempt_piece_movement(Movement::Down, 0);
            }
            if board.matrix[19 + BOARD_HEIGHT_BUFFER_U as usize - player_0_place_count][0].player == 0xffu8 {
                test = false;
            }
        }
        board.vec_active_piece[0] = Piece::new(Shapes::I);
        board.vec_active_piece[0].spawn(2);
        board.attempt_piece_movement(Movement::RotateCw, 0);
        board.attempt_piece_movement(Movement::Left, 0);
        board.attempt_piece_movement(Movement::Left, 0);
        for _ in 0..11 {
            board.attempt_piece_movement(Movement::Down, 0);
        }

        // now it should be like this
        // [-][-][-][-][-]
        // [-][-][-][-][-]
        // [-][-][-][-][-]
        // [-][-][-][-][-]
        // [-][-][-][-][-]
        // [-][-][-][-][-]
        // [-][-][-][-][-]
        // [-][-][-][-][-]
        // [0][-][-][-][-]
        // [0][-][-][-][-]
        // [0][-][-][-][-]
        // [0][-][-][-][-]
        // [0][0][0][0][-]
        // [0][0][0][0][-]
        // [0][0][0][0][-]
        // [0][0][0][0][-]
        // [0][0][0][0][-]
        // [0][0][0][0][-]
        // [0][0][0][0][-]
        // [0][0][0][0][-]

        board.vec_active_piece[1] = Piece::new(Shapes::I);
        board.vec_active_piece[1].spawn(2);
        board.attempt_piece_movement(Movement::RotateCw, 1);
        board.attempt_piece_movement(Movement::Right, 1);
        board.attempt_piece_movement(Movement::Right, 1);
        for _ in 0..19 {
            board.attempt_piece_movement(Movement::Down, 1);
        }

        board.vec_active_piece[2] = Piece::new(Shapes::I);
        board.vec_active_piece[2].spawn(2);
        board.attempt_piece_movement(Movement::RotateCw, 2);
        board.attempt_piece_movement(Movement::Right, 2);
        board.attempt_piece_movement(Movement::Right, 2);
        for _ in 0..15 {
            board.attempt_piece_movement(Movement::Down, 2);
        }

        // now to clear 2 Tetrises on the same frame and see what happens
        for _ in 0..=CLEAR_DELAY {
            let (returned_lines, returned_score) = board.attempt_clear_lines(0);
            if returned_lines > 0 {
                num_cleared_lines += returned_lines as u16;
                score += returned_score as u64;
            }
        }

        if board.matrix[16 + BOARD_HEIGHT_BUFFER_U as usize][0].empty {
            test = false;
        }

        assert_eq!((num_cleared_lines, score, test), (8, (2 * SCORE_QUADRUPLE_BASE as u32 * (1)) as u64, true));

        // now try with some L's because that has been known to break it
        let mut score: u64 = 0;
        let mut num_cleared_lines: u16 = 0;

        board.vec_active_piece[0] = Piece::new(Shapes::I);
        board.vec_active_piece[0].spawn(2);
        board.attempt_piece_movement(Movement::RotateCw, 0);
        board.attempt_piece_movement(Movement::Left, 0);
        for _ in 0..19 {
            board.attempt_piece_movement(Movement::Down, 0);
        }

        board.vec_active_piece[0] = Piece::new(Shapes::I);
        board.vec_active_piece[0].spawn(2);
        board.attempt_piece_movement(Movement::RotateCw, 0);
        for _ in 0..19 {
            board.attempt_piece_movement(Movement::Down, 0);
        }

        board.vec_active_piece[1] = Piece::new(Shapes::L);
        board.vec_active_piece[1].spawn(2);
        board.attempt_piece_movement(Movement::RotateCcw, 1);
        board.attempt_piece_movement(Movement::Right, 1);
        for _ in 0..19 {
            board.attempt_piece_movement(Movement::Down, 1);
        }

        // this is so that one piece locks in 1 frame before the other
        let (returned_lines, returned_score) = board.attempt_clear_lines(0);
        if returned_lines > 0 {
            num_cleared_lines += returned_lines as u16;
            score += returned_score as u64;
        }

        board.vec_active_piece[2] = Piece::new(Shapes::L);
        board.vec_active_piece[2].spawn(2);
        board.attempt_piece_movement(Movement::RotateCw, 2);
        board.attempt_piece_movement(Movement::Right, 2);
        board.attempt_piece_movement(Movement::Right, 2);
        for _ in 0..18 {
            board.attempt_piece_movement(Movement::Down, 2);
        }

        // now clear and see what happens
        for _ in 0..CLEAR_DELAY + 1 {
            let (returned_lines, returned_score) = board.attempt_clear_lines(0);
            if returned_lines > 0 {
                num_cleared_lines += returned_lines as u16;
                score += returned_score as u64;
            }
        }

        assert_eq!((num_cleared_lines, score), (4, (1 * SCORE_SINGLE_BASE as u32 * (1) + 1 * SCORE_TRIPLE_BASE as u32 * (1)) as u64));
    }
}
