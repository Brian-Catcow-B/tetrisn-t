use crate::game::board::{BoardDim, BoardPos, Gravity};
use crate::movement::Movement;

use std::convert::TryFrom;

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Shapes {
    I,
    O,
    T,
    J,
    S,
    L,
    Z,
    None,
}

impl TryFrom<u8> for Shapes {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Shapes::I),
            1 => Ok(Shapes::O),
            2 => Ok(Shapes::T),
            3 => Ok(Shapes::J),
            4 => Ok(Shapes::S),
            5 => Ok(Shapes::L),
            6 => Ok(Shapes::Z),
            _ => Err("Conversion failed (u8 -> Shapes): Invalid value"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub shape: Shapes,
    pub positions: [(BoardPos, BoardPos); 4],
    pub rotation: u8, // 0, 1, 2, 3: 0, 90, 180, 270; CW
    pub num_rotations: u8,
    pivot: BoardPos,
}

impl Piece {
    pub fn new(shape: Shapes) -> Self {
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
    }

    pub fn new_next(shape: Shapes) -> Self {
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
    }

    pub fn spawn_pos(
        &self,
        spawn_column: BoardPos,
        spawn_row: BoardPos,
        board_height_buffer: BoardDim,
        current_gravity: Gravity,
    ) -> [(BoardPos, BoardPos); 4] {
        let mut piece_copy = Self::new(self.shape);
        piece_copy.positions = match piece_copy.shape {
            Shapes::None => {
                println!("[!] tried to spawn a piece with shape type Shapes::None");
                [(0xff, 0xff); 4]
            }
            Shapes::I => {
                [
                    (spawn_row + board_height_buffer, spawn_column - 2), // [-][-][-][-] | [-][-][0][-]
                    (spawn_row + board_height_buffer, spawn_column - 1), // [-][-][-][-] | [-][-][1][-]
                    (spawn_row + board_height_buffer, spawn_column), //     [0][1][2][3] | [-][-][2][-]
                    (spawn_row + board_height_buffer, spawn_column + 1), // [-][-][-][-] | [-][-][3][-]
                ]
            }
            Shapes::O => {
                [
                    (spawn_row + board_height_buffer, spawn_column - 1), //     [-][-][-][-]
                    (spawn_row + board_height_buffer, spawn_column),     //     [-][-][-][-]
                    (spawn_row + 1 + board_height_buffer, spawn_column - 1), // [-][0][1][-]
                    (spawn_row + 1 + board_height_buffer, spawn_column), //     [-][2][3][-]
                ]
            }
            Shapes::T => {
                [
                    (spawn_row + board_height_buffer, spawn_column - 1), // [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                    (spawn_row + board_height_buffer, spawn_column), //     [-][-][-][-] | [-][-][0][-] | [-][-][3][-] | [-][-][2][-]
                    (spawn_row + board_height_buffer, spawn_column + 1), // [-][0][1][2] | [-][3][1][-] | [-][2][1][0] | [-][-][1][3]
                    (spawn_row + 1 + board_height_buffer, spawn_column), // [-][-][3][-] | [-][-][2][-] | [-][-][-][-] | [-][-][0][-]
                ]
            }
            Shapes::J => {
                [
                    (spawn_row + board_height_buffer, spawn_column - 1), //     [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                    (spawn_row + board_height_buffer, spawn_column), //         [-][-][-][-] | [-][-][0][-] | [-][3][-][-] | [-][-][2][3]
                    (spawn_row + board_height_buffer, spawn_column + 1), //     [-][0][1][2] | [-][-][1][-] | [-][2][1][0] | [-][-][1][-]
                    (spawn_row + 1 + board_height_buffer, spawn_column + 1), // [-][-][-][3] | [-][3][2][-] | [-][-][-][-] | [-][-][0][-]
                ]
            }
            Shapes::L => {
                [
                    (spawn_row + board_height_buffer, spawn_column - 1), //     [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                    (spawn_row + board_height_buffer, spawn_column), //         [-][-][-][-] | [-][3][0][-] | [-][-][-][3] | [-][-][2][-]
                    (spawn_row + board_height_buffer, spawn_column + 1), //     [-][0][1][2] | [-][-][1][-] | [-][2][1][0] | [-][-][1][-]
                    (spawn_row + 1 + board_height_buffer, spawn_column - 1), // [-][3][-][-] | [-][-][2][-] | [-][-][-][-] | [-][-][0][3]
                ]
            }
            Shapes::S => {
                [
                    (spawn_row + board_height_buffer, spawn_column), //         [-][-][-][-] | [-][-][-][-]
                    (spawn_row + board_height_buffer, spawn_column + 1), //     [-][-][-][-] | [-][-][1][-]
                    (spawn_row + 1 + board_height_buffer, spawn_column - 1), // [-][-][0][1] | [-][-][0][3]
                    (spawn_row + 1 + board_height_buffer, spawn_column), //     [-][2][3][-] | [-][-][-][2]
                ]
            }
            Shapes::Z => {
                [
                    (spawn_row + board_height_buffer, spawn_column - 1), //     [-][-][-][-] | [-][-][-][-]
                    (spawn_row + board_height_buffer, spawn_column), //         [-][-][-][-] | [-][-][-][3]
                    (spawn_row + 1 + board_height_buffer, spawn_column), //     [-][0][1][-] | [-][-][1][2]
                    (spawn_row + 1 + board_height_buffer, spawn_column + 1), // [-][-][2][3] | [-][-][0][-]
                ]
            }
        };
        if piece_copy.shape == Shapes::None {
            unreachable!("[!] Error: `Piece::spawn_pos()` called with shape: Shapes::None");
        } else if piece_copy.shape == Shapes::O {
            piece_copy.positions
        } else {
            match current_gravity {
                Gravity::Down => piece_copy.positions,
                Gravity::Left => {
                    piece_copy.positions = piece_copy.rotate(true);
                    piece_copy.piece_pos(Movement::Right)
                }
                Gravity::Up => {
                    piece_copy.positions = piece_copy.double_rotate();
                    piece_copy.piece_pos(Movement::Down)
                }
                Gravity::Right => {
                    piece_copy.positions = piece_copy.rotate(false);
                    piece_copy.piece_pos(Movement::Down)
                }
                Gravity::Invalid => unreachable!("[!] Gravity::Invalid passed into `spawn_pos()`"),
            }
        }
    }

