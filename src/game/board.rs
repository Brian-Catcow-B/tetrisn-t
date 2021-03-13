use crate::game::piece::{Movement, Piece, Shapes};
use crate::game::tile::Tile;
use crate::game::{
    CLEAR_DELAY, SCORE_DOUBLE_BASE, SCORE_QUADRUPLE_BASE, SCORE_SINGLE_BASE, SCORE_TRIPLE_BASE,
};

// this constant is for the two unseen columns above the board so that when an I piece is rotated
// right after spawning, the two tiles that go above the board are kept track of
pub const BOARD_HEIGHT_BUFFER_U: u8 = 2;

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub matrix: Vec<Vec<Tile>>,
    pub vec_active_piece: Vec<Piece>,
    pub vec_full_lines: Vec<FullLine>,
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
        let matrix = vec![
            vec![Tile::default(); board_width as usize];
            (board_height + BOARD_HEIGHT_BUFFER_U) as usize
        ];

        // DEBUG TILES ADDED
        // let mut matrix = vec![vec![Tile::new_empty(); board_width as usize]; (board_height + BOARD_HEIGHT_BUFFER_U) as usize];
        // for x in 0..(board_width - 1) {
        //     for y in (board_height + BOARD_HEIGHT_BUFFER_U - 8)..(board_height + BOARD_HEIGHT_BUFFER_U) {
        //         matrix[y as usize][x as usize] = Tile::new(false, false, 0u8);
        //     }
        // }

        Self {
            width: board_width,
            height: board_height,
            matrix,
            vec_active_piece,
            vec_full_lines: vec![],
        }
    }

    fn emptify_piece(&mut self, player: u8) {
        for position in self.vec_active_piece[player as usize]
            .positions
            .iter()
            .take(4)
        {
            if position != &(0xffu8, 0xffu8) {
                self.matrix[position.0 as usize][position.1 as usize] = Tile::default();
            } else {
                println!("[!] tried to emptify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }

    pub fn playerify_piece(&mut self, player: u8) {
        for position in self.vec_active_piece[player as usize]
            .positions
            .iter()
            .take(4)
        {
            if position != &(0xffu8, 0xffu8) {
                self.matrix[position.0 as usize][position.1 as usize] =
                    Tile::new(false, true, player);
            } else {
                println!("[!] tried to playerify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }

    // returns (bool, bool) based on (blocked, blocked by some !active tile)
    pub fn attempt_piece_spawn(
        &mut self,
        player: u8,
        spawn_col: u8,
        spawn_piece_shape: Shapes,
    ) -> (bool, bool) {
        let new_piece = Piece::new(spawn_piece_shape);
        let spawn_positions = new_piece.spawn_pos(spawn_col);
        let mut blocked_flag: bool = false;
        for position in spawn_positions.iter().take(4) {
            if !self.matrix[position.0 as usize][position.1 as usize].empty {
                if !self.matrix[position.0 as usize][position.1 as usize].active {
                    return (true, true);
                }
                blocked_flag = true;
            }
        }
        if blocked_flag {
            return (true, false);
        }
        self.vec_active_piece[player as usize] = new_piece;
        self.vec_active_piece[player as usize].positions = spawn_positions;
        (false, false)
    }

    // returns (bool, bool) based on (if piece moved successfully, if (piece is locked && filled some line))
    // sets the shape of the piece to Shapes::None if it locks
    pub fn attempt_piece_movement(&mut self, movement: Movement, player: u8) -> (bool, bool) {
        let mut cant_move_flag = false;
        // determine if it can move
        let new_positions = self.vec_active_piece[player as usize].piece_pos(movement);
        for position in new_positions.iter().take(4) {
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
                    && self.matrix[position.0 as usize][position.1 as usize].player == player)
            {
                cant_move_flag = true;
                break;
            }
        }

        if cant_move_flag {
            if movement == Movement::Down && self.should_lock(player) {
                // lock piece and push any full lines to vec_full_lines
                self.vec_active_piece[player as usize].shape = Shapes::None;
                let mut full_line_rows: Vec<u8> = Vec::with_capacity(4);
                for row in &self.lock_piece(player) {
                    if self.is_row_full(*row) {
                        full_line_rows.push(*row);
                    }
                }
                if !full_line_rows.is_empty() {
                    for row in full_line_rows.iter() {
                        self.vec_full_lines.push(FullLine::new(
                            *row,
                            full_line_rows.len() as u8,
                            player,
                        ));
                    }
                    self.vec_full_lines.sort();
                }

                return (false, !full_line_rows.is_empty());
            }

            return (false, false);
        }

        // move it
        self.emptify_piece(player);
        self.vec_active_piece[player as usize].positions = new_positions;
        self.playerify_piece(player);

        // update self.piece.rotation if it was a rotate
        if movement == Movement::RotateCw {
            self.vec_active_piece[player as usize].rotation =
                (self.vec_active_piece[player as usize].rotation + 1)
                    % self.vec_active_piece[player as usize].num_rotations;
        }
        if movement == Movement::RotateCcw {
            self.vec_active_piece[player as usize].rotation =
                (self.vec_active_piece[player as usize].rotation
                    + self.vec_active_piece[player as usize].num_rotations
                    - 1)
                    % self.vec_active_piece[player as usize].num_rotations;
        }

        (true, false)
    }

    fn should_lock(&self, player: u8) -> bool {
        for position in self.vec_active_piece[player as usize]
            .positions
            .iter()
            .take(4)
        {
            // we just want to know if moving down by 1 will run the piece into the bottom of the board or an inactive tile
            if position.0 as usize + 1 >= (self.height + BOARD_HEIGHT_BUFFER_U) as usize {
                return true;
            }
            if !self.matrix[position.0 as usize + 1][position.1 as usize].active
                && !self.matrix[position.0 as usize + 1][position.1 as usize].empty
            {
                return true;
            }
        }

        false
    }

    // returns y position(s) of the locked piece to test if it filled a line
    fn lock_piece(&mut self, player: u8) -> Vec<u8> {
        for position in self.vec_active_piece[player as usize]
            .positions
            .iter()
            .take(4)
        {
            self.matrix[position.0 as usize][position.1 as usize] = Tile::new(false, false, player);
        }

        let mut y_vals: Vec<u8> = vec![self.vec_active_piece[player as usize].positions[0].0];
        if self.vec_active_piece[player as usize].positions[1].0
            != self.vec_active_piece[player as usize].positions[0].0
        {
            y_vals.push(self.vec_active_piece[player as usize].positions[1].0);
        }
        if self.vec_active_piece[player as usize].positions[2].0
            != self.vec_active_piece[player as usize].positions[1].0
            && self.vec_active_piece[player as usize].positions[2].0
                != self.vec_active_piece[player as usize].positions[0].0
        {
            y_vals.push(self.vec_active_piece[player as usize].positions[2].0);
        }
        if self.vec_active_piece[player as usize].positions[3].0
            != self.vec_active_piece[player as usize].positions[2].0
            && self.vec_active_piece[player as usize].positions[3].0
                != self.vec_active_piece[player as usize].positions[1].0
            && self.vec_active_piece[player as usize].positions[3].0
                != self.vec_active_piece[player as usize].positions[0].0
        {
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
        if self.vec_full_lines.is_empty() {
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

        if vec_clearing_now_indices.is_empty() {
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
            let player_num =
                self.vec_full_lines[vec_clearing_now_indices[checked_lines_for_scoring]].player;
            let mut lines_player_cleared = 1;
            // go through and determine how many lines this player cleared this time
            while checked_lines_for_scoring + lines_player_cleared < lines_cleared {
                if self.vec_full_lines
                    [vec_clearing_now_indices[checked_lines_for_scoring + lines_player_cleared]]
                    .player
                    == player_num
                {
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
                }
            };
        }

        // emptify pieces here before clear lines so the tiles don't move with the line clear and then playerify them after
        for player in 0..self.vec_active_piece.len() {
            if self.vec_active_piece[player].shape != Shapes::None {
                self.emptify_piece(player as u8);
            }
        }

        // it is very helpful that we sort self.vec_full_lines because then the lines that are waiting to be cleared are easy to find
        // (just iterate backwards from where we are in self.vec_full_lines because we are removing clearing_now_lines by iterating forward,
        // so any line being cleared now will have been cleared up to the offset we are at in self.vec_full_lines);
        // we do have to worry about getting the right index as we do this, since we are removing from a vector as we parse it, so
        // we simply have a variable that increments each time we remove an element from the vector, and then subtract that variable
        // from whatever index we are trying to access, since each index beyond what we removed will be incremented
        let mut indices_destroyed = 0;
        for index in &vec_clearing_now_indices {
            self.matrix
                .remove(self.vec_full_lines[index - indices_destroyed].row as usize);
            self.matrix
                .insert(0, vec![Tile::default(); self.width as usize]);
            self.vec_full_lines.remove(index - indices_destroyed);
            indices_destroyed += 1;
            // now is when we step backwards through the self.vec_full_lines vector,
            // incrementing the row value of each element so when it gets cleared it lines up correctly
            let mut inc_row_backwards_index = 0;
            // help this feels like magic
            while *index as isize - indices_destroyed as isize >= 0
                && *index as isize - indices_destroyed as isize - inc_row_backwards_index as isize
                    >= 0
            {
                self.vec_full_lines[*index - indices_destroyed - inc_row_backwards_index].row += 1;
                inc_row_backwards_index += 1;
            }
        }

        // playerify pieces
        for player in 0..self.vec_active_piece.len() {
            if self.vec_active_piece[player].shape != Shapes::None {
                self.playerify_piece(player as u8);
            }
        }

        (lines_cleared as u8, score)
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub struct FullLine {
    pub row: u8,
    pub lines_cleared_together: u8,
    pub player: u8,
    pub clear_delay: i8,
    pub remove_flag: bool,
}

impl FullLine {
    pub fn new(row: u8, lines_cleared_together: u8, player: u8) -> Self {
        Self {
            row,
            lines_cleared_together,
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
        let board_width = 5;
        let board_height = 20;
        let num_players = 3;
        let mut score: u64 = 0;
        let mut num_cleared_lines: u16 = 0;
        let mut board = Board::new(board_width, board_height, num_players);

        for x in 0..4 {
            for y in
                (board_height + BOARD_HEIGHT_BUFFER_U - 8)..board_height + BOARD_HEIGHT_BUFFER_U
            {
                board.matrix[y as usize][x as usize] = Tile::new(false, false, 0u8);
            }
        }

        board.attempt_piece_spawn(1, 2, Shapes::I);
        board.attempt_piece_movement(Movement::RotateCw, 1);
        board.attempt_piece_movement(Movement::Right, 1);
        board.attempt_piece_movement(Movement::Right, 1);
        for _ in 0..19 {
            board.attempt_piece_movement(Movement::Down, 1);
        }

        board.attempt_piece_spawn(2, 2, Shapes::I);
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

        assert_eq!(
            (num_cleared_lines, score),
            (8, (2 * SCORE_QUADRUPLE_BASE as u32 * (1)) as u64)
        );
        println!("[~] Passed scoring 2 tetrises on the same frame");

        // now try with some L's because that can break it
        let mut score: u64 = 0;
        let mut num_cleared_lines: u16 = 0;

        for x in 0..board_width - 2 {
            for y in
                (board_height + BOARD_HEIGHT_BUFFER_U - 4)..board_height + BOARD_HEIGHT_BUFFER_U
            {
                board.matrix[y as usize][x as usize] = Tile::new(false, false, 0u8);
            }
        }

        board.attempt_piece_spawn(1, 2, Shapes::L);
        board.attempt_piece_movement(Movement::RotateCcw, 1);
        board.attempt_piece_movement(Movement::Right, 1);
        for _ in 0..19 {
            board.attempt_piece_movement(Movement::Down, 1);
        }

        // run 1 frame of clear lines between creating the FullLines
        let (returned_lines, returned_score) = board.attempt_clear_lines(0);
        if returned_lines > 0 {
            num_cleared_lines += returned_lines as u16;
            score += returned_score as u64;
        }

        board.attempt_piece_spawn(2, 2, Shapes::L);
        board.attempt_piece_movement(Movement::RotateCw, 2);
        board.attempt_piece_movement(Movement::Right, 2);
        board.attempt_piece_movement(Movement::Right, 2);
        for _ in 0..18 {
            board.attempt_piece_movement(Movement::Down, 2);
        }

        // now clear and see what happens
        for _ in 0..=CLEAR_DELAY {
            let (returned_lines, returned_score) = board.attempt_clear_lines(0);
            if returned_lines > 0 {
                num_cleared_lines += returned_lines as u16;
                score += returned_score as u64;
            }
        }

        assert_eq!(
            (num_cleared_lines, score),
            (
                4,
                (1 * SCORE_SINGLE_BASE as u32 * (1) + 1 * SCORE_TRIPLE_BASE as u32 * (1)) as u64
            )
        );
        println!("[~] Passed scoring a single as one player and then a triple as another player one frame after");

        // now clear 2 tetrises, the second one 1 frame after the other, which can also break things
        let mut score: u64 = 0;
        let mut num_cleared_lines: u16 = 0;

        for x in 0..board_width - 1 {
            for y in
                (board_height + BOARD_HEIGHT_BUFFER_U - 8)..board_height + BOARD_HEIGHT_BUFFER_U
            {
                board.matrix[y as usize][x as usize] = Tile::new(false, false, 0u8);
            }
        }

        board.attempt_piece_spawn(1, 2, Shapes::I);
        board.attempt_piece_movement(Movement::RotateCw, 1);
        board.attempt_piece_movement(Movement::Right, 1);
        board.attempt_piece_movement(Movement::Right, 1);
        for _ in 0..19 {
            board.attempt_piece_movement(Movement::Down, 1);
        }

        // run 1 frame of clear lines between creating the FullLines
        let (returned_lines, returned_score) = board.attempt_clear_lines(0);
        if returned_lines > 0 {
            num_cleared_lines += returned_lines as u16;
            score += returned_score as u64;
        }

        board.attempt_piece_spawn(2, 2, Shapes::I);
        board.attempt_piece_movement(Movement::RotateCw, 2);
        board.attempt_piece_movement(Movement::Right, 2);
        board.attempt_piece_movement(Movement::Right, 2);
        for _ in 0..15 {
            board.attempt_piece_movement(Movement::Down, 2);
        }

        // now to clear 2 Tetrises one frame apart and see what happens
        for _ in 0..=CLEAR_DELAY {
            let (returned_lines, returned_score) = board.attempt_clear_lines(0);
            if returned_lines > 0 {
                num_cleared_lines += returned_lines as u16;
                score += returned_score as u64;
            }
        }

        assert_eq!(
            (num_cleared_lines, score),
            (8, (2 * SCORE_QUADRUPLE_BASE as u32 * (1)) as u64)
        );
        println!("[~] Passed scoring 2 tetrises one frame apart");
    }
}
