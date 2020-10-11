use crate::game::board::BOARD_HEIGHT_BUFFER_U;

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

impl Shapes {
    pub fn from_u8(value: u8) -> Shapes {
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
    None,
    Left,
    Right,
    Down,
    RotateCw,
    RotateCcw,
    // Up,
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
            Shapes::I => {
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 1,
                    num_rotations: 2,
                    pivot: 2,
                }
            }
            Shapes::O => {
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 0,
                    num_rotations: 1,
                    pivot: 0xff,
                }
            }
            Shapes::T => {
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 0,
                    num_rotations: 4,
                    pivot: 1,
                }
            }
            Shapes::J => {
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 0,
                    num_rotations: 4,
                    pivot: 1,
                }
            }
            Shapes::L => {
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 0,
                    num_rotations: 4,
                    pivot: 1,
                }
            }
            Shapes::S => {
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 0,
                    num_rotations: 2,
                    pivot: 0,
                }
            }
            Shapes::Z => {
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 0,
                    num_rotations: 2,
                    pivot: 1,
                }
            }
            _ => {
                // println!("[+] creating new None shaped piece");
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 0,
                    num_rotations: 0,
                    pivot: 0xff,
                };
            }
        }
    }

    pub fn new_next(shape: Shapes) -> Self {
        match shape {
            Shapes::None => {
                // println!("[+] creating new next piece with Shapes::None");
                return Self {
                    shape,
                    positions: [(0xff, 0xff); 4],
                    rotation: 0,
                    num_rotations: 0,
                    pivot: 0xff,
                };
            }
            Shapes::I => {
                return Self {
                    shape,
                    positions: [(0, 0), (0, 1), (0, 2), (0, 3)],
                    rotation: 0,
                    num_rotations: 2,
                    pivot: 2,
                }
            }
            Shapes::O => {
                return Self {
                    shape,
                    positions: [(0, 1), (0, 2), (1, 1), (1, 2)],
                    rotation: 0,
                    num_rotations: 1,
                    pivot: 0xff,
                }
            }
            Shapes::T => {
                return Self {
                    shape,
                    positions: [(0, 1), (0, 2), (0, 3), (1, 2)],
                    rotation: 0,
                    num_rotations: 4,
                    pivot: 1,
                }
            }
            Shapes::J => {
                return Self {
                    shape,
                    positions: [(0, 1), (0, 2), (0, 3), (1, 3)],
                    rotation: 0,
                    num_rotations: 4,
                    pivot: 1,
                }
            }
            Shapes::L => {
                return Self {
                    shape,
                    positions: [(0, 1), (0, 2), (0, 3), (1, 1)],
                    rotation: 0,
                    num_rotations: 4,
                    pivot: 1,
                }
            }
            Shapes::S => {
                return Self {
                    shape,
                    positions: [(0, 2), (0, 3), (1, 1), (1, 2)],
                    rotation: 0,
                    num_rotations: 2,
                    pivot: 0,
                }
            }
            Shapes::Z => {
                return Self {
                    shape,
                    positions: [(0, 1), (0, 2), (1, 2), (1, 3)],
                    rotation: 0,
                    num_rotations: 2,
                    pivot: 1,
                }
            }
        }
    }

    pub fn spawn_pos(&self, spawn_column: u8) -> [(u8, u8); 4] {
        match self.shape {
            Shapes::None => {
                println!("[!] tried to spawn a piece with shape type Shapes::None");
                return [(0xff, 0xff); 4];
            }
            Shapes::I => {
                return [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 2), // [-][-][-][-] | [-][-][0][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-] | [-][-][1][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),     // [0][1][2][3] | [-][-][2][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][-][-][-] | [-][-][3][-]
                ];
            }
            Shapes::O => {
                return [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),     // [-][-][-][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][0][1][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][2][3][-]
                ];
            }
            Shapes::T => {
                return [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][-][-][-] | [-][-][0][-] | [-][-][3][-] | [-][-][2][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][0][1][2] | [-][3][1][-] | [-][2][1][0] | [-][-][1][3]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][-][3][-] | [-][-][2][-] | [-][-][-][-] | [-][-][0][-]
                ];
            }
            Shapes::J => {
                return [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][-][-][-] | [-][-][0][-] | [-][3][-][-] | [-][-][2][3]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][0][1][2] | [-][-][1][-] | [-][2][1][0] | [-][-][1][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][-][-][3] | [-][3][2][-] | [-][-][-][-] | [-][-][0][-]
                ];
            }
            Shapes::L => {
                return [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][-][-][-] | [-][3][0][-] | [-][-][-][3] | [-][-][2][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][0][1][2] | [-][-][1][-] | [-][2][1][0] | [-][-][1][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][3][-][-] | [-][-][2][-] | [-][-][-][-] | [-][-][0][3]
                ];
            }
            Shapes::S => {
                return [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][-][-][-] | [-][-][-][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][-][-][-] | [-][-][1][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][0][1] | [-][-][0][3]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][2][3][-] | [-][-][-][2]
                ];
            }
            Shapes::Z => {
                return [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-] | [-][-][-][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),     // [-][-][-][-] | [-][-][-][3]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][0][1][-] | [-][-][1][2]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][-][2][3] | [-][-][0][-]
                ];
            }
        }
    }

    // returns the position based on the given Movement type
    pub fn piece_pos(&self, r#move: Movement) -> [(u8, u8); 4] {
        // for movements and rotations, we don't have to worry about integer underflow because we will assume the board width is nowhere close to 0xff
        if r#move == Movement::None {
            return self.positions;
        } else if r#move == Movement::Left {
            return [
                (self.positions[0].0, self.positions[0].1 - 1),
                (self.positions[1].0, self.positions[1].1 - 1),
                (self.positions[2].0, self.positions[2].1 - 1),
                (self.positions[3].0, self.positions[3].1 - 1),
            ];
        } else if r#move == Movement::Right {
            return [
                (self.positions[0].0, self.positions[0].1 + 1),
                (self.positions[1].0, self.positions[1].1 + 1),
                (self.positions[2].0, self.positions[2].1 + 1),
                (self.positions[3].0, self.positions[3].1 + 1),
            ];
        } else if r#move == Movement::Down {
            return [
                (self.positions[0].0 + 1, self.positions[0].1),
                (self.positions[1].0 + 1, self.positions[1].1),
                (self.positions[2].0 + 1, self.positions[2].1),
                (self.positions[3].0 + 1, self.positions[3].1),
            ];
        // } else if r#move == Movement::Up {
        //     return [
        //         (self.positions[0].0 - 1, self.positions[0].1),
        //         (self.positions[1].0 - 1, self.positions[1].1),
        //         (self.positions[2].0 - 1, self.positions[2].1),
        //         (self.positions[3].0 - 1, self.positions[3].1),
        //     ];
        } else {
            // T, L, J
            if self.num_rotations == 4 {
                if r#move == Movement::RotateCw {
                    return self.rotate(true);
                } else {
                    return self.rotate(false);
                }
            // I, S, Z
            } else if self.num_rotations == 2 {
                // the I piece is special in that it starts with rotation: 1 so that it lines up with S and Z
                if self.rotation == 0 {
                    return self.rotate(false);
                } else {
                    return self.rotate(true);
                }
            } else if self.num_rotations == 1 {
                return self.positions;
            } else {
                println!(
                    "[!] tried to rotate piece with num_rotations: {}",
                    self.num_rotations
                );
                return self.positions;
            }
        }
    }

    fn rotate(&self, clockwise_flag: bool) -> [(u8, u8); 4] {
        if self.pivot > 3 {
            println!("[!] tried to rotate piece with pivot fields {}", self.pivot);
            return self.positions;
        }
        if clockwise_flag {
            return [
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
        } else {
            return [
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
            ];
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