    // returns the resulting positions based on the given Movement type
    pub fn piece_pos(&self, movement: Movement) -> [(BoardPos, BoardPos); 4] {
        // for movements and rotations, we don't have to worry about integer underflow because we will assume the board width is nowhere close to 0xff
        if movement == Movement::None {
            self.positions
        } else if movement == Movement::Left {
            [
                (self.positions[0].0, self.positions[0].1 - 1),
                (self.positions[1].0, self.positions[1].1 - 1),
                (self.positions[2].0, self.positions[2].1 - 1),
                (self.positions[3].0, self.positions[3].1 - 1),
            ]
        } else if movement == Movement::Right {
            [
                (self.positions[0].0, self.positions[0].1 + 1),
                (self.positions[1].0, self.positions[1].1 + 1),
                (self.positions[2].0, self.positions[2].1 + 1),
                (self.positions[3].0, self.positions[3].1 + 1),
            ]
        } else if movement == Movement::Down {
            [
                (self.positions[0].0 + 1, self.positions[0].1),
                (self.positions[1].0 + 1, self.positions[1].1),
                (self.positions[2].0 + 1, self.positions[2].1),
                (self.positions[3].0 + 1, self.positions[3].1),
            ]
        } else if movement == Movement::Up {
            return [
                (self.positions[0].0 - 1, self.positions[0].1),
                (self.positions[1].0 - 1, self.positions[1].1),
                (self.positions[2].0 - 1, self.positions[2].1),
                (self.positions[3].0 - 1, self.positions[3].1),
            ];
        } else {
            // T, L, J
            if self.num_rotations == 4 {
                if movement == Movement::RotateCw {
                    self.rotate(true)
                } else if movement == Movement::RotateCcw {
                    self.rotate(false)
                } else {
                    self.double_rotate()
                }
            // I, S, Z
            } else if self.num_rotations == 2 {
                // the I piece is special in that it starts with rotation: 1 so that it lines up with S and Z
                if movement != Movement::DoubleRotate {
                    if self.rotation == 0 {
                        self.rotate(false)
                    } else {
                        self.rotate(true)
                    }
                } else {
                    self.positions
                }
            } else if self.num_rotations == 1 {
                self.positions
            } else {
                println!(
                    "[!] tried to rotate piece with num_rotations: {}",
                    self.num_rotations
                );
                self.positions
            }
        }
    }

