use crate::game::piece::{Movement, Piece, Shapes};
use crate::game::tile::Tile;
use crate::game::Modes;
use crate::game::{
    CLEAR_DELAY, SCORE_DOUBLE_BASE, SCORE_QUADRUPLE_BASE, SCORE_SINGLE_BASE, SCORE_TRIPLE_BASE,
};

static BH_WRONG_MODE: &str = "[!] BoardHandler has wrong option";

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Gravity {
    Down,
    Left,
    Up,
    Right,
}

impl From<u8> for Gravity {
    fn from(value: u8) -> Gravity {
        match value {
            0 => Gravity::Down,
            1 => Gravity::Left,
            2 => Gravity::Up,
            3 => Gravity::Right,
            _ => panic!("Unknown Gravity value: {}", value),
        }
    }
}

// abstract the board and the possible gamemodes into one struct
pub struct BoardHandler {
    pub mode: Modes,
    pub classic: Option<BoardClassic>,
    pub rotatris: Option<BoardRotatris>,
}

impl BoardHandler {
    pub fn new(board_width: u8, board_height: u8, num_players: u8, mode: Modes) -> Self {
        // determine some rules based on gamemode
        let (board_height_buffer, spawn_row) = match mode {
            Modes::Rotatris => (0, (board_height - 1) / 2),
            Modes::Classic => (2, 0),
        };
        let mut classic: Option<BoardClassic> = None;
        let mut rotatris: Option<BoardRotatris> = None;
        match mode {
            Modes::Rotatris => {
                rotatris = Some(BoardRotatris::new(board_width, spawn_row, num_players))
            }
            Modes::Classic => {
                classic = Some(BoardClassic::new(
                    board_width,
                    board_height,
                    board_height_buffer,
                    spawn_row,
                    num_players,
                ))
            }
        }
        Self {
            mode,
            classic,
            rotatris,
        }
    }

    // get...
    pub fn get_width(&mut self) -> u8 {
        match self.mode {
            Modes::Classic => self.classic.expect(BH_WRONG_MODE).width,
            Modes::Rotatris => self.rotatris.expect(BH_WRONG_MODE).board_size,
        }
    }

    pub fn get_height(&mut self) -> u8 {
        match self.mode {
            Modes::Classic => self.classic.expect(BH_WRONG_MODE).height,
            Modes::Rotatris => self.rotatris.expect(BH_WRONG_MODE).board_size,
        }
    }

    pub fn get_height_buffer(&mut self) -> u8 {
        match self.mode {
            Modes::Classic => self.classic.expect(BH_WRONG_MODE).height_buffer,
            Modes::Rotatris => 0u8,
        }
    }

    pub fn get_active_from_pos(&mut self, y: u8, x: u8) -> bool {
        match self.mode {
            Modes::Classic => {
                self.classic.expect(BH_WRONG_MODE).matrix[y as usize][x as usize].active
            }
            Modes::Rotatris => {
                self.rotatris.expect(BH_WRONG_MODE).matrix[y as usize][x as usize].active
            }
        }
    }

    pub fn get_empty_from_pos(&mut self, y: u8, x: u8) -> bool {
        match self.mode {
            Modes::Classic => {
                self.classic.expect(BH_WRONG_MODE).matrix[y as usize][x as usize].empty
            }
            Modes::Rotatris => {
                self.rotatris.expect(BH_WRONG_MODE).matrix[y as usize][x as usize].empty
            }
        }
    }

    pub fn get_player_from_pos(&mut self, y: u8, x: u8) -> u8 {
        match self.mode {
            Modes::Classic => {
                self.classic.expect(BH_WRONG_MODE).matrix[y as usize][x as usize].player
            }
            Modes::Rotatris => {
                self.rotatris.expect(BH_WRONG_MODE).matrix[y as usize][x as usize].player
            }
        }
    }

    pub fn get_shape_from_pos(&mut self, y: u8, x: u8) -> Shapes {
        match self.mode {
            Modes::Classic => {
                self.classic.expect(BH_WRONG_MODE).matrix[y as usize][x as usize].shape
            }
            Modes::Rotatris => {
                self.rotatris.expect(BH_WRONG_MODE).matrix[y as usize][x as usize].shape
            }
        }
    }

    pub fn get_shape_from_player(&mut self, player: u8) -> Shapes {
        match self.mode {
            Modes::Classic => {
                self.classic.expect(BH_WRONG_MODE).vec_active_piece[player as usize].shape
            }
            Modes::Rotatris => {
                self.rotatris.expect(BH_WRONG_MODE).vec_active_piece[player as usize].shape
            }
        }
    }

