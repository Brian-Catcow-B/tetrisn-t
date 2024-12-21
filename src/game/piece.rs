use crate::game::board::{BoardDim, BoardPos, Gravity};
use crate::movement::{Movement, RotationDirection};

use std::convert::TryFrom;

pub type PieceSetId = usize;
pub const PIECE_SET_NULL_ID: PieceSetId = usize::MAX;
pub struct PieceManager {
    pub vec_pieceset: Vec<Piece>,
}

impl TryFrom<String> for PieceManager {
    type Error = &'static str;
    fn try_from(_: String) -> Result<Self, <Self as TryFrom<String>>::Error> { todo!() }
}

impl From<&PieceManager> for String {
    fn from(pm: &PieceManager) -> String {
        todo!()
    }
}

impl PieceManager {
    pub fn generate_random_piece_set_id(&mut self, player_previous_piece_set_id: PieceSetId) -> PieceSetId {
        // TODO: make this random
        0
    }

    pub fn get_piece_clone(&self, id: PieceSetId) -> Piece {
        self.vec_pieceset[id].clone()
    }
}

pub type NextRotationDirection = RotationDirection;
#[derive(Copy, Clone)]
pub enum PivotType {
    QuadRotation,
    BiRotation(NextRotationDirection),
}

impl PivotType {
    fn get_num_rotations(&self) -> u8 {
        match self {
            Self::QuadRotation => 4,
            Self::BiRotation(_) => 2,
        }
    }
}

#[derive(Copy, Clone)]
struct Pivot {
    pub pivot_type: PivotType,
    pub position: (BoardPos, BoardPos) // y, x
}

impl Pivot {
    fn get_num_rotations(&self) -> u8 {
        self.pivot_type.get_num_rotations()
    }
}

#[derive(Clone)]
pub struct Piece {
    pub block_positions: Vec<(BoardPos, BoardPos)>, // y, x
    pub pivot: Option<Pivot>,
    pub rotation: u8, // 0, 1, 2, 3: 0, 90, 180, 270; CW
    pub piece_set_id: PieceSetId,
}

impl TryFrom<String> for Piece {
    type Error = &'static str;
    fn try_from(_: String) -> Result<Self, <Self as TryFrom<String>>::Error> { todo!() }
}

impl From<&Piece> for String {
    fn from(p: &Piece) -> String {
        todo!()
    }
}

impl Piece {
    // 1, 2, or 4 based on the style of rotation the piece has
    pub fn get_num_rotations(&self) -> u8 {
        match self.pivot {
            Some(ref p) => p.get_num_rotations(),
            None => 1,
        }
    }

    pub fn get_width(&self) -> BoardDim {
        if self.block_positions.is_empty() {
            panic!("Piece::get_piece_width called with empty self.block_positions vector");
        }
        let (mut min_x, mut max_x): (BoardPos, BoardPos) = (self.block_positions[0].1, self.block_positions[0].1);
        for i in 1..self.block_positions.len() {
            min_x = std::cmp::min(min_x, self.block_positions[i].1);
            max_x = std::cmp::max(max_x, self.block_positions[i].1);
        }
        max_x - min_x + 1
    }

/*#[derive(Copy, Clone)]
pub struct Piece {
    pub shape: Shapes,
    pub positions: [(BoardPos, BoardPos); 4], // y, x
    pub rotation: u8, // 0, 1, 2, 3: 0, 90, 180, 270; CW
    pub num_rotations: u8,
    pivot: usize,
}

impl Piece {*/
    /*pub fn new(shape: Shapes) -> Self {
        match shape {
            // The I piece is the special case for rotation because then the logic is easier for pieces of num_rotations: 2
            // (notice the pivots and which direction the piece must turn from the spawned positions)
            Shapes::I => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 1,
                num_rotations: 2,
                pivot: 2,
            },
            Shapes::O => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 0,
                num_rotations: 1,
                pivot: 0xff,
            },
            Shapes::T => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 0,
                num_rotations: 4,
                pivot: 1,
            },
            Shapes::J => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 0,
                num_rotations: 4,
                pivot: 1,
            },
            Shapes::L => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 0,
                num_rotations: 4,
                pivot: 1,
            },
            Shapes::S => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 0,
                num_rotations: 2,
                pivot: 0,
            },
            Shapes::Z => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 0,
                num_rotations: 2,
                pivot: 1,
            },
            _ => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 0,
                num_rotations: 0,
                pivot: 0xff,
            },
        }
    }*/

    /*pub fn new_next(shape: Shapes) -> Self {
        match shape {
            Shapes::None => Self {
                shape,
                positions: [(0xff, 0xff); 4],
                rotation: 0,
                num_rotations: 0,
                pivot: 0xff,
            },
            Shapes::I => Self {
                shape,
                positions: [(0, 0), (0, 1), (0, 2), (0, 3)],
                rotation: 0,
                num_rotations: 2,
                pivot: 2,
            },
            Shapes::O => Self {
                shape,
                positions: [(0, 1), (0, 2), (1, 1), (1, 2)],
                rotation: 0,
                num_rotations: 1,
                pivot: 0xff,
            },
            Shapes::T => Self {
                shape,
                positions: [(0, 1), (0, 2), (0, 3), (1, 2)],
                rotation: 0,
                num_rotations: 4,
                pivot: 1,
            },
            Shapes::J => Self {
                shape,
                positions: [(0, 1), (0, 2), (0, 3), (1, 3)],
                rotation: 0,
                num_rotations: 4,
                pivot: 1,
            },
            Shapes::L => Self {
                shape,
                positions: [(0, 1), (0, 2), (0, 3), (1, 1)],
                rotation: 0,
                num_rotations: 4,
                pivot: 1,
            },
            Shapes::S => Self {
                shape,
                positions: [(0, 2), (0, 3), (1, 1), (1, 2)],
                rotation: 0,
                num_rotations: 2,
                pivot: 0,
            },
            Shapes::Z => Self {
                shape,
                positions: [(0, 1), (0, 2), (1, 2), (1, 3)],
                rotation: 0,
                num_rotations: 2,
                pivot: 1,
            },
        }
    }*/