    fn rotate(&self, clockwise_flag: bool) -> [(BoardPos, BoardPos); 4] {
        if self.pivot > 3 {
            println!("[!] tried to rotate piece with pivot fields {}", self.pivot);
            return self.positions;
        }
        if clockwise_flag {
            [
                (
                    self.positions[self.pivot as usize].0
                        + (self.positions[0].1 - self.positions[self.pivot as usize].1),
                    self.positions[self.pivot as usize].1
                        + (self.positions[self.pivot as usize].0 - self.positions[0].0),
                ),
                (
                    self.positions[self.pivot as usize].0
                        + (self.positions[1].1 - self.positions[self.pivot as usize].1),
                    self.positions[self.pivot as usize].1
                        + (self.positions[self.pivot as usize].0 - self.positions[1].0),
                ),
                (
                    self.positions[self.pivot as usize].0
                        + (self.positions[2].1 - self.positions[self.pivot as usize].1),
                    self.positions[self.pivot as usize].1
                        + (self.positions[self.pivot as usize].0 - self.positions[2].0),
                ),
                (
                    self.positions[self.pivot as usize].0
                        + (self.positions[3].1 - self.positions[self.pivot as usize].1),
                    self.positions[self.pivot as usize].1
                        + (self.positions[self.pivot as usize].0 - self.positions[3].0),
                ),
            ]
        } else {
            [
                (
                    self.positions[self.pivot as usize].0
                        + (self.positions[self.pivot as usize].1 - self.positions[0].1),
                    self.positions[self.pivot as usize].1
                        + (self.positions[0].0 - self.positions[self.pivot as usize].0),
                ),
                (
                    self.positions[self.pivot as usize].0
                        + (self.positions[self.pivot as usize].1 - self.positions[1].1),
                    self.positions[self.pivot as usize].1
                        + (self.positions[1].0 - self.positions[self.pivot as usize].0),
                ),
                (
                    self.positions[self.pivot as usize].0
                        + (self.positions[self.pivot as usize].1 - self.positions[2].1),
                    self.positions[self.pivot as usize].1
                        + (self.positions[2].0 - self.positions[self.pivot as usize].0),
                ),
                (
                    self.positions[self.pivot as usize].0
                        + (self.positions[self.pivot as usize].1 - self.positions[3].1),
                    self.positions[self.pivot as usize].1
                        + (self.positions[3].0 - self.positions[self.pivot as usize].0),
                ),
            ]
        }
    }

    fn double_rotate(&self) -> [(BoardPos, BoardPos); 4] {
        if self.pivot > 3 {
            println!("[!] tried to rotate piece with pivot fields {}", self.pivot);
            return self.positions;
        }
        let pivot = self.pivot;
        let positions = [
            (
                self.positions[self.pivot as usize].0
                    + (self.positions[0].1 - self.positions[self.pivot as usize].1),
                self.positions[self.pivot as usize].1
                    + (self.positions[self.pivot as usize].0 - self.positions[0].0),
            ),
            (
                self.positions[self.pivot as usize].0
                    + (self.positions[1].1 - self.positions[self.pivot as usize].1),
                self.positions[self.pivot as usize].1
                    + (self.positions[self.pivot as usize].0 - self.positions[1].0),
            ),
            (
                self.positions[self.pivot as usize].0
                    + (self.positions[2].1 - self.positions[self.pivot as usize].1),
                self.positions[self.pivot as usize].1
                    + (self.positions[self.pivot as usize].0 - self.positions[2].0),
            ),
            (
                self.positions[self.pivot as usize].0
                    + (self.positions[3].1 - self.positions[self.pivot as usize].1),
                self.positions[self.pivot as usize].1
                    + (self.positions[self.pivot as usize].0 - self.positions[3].0),
            ),
        ];

        [
            (
                positions[pivot as usize].0 + (positions[0].1 - positions[pivot as usize].1),
                positions[pivot as usize].1 + (positions[pivot as usize].0 - positions[0].0),
            ),
            (
                positions[pivot as usize].0 + (positions[1].1 - positions[pivot as usize].1),
                positions[pivot as usize].1 + (positions[pivot as usize].0 - positions[1].0),
            ),
            (
                positions[pivot as usize].0 + (positions[2].1 - positions[pivot as usize].1),
                positions[pivot as usize].1 + (positions[pivot as usize].0 - positions[2].0),
            ),
            (
                positions[pivot as usize].0 + (positions[3].1 - positions[pivot as usize].1),
                positions[pivot as usize].1 + (positions[pivot as usize].0 - positions[3].0),
            ),
        ]
    }
}

#[derive(PartialEq, Eq)]
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
}