    // board logic functions
    pub fn attempt_piece_spawn(
        &mut self,
        player: u8,
        spawn_col: u8,
        spawn_piece_shape: Shapes,
    ) -> (bool, bool) {
        match self.mode {
            Modes::Classic => self.classic.expect(BH_WRONG_MODE).attempt_piece_spawn(
                player,
                spawn_col,
                spawn_piece_shape,
            ),
            Modes::Rotatris => self.rotatris.expect(BH_WRONG_MODE).attempt_piece_spawn(
                player,
                spawn_col,
                spawn_piece_shape,
            ),
        }
    }

    pub fn attempt_clear_lines(&mut self, level: u8) -> (u8, u32) {
        match self.mode {
            Modes::Classic => self
                .classic
                .expect(BH_WRONG_MODE)
                .attempt_clear_lines(level),
            Modes::Rotatris => self
                .rotatris
                .expect(BH_WRONG_MODE)
                .rotatris_attempt_clear_rings(level),
        }
    }

    pub fn attempt_piece_movement(&mut self, m: Movement, p: u8) -> (bool, bool) {
        match self.mode {
            Modes::Classic => self
                .classic
                .expect(BH_WRONG_MODE)
                .attempt_piece_movement(m, p),
            Modes::Rotatris => self
                .rotatris
                .expect(BH_WRONG_MODE)
                .attempt_piece_movement(m, p),
        }
    }

    pub fn attempt_rotate_board(&mut self, rd: Movement) -> bool {
        match self.mode {
            Modes::Classic => {
                println!("[!] `attempt_rotate_board` called but mode is Modes::Classic");
                false
            }
            Modes::Rotatris => self.rotatris.expect(BH_WRONG_MODE).attempt_rotate_board(rd),
        }
    }

    pub fn playerify_piece(&mut self, player: u8) {
        match self.mode {
            Modes::Classic => self.classic.expect(BH_WRONG_MODE).playerify_piece(player),
            Modes::Rotatris => self.rotatris.expect(BH_WRONG_MODE).playerify_piece(player),
        }
    }
}

// example Board coordinates system (2 width, 2 height)
// [(0, 0)][(0, 1)]
// [(1, 0)][(1, 1)]

pub struct BoardClassic {
    pub width: u8,
    pub height: u8,
    pub height_buffer: u8,
    pub spawn_row: u8,
    pub matrix: Vec<Vec<Tile>>,
    pub vec_active_piece: Vec<Piece>,
    pub vec_full_lines: Vec<FullLine>,
}