    // spawn_column should favor right when necessary because this function favors left when
    // necessary
    pub fn spawn_pos(
        &self,
        spawn_column: BoardPos,
        topmost_spawn_row: BoardPos,
        board_height_buffer: BoardDim,
        current_gravity: Gravity,
    ) -> Vec<(BoardPos, BoardPos)> {
        let piece_width = self.get_width();
        let x_addend = spawn_column - (piece_width / 2);
        let y_addend = topmost_spawn_row;
        let mut piece_copy = self.clone();
        for pos in piece_copy.block_positions.iter_mut() {
            pos.0 += y_addend;
            pos.1 += x_addend;
        }
        // rotatris magic :O
        match current_gravity {
            Gravity::Down => piece_copy.block_positions,
            Gravity::Left => {
                piece_copy.block_positions = piece_copy.rotate(true);
                piece_copy.piece_pos(Movement::Right)
            }
            Gravity::Up => {
                piece_copy.block_positions = piece_copy.double_rotate();
                piece_copy.piece_pos(Movement::Down)
            }
            Gravity::Right => {
                piece_copy.block_positions = piece_copy.rotate(false);
                piece_copy.piece_pos(Movement::Down)
            }
            Gravity::Invalid => unreachable!("[!] Gravity::Invalid passed into Piece::spawn_pos()"),
        }
    }

    // returns the resulting positions based on the given Movement type
    pub fn piece_pos(&self, movement: Movement) -> Vec<(BoardPos, BoardPos)> {
        // for movements and rotations, we don't have to worry about integer underflow because we will assume the board width is nowhere close to 0xff
        let rotation_regulated_movement = match movement {
            Movement::RotateCcw | Movement::RotateCw => {
                match self.pivot {
                    Some(ref p) => {
                        match p.pivot_type {
                            PivotType::QuadRotation => movement,
                            PivotType::BiRotation(next_rot) => Movement::from(next_rot),
                        }
                    },
                    None => Movement::None,
                }
            },
            _ => movement,
        };
        let mut new_positions: Vec<(BoardPos, BoardPos)> = vec![];
        match rotation_regulated_movement {
            Movement::Down => {
                for pos in self.block_positions.iter() {
                    new_positions.push((pos.0 + 1, pos.1));
                }
                new_positions
            },
            Movement::Left => {
                for pos in self.block_positions.iter() {
                    new_positions.push((pos.0, pos.1 - 1));
                }
                new_positions
            },
            Movement::Up => {
                for pos in self.block_positions.iter() {
                    new_positions.push((pos.0 - 1, pos.1));
                }
                new_positions
            },
            Movement::Right => {
                for pos in self.block_positions.iter() {
                    new_positions.push((pos.0, pos.1 + 1));
                }
                new_positions
            },
            Movement::RotateCw => {
                self.rotate(true)
            },
            Movement::RotateCcw => {
                self.rotate(false)
            },
            Movement::None => self.block_positions, // ggez :D
            _ => {
                unreachable!("Invalid Movement enum given to Piece::piece_pos");
            },
        }
    }

    fn rotate(&self, clockwise_flag: bool) -> Vec<(BoardPos, BoardPos)> {
        let mut new_positions: Vec<(BoardPos, BoardPos)> = vec![];
        match self.pivot {
            Some(ref piv) => {
                if clockwise_flag {
                    for pos in self.block_positions.iter() {
                        // what
                        new_positions.push(
                            (
                                piv.position.0 + (pos.1 - piv.position.1),
                                piv.position.1 + (piv.position.0 - pos.0),
                            )
                        );
                    }
                } else {
                    for pos in self.block_positions.iter() {
                        // even
                        new_positions.push(
                            (
                                piv.position.0 + (piv.position.1 - pos.1),
                                piv.position.1 + (pos.0 - piv.position.0),
                            )
                        );
                    }
                }
            },
            None => unreachable!("[internal error] Piece::rotate called with self.pivot set to None"),
        }
        new_positions
    }

    fn double_rotate(&self) -> Vec<(BoardPos, BoardPos)> {
        let mut piece_copy = self.clone();
        piece_copy.block_positions = self.rotate(true);
        piece_copy.rotate(true)
    }
}

/*#[derive(PartialEq, Eq)]
pub struct NextPiece {
    pub shape: Shapes,
    pub matrix: [[bool; 4]; 2],
}

impl NextPiece {
    pub fn new(shape: Shapes) -> Self {
        if shape == Shapes::None {
            return Self {
                shape,
                matrix: [[false; 4]; 2],
            };
        }
        let mut matrix: [[bool; 4]; 2] = [[false; 4]; 2];
        for position in Piece::new_next(shape).positions.iter().take(4) {
            matrix[position.0 as usize][position.1 as usize] = true;
        }
        Self { shape, matrix }
    }
}*/
