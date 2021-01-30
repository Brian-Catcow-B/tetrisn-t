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

impl From<u8> for Shapes {
    fn from(value: u8) -> Shapes {
        match value {
            0 => Shapes::I,
            1 => Shapes::O,
            2 => Shapes::T,
            3 => Shapes::J,
            4 => Shapes::S,
            5 => Shapes::L,
            6 => Shapes::Z,
            _ => panic!("Unknown Shapes value: {}", value),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Movement {
    Down,
    Left,
    Up,
    Right,
    RotateCw,
    RotateCcw,
    None,
}

impl From<u8> for Movement {
    fn from(value: u8) -> Movement {
        match value {
            0 => Movement::Down,
            1 => Movement::Left,
            2 => Movement::Up,
            3 => Movement::Right,
            4 => Movement::RotateCw,
            5 => Movement::RotateCcw,
            6 => Movement::None,
            _ => panic!("[!] Unknown Movement value: {}", value),
        }
    }
}

pub struct Piece {
    pub shape: Shapes,
    pub positions: [(u8, u8); 4],
    pub rotation: u8, // 0, 1, 2, 3: 0, 90, 180, 270; CW
    pub num_rotations: u8,
    pivot: u8,
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

    pub fn spawn_pos(&self, spawn_column: u8, spawn_row: u8, board_height_buffer: u8) -> [(u8, u8); 4] {
        match self.shape {
            Shapes::None => {
                println!("[!] tried to spawn a piece with shape type Shapes::None");
                [(0xff, 0xff); 4]
            }
            Shapes::I => {
                [
                    (spawn_row + board_height_buffer, spawn_column - 2), // [-][-][-][-] | [-][-][0][-]
                    (spawn_row + board_height_buffer, spawn_column - 1), // [-][-][-][-] | [-][-][1][-]
                    (spawn_row + board_height_buffer, spawn_column),     // [0][1][2][3] | [-][-][2][-]
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
        }
    }

    // returns the position based on the given Movement type
    pub fn piece_pos(&self, r#move: Movement) -> [(u8, u8); 4] {
        // for movements and rotations, we don't have to worry about integer underflow because we will assume the board width is nowhere close to 0xff
        if r#move == Movement::None {
            self.positions
        } else if r#move == Movement::Left {
            [
                (self.positions[0].0, self.positions[0].1 - 1),
                (self.positions[1].0, self.positions[1].1 - 1),
                (self.positions[2].0, self.positions[2].1 - 1),
                (self.positions[3].0, self.positions[3].1 - 1),
            ]
        } else if r#move == Movement::Right {
            [
                (self.positions[0].0, self.positions[0].1 + 1),
                (self.positions[1].0, self.positions[1].1 + 1),
                (self.positions[2].0, self.positions[2].1 + 1),
                (self.positions[3].0, self.positions[3].1 + 1),
            ]
        } else if r#move == Movement::Down {
            [
                (self.positions[0].0 + 1, self.positions[0].1),
                (self.positions[1].0 + 1, self.positions[1].1),
                (self.positions[2].0 + 1, self.positions[2].1),
                (self.positions[3].0 + 1, self.positions[3].1),
            ]
        } else if r#move == Movement::Up {
            return [
                (self.positions[0].0 - 1, self.positions[0].1),
                (self.positions[1].0 - 1, self.positions[1].1),
                (self.positions[2].0 - 1, self.positions[2].1),
                (self.positions[3].0 - 1, self.positions[3].1),
            ];
        } else {
            // T, L, J
            if self.num_rotations == 4 {
                if r#move == Movement::RotateCw {
                    self.rotate(true)
                } else {
                    self.rotate(false)
                }
            // I, S, Z
            } else if self.num_rotations == 2 {
                // the I piece is special in that it starts with rotation: 1 so that it lines up with S and Z
                if self.rotation == 0 {
                    self.rotate(false)
                } else {
                    self.rotate(true)
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

    fn rotate(&self, clockwise_flag: bool) -> [(u8, u8); 4] {
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