impl BoardClassic {
    pub fn new(
        board_width: u8,
        board_height: u8,
        board_height_buffer: u8,
        spawn_row: u8,
        num_players: u8,
    ) -> Self {
        let mut vec_active_piece: Vec<Piece> = Vec::with_capacity(num_players as usize);
        for _ in 0..num_players {
            vec_active_piece.push(Piece::new(Shapes::None));
        }
        let matrix = vec![
            vec![Tile::default(); board_width as usize];
            (board_height + board_height_buffer) as usize
        ];

        // DEBUG TILES ADDED
        // let mut matrix = vec![vec![Tile::default(); board_width as usize]; (board_height + BOARD_HEIGHT_BUFFER_U) as usize];
        // for x in 0..(board_width - 1) {
        //     for y in (board_height + BOARD_HEIGHT_BUFFER_U - 8)..(board_height + BOARD_HEIGHT_BUFFER_U) {
        //         matrix[y as usize][x as usize] = Tile::new(false, false, 0u8);
        //     }
        // }

        Self {
            width: board_width,
            height: board_height,
            height_buffer: board_height_buffer,
            spawn_row: board_height_buffer,
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
                self.matrix[position.0 as usize][position.1 as usize].empty = true;
                self.matrix[position.0 as usize][position.1 as usize].active = false;
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
                self.matrix[position.0 as usize][position.1 as usize] = Tile::new(
                    false,
                    true,
                    player,
                    self.vec_active_piece[player as usize].shape,
                );
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
        let spawn_positions =
            new_piece.spawn_pos(spawn_col, self.spawn_row, self.height_buffer, Gravity::Down);
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
        // initialize the tile logic for the newly spawned piece
        for position in spawn_positions.iter().take(4) {
            self.matrix[position.0 as usize][position.1 as usize] =
                Tile::new(false, true, player, spawn_piece_shape);
        }

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
            if position.0 >= self.height + self.height_buffer {
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
            if position.0 as usize + 1 >= (self.height + self.height_buffer) as usize {
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
            self.matrix[position.0 as usize][position.1 as usize].empty = false;
            self.matrix[position.0 as usize][position.1 as usize].active = false;
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

// rotatris
pub struct BoardRotatris {
    pub gravity: Gravity,
    pub board_size: u8,
    pub spawn_row: u8,
    pub matrix: Vec<Vec<Tile>>,
    pub vec_active_piece: Vec<Piece>,
}

impl BoardRotatris {
    pub fn new(board_size: u8, spawn_row: u8, num_players: u8) -> Self {
        let mut vec_active_piece: Vec<Piece> = Vec::with_capacity(num_players as usize);
        for _ in 0..num_players {
            vec_active_piece.push(Piece::new(Shapes::None));
        }
        let mut matrix = vec![vec![Tile::default(); board_size as usize]; board_size as usize];

        // DEBUG
        for a in 0..4 {
            for b in 0..board_size as usize {
                matrix[a][b] = Tile::new(false, false, 0, Shapes::I);
                matrix[b][a] = Tile::new(false, false, 0, Shapes::I);
                matrix[board_size as usize - a - 1][b] = Tile::new(false, false, 0, Shapes::I);
                matrix[b][board_size as usize - a - 1] = Tile::new(false, false, 0, Shapes::I);
            }
        }
        matrix[board_size as usize / 2][board_size as usize - 1] = Tile::default();
        matrix[board_size as usize / 2][board_size as usize - 2] = Tile::default();
        matrix[board_size as usize / 2][board_size as usize - 3] = Tile::default();
        matrix[board_size as usize / 2][board_size as usize - 4] = Tile::default();

        Self {
            gravity: Gravity::Down,
            board_size,
            spawn_row,
            matrix,
            vec_active_piece,
        }
    }

    // return bool is if rotate was successful
    pub fn attempt_rotate_board(&mut self, rotate_direction: Movement) -> bool {
        let center: u8 = self.board_size / 2;
        let is_center_even: u8 = (self.board_size + 1) % 2;
        let mut new_positions: [(u8, u8); 4] = [(0u8, 0u8); 4];
        match rotate_direction {
            Movement::RotateCw => {
                for (index, position) in self.vec_active_piece[0]
                    .positions
                    .iter()
                    .take(4)
                    .enumerate()
                {
                    new_positions[index] = (position.1, center * 2 - position.0 - is_center_even);
                }
            }
            Movement::RotateCcw => {
                for (index, position) in self.vec_active_piece[0]
                    .positions
                    .iter()
                    .take(4)
                    .enumerate()
                {
                    new_positions[index] = (center * 2 - position.1 - is_center_even, position.0);
                }
            }
            _ => {
                println!("[!] Sent some non-rotation Movement to `attempt_rotate_board()`, a method of `BoardHandler`");
                return false;
            }
        }

        // check validity of new positions
        for position in new_positions.iter().take(4) {
            if !self.matrix[position.0 as usize][position.1 as usize].empty
                && !self.matrix[position.0 as usize][position.1 as usize].active
            {
                return false;
            }
        }

        self.emptify_piece(0);
        self.vec_active_piece[0].positions = new_positions;
        self.playerify_piece(0);

        self.gravity = Gravity::from(
            (self.gravity as u8
                + if rotate_direction == Movement::RotateCw {
                    3
                } else {
                    1
                })
                % 4,
        );

        true
    }

    pub fn playerify_piece(&mut self, player: u8) {
        for position in self.vec_active_piece[player as usize]
            .positions
            .iter()
            .take(4)
        {
            if position != &(0xffu8, 0xffu8) {
                self.matrix[position.0 as usize][position.1 as usize] = Tile::new(
                    false,
                    true,
                    player,
                    self.vec_active_piece[player as usize].shape,
                );
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
        let new_positions = self.vec_active_piece[player as usize].piece_pos(movement);
        for position in new_positions.iter().take(4) {
            // due to integer underflow (u8 board width and u8 board height), we must only check the positive side of x and y positions
            if position.0 >= self.board_size + self.board_size {
                cant_move_flag = true;
                break;
            }
            if position.1 >= self.board_size {
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
            // TODO: fix this awful code
            if movement as u8 == self.gravity as u8 && self.should_lock(player) {
                // lock piece and push any full lines to vec_full_lines
                self.vec_active_piece[player as usize].shape = Shapes::None;

                let mut does_full_ring_exist: bool = false;
                for ring in &self.lock_piece(player) {
                    if self.rotatris_check_single_ring(*ring) {
                        does_full_ring_exist = true;
                    }
                }

                return (false, does_full_ring_exist);
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

    // returns (bool, bool) based on (blocked, blocked by some !active tile)
    pub fn attempt_piece_spawn(
        &mut self,
        player: u8,
        spawn_col: u8,
        spawn_piece_shape: Shapes,
    ) -> (bool, bool) {
        let new_piece = Piece::new(spawn_piece_shape);
        let spawn_positions =
            new_piece.spawn_pos(spawn_col, self.spawn_row, self.board_size / 2, self.gravity);
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
        // initialize the tile logic for the newly spawned piece
        for position in spawn_positions.iter().take(4) {
            self.matrix[position.0 as usize][position.1 as usize] =
                Tile::new(false, true, player, spawn_piece_shape);
        }

        (false, false)
    }

    // returns ring(s) of the locked piece to test if it filled a line
    fn lock_piece(&mut self, player: u8) -> Vec<u8> {
        for position in self.vec_active_piece[player as usize]
            .positions
            .iter()
            .take(4)
        {
            self.matrix[position.0 as usize][position.1 as usize].empty = false;
            self.matrix[position.0 as usize][position.1 as usize].active = false;
        }

        let rings_with_repeat: [u8; 4] = [
            self.find_ring_from_pos(
                self.vec_active_piece[player as usize].positions[0].0,
                self.vec_active_piece[player as usize].positions[0].1,
            ),
            self.find_ring_from_pos(
                self.vec_active_piece[player as usize].positions[1].0,
                self.vec_active_piece[player as usize].positions[1].1,
            ),
            self.find_ring_from_pos(
                self.vec_active_piece[player as usize].positions[2].0,
                self.vec_active_piece[player as usize].positions[2].1,
            ),
            self.find_ring_from_pos(
                self.vec_active_piece[player as usize].positions[3].0,
                self.vec_active_piece[player as usize].positions[3].1,
            ),
        ];

        let mut rings: Vec<u8> = vec![rings_with_repeat[0]];
        if rings_with_repeat[0] != rings_with_repeat[1] {
            rings.push(rings_with_repeat[1]);
        }
        if rings_with_repeat[0] != rings_with_repeat[2]
            && rings_with_repeat[1] != rings_with_repeat[2]
        {
            rings.push(rings_with_repeat[2]);
        }
        if rings_with_repeat[0] != rings_with_repeat[3]
            && rings_with_repeat[1] != rings_with_repeat[3]
            && rings_with_repeat[2] != rings_with_repeat[3]
        {
            rings.push(rings_with_repeat[3]);
        }

        rings
    }

    fn find_ring_from_pos(&self, y: u8, x: u8) -> u8 {
        std::cmp::min(
            std::cmp::min(x, self.board_size - x - 1),
            std::cmp::min(y, self.board_size - y - 1),
        )
    }

    fn should_lock(&self, player: u8) -> bool {
        for position in self.vec_active_piece[player as usize]
            .positions
            .iter()
            .take(4)
        {
            // we just want to know if moving down by 1 will run the piece into the bottom of the board or an inactive tile
            match self.gravity {
                Gravity::Down => {
                    if position.0 as usize + 1 >= self.board_size as usize {
                        return true;
                    }
                    if !self.matrix[position.0 as usize + 1][position.1 as usize].active
                        && !self.matrix[position.0 as usize + 1][position.1 as usize].empty
                    {
                        return true;
                    }
                }
                Gravity::Left => {
                    if position.1 <= 0 {
                        return true;
                    }
                    if !self.matrix[position.0 as usize][position.1 as usize - 1].active
                        && !self.matrix[position.0 as usize][position.1 as usize - 1].empty
                    {
                        return true;
                    }
                }
                Gravity::Up => {
                    if position.0 <= 0 {
                        return true;
                    }
                    if !self.matrix[position.0 as usize - 1][position.1 as usize].active
                        && !self.matrix[position.0 as usize - 1][position.1 as usize].empty
                    {
                        return true;
                    }
                }
                Gravity::Right => {
                    if position.1 >= self.board_size - 1 {
                        return true;
                    }
                    if !self.matrix[position.0 as usize][position.1 as usize + 1].active
                        && !self.matrix[position.0 as usize][position.1 as usize + 1].empty
                    {
                        return true;
                    }
                }
                _ => panic!("[!] Error: current gravity is {}", self.gravity as u8),
            }
        }

        false
    }

    fn emptify_piece(&mut self, player: u8) {
        for position in self.vec_active_piece[player as usize]
            .positions
            .iter()
            .take(4)
        {
            if position != &(0xffu8, 0xffu8) {
                self.matrix[position.0 as usize][position.1 as usize].empty = true;
                self.matrix[position.0 as usize][position.1 as usize].active = false;
            } else {
                println!("[!] tried to emptify piece that contained position (0xffu8, 0xffu8)");
            }
        }
    }

    pub fn rotatris_attempt_clear_rings(&mut self, level: u8) -> (u8, u32) {
        let mut num_cleared_rings = 0;
        let mut score_from_cleared_rings = 0;
        let num_rings_to_check = self.board_size / 2 - 4;

        // go from inner rings to outer rings checking if any ring is full, avoiding the middle 4 rings
        for z in (0..num_rings_to_check).rev() {
            if self.rotatris_check_single_ring(z) {
                num_cleared_rings += 1;
                // clear and pull inner stuff out
                for j in (z + 1)..num_rings_to_check {
                    self.rotatris_pull_single_ring_out(j);
                }
            }
        }

        score_from_cleared_rings += match num_cleared_rings {
            1 => SCORE_SINGLE_BASE as u32 * (level as u32 + 1),
            2 => SCORE_DOUBLE_BASE as u32 * (level as u32 + 1),
            3 => SCORE_TRIPLE_BASE as u32 * (level as u32 + 1),
            4 => SCORE_QUADRUPLE_BASE as u32 * (level as u32 + 1),
            _ => {
                println!("[!] player was attributed a number of lines too large maybe, what the heck? lines_player_cleared: {}", num_cleared_rings);
                0u32
            }
        };

        (num_cleared_rings, score_from_cleared_rings)
    }

    fn rotatris_check_single_ring(&mut self, z: u8) -> bool {
        let min = std::cmp::min(z, self.board_size - z);
        let max = std::cmp::max(z, self.board_size - z);
        for a in [min, max - 1].into_iter() {
            for b in min..max {
                if b >= min && b <= max {
                    if self.matrix[*a as usize][b as usize].empty
                        || self.matrix[*a as usize][b as usize].active
                        || self.matrix[b as usize][*a as usize].empty
                        || self.matrix[b as usize][*a as usize].active
                    {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn rotatris_pull_single_ring_out(&mut self, j: u8) {
        let j = j as usize;
        let k = self.board_size as usize - j - 1;

        // sides
        for a in j..=k {
            // top
            self.matrix[j - 1][a] = self.matrix[j][a];
            // left
            self.matrix[a][j - 1] = self.matrix[a][j];
            // down
            self.matrix[k + 1][a] = self.matrix[k][a];
            // right
            self.matrix[a][k + 1] = self.matrix[a][k];
        }
        // corners
        self.matrix[j - 1][j - 1] = self.matrix[j][j];
        self.matrix[j - 1][k + 1] = self.matrix[j][k];
        self.matrix[k + 1][j - 1] = self.matrix[k][j];
        self.matrix[k + 1][k + 1] = self.matrix[k][k];
    }

    fn rotatris_emptify_single_ring(&mut self, z: u8) {
        for a in [z, self.board_size - z - 1].into_iter() {
            for b in z..(self.board_size - z) {
                if b >= z && b <= self.board_size - z {
                    self.matrix[*a as usize][b as usize].empty = true;
                    self.matrix[*a as usize][b as usize].active = false;
                    self.matrix[b as usize][*a as usize].empty = true;
                    self.matrix[b as usize][*a as usize].active = false;
                }
            }
        }
    }
}

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
        let mut board = BoardClassic::new(board_width, board_height, num_players);

        for x in 0..4 {
            for y in (board_height + self.height_buffer - 8)..board_height + self.height_buffer {
                board.matrix[y as usize][x as usize] = Tile::new(false, false, 0u8, Shapes::I);
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
            for y in (board_height + self.height_buffer - 4)..board_height + self.height_buffer {
                board.matrix[y as usize][x as usize] = Tile::new(false, false, 0u8, Shapes::I);
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
            for y in (board_height + self.height_buffer - 8)..board_height + self.height_buffer {
                board.matrix[y as usize][x as usize] = Tile::new(false, false, 0u8, Shapes::I);
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
