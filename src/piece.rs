use crate::board::BOARD_HEIGHT_BUFFER_U;

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum Shapes {
    None,
    I,
    O,
    T,
    J,
    S,
    L,
    Z,
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum Movement {
    None,
    Left,
    Right,
    Down,
    RotateCw,
    RotateCcw,
    // Up, // implemented where Board::clear_line() is called (in main.rs) to fix a weird case; TODO: move the Piece class to be part of the Board class
}

pub struct Piece {
    pub shape: Shapes,
    pub positions: [(u8, u8); 4],
    pub rotation: u8, // 0, 1, 2, 3: 0, 90, 180, 270; CW
    player: u8,
}

impl Piece {
    pub fn new(shape: Shapes, player: u8) -> Self {
        Self {
            shape,
            positions: [(0xff, 0xff); 4],
            rotation: 0,
            player,
        }
    }

    pub fn spawn(&mut self, spawn_column: u8) {
        match self.shape {
            Shapes::None => println!("[!] tried to spawn a piece with shape type Shapes::None"),
            Shapes::I => {
                self.positions = [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 2), // [-][-][-][-] | [-][-][0][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-] | [-][-][1][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),     // [0][1][2][3] | [-][-][2][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][-][-][-] | [-][-][3][-]
                ]
            },
            Shapes::O => {
                self.positions = [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1),     // [-][-][-][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),         // [-][0][1][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][2][3][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column),     // [-][-][-][-]
                ]
            },
            Shapes::T => {
                self.positions = [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-] | [-][-][0][-] | [-][-][3][-] | [-][-][2][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),     // [-][0][1][2] | [-][3][1][-] | [-][2][1][0] | [-][-][1][3]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][-][3][-] | [-][-][2][-] | [-][-][-][-] | [-][-][0][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column), // [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                ]
            },
            Shapes::J => {
                self.positions = [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1),     // [-][-][-][-] | [-][-][0][-] | [-][3][-][-] | [-][-][2][3]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),         // [-][0][1][2] | [-][-][1][-] | [-][2][1][0] | [-][-][1][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1),     // [-][-][-][3] | [-][3][2][-] | [-][-][-][-] | [-][-][0][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                ]
            },
            Shapes::L => {
                self.positions = [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1),     // [-][-][-][-] | [-][3][0][-] | [-][-][-][3] | [-][-][2][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),         // [-][0][1][2] | [-][-][1][-] | [-][2][1][0] | [-][-][1][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1),     // [-][3][-][-] | [-][-][2][-] | [-][-][-][-] | [-][-][0][3]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][-][-][-] | [-][-][-][-] | [-][-][-][-] | [-][-][-][-]
                ]
            },
            Shapes::S => {
                self.positions = [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),         // [-][-][-][-] | [-][-][1][-]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column + 1),     // [-][-][0][1] | [-][-][0][3]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column - 1), // [-][2][3][-] | [-][-][-][2]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column),     // [-][-][-][-] | [-][-][-][-]
                ]
            },
            Shapes::Z => {
                self.positions = [
                    (BOARD_HEIGHT_BUFFER_U, spawn_column - 1),     // [-][-][-][-] | [-][-][-][3]
                    (BOARD_HEIGHT_BUFFER_U, spawn_column),         // [-][0][1][-] | [-][-][1][2]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column),     // [-][-][2][3] | [-][-][0][-]
                    (1 + BOARD_HEIGHT_BUFFER_U, spawn_column + 1), // [-][-][-][-] | [-][-][-][-]
                ]
            },
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
            match self.shape {
                Shapes::O => return self.positions,
                Shapes::I => {
                    if self.rotation % 2 == 0 {
                        return [
                            (self.positions[0].0 - 2, self.positions[0].1 + 2),
                            (self.positions[1].0 - 1, self.positions[1].1 + 1),
                            self.positions[2],
                            (self.positions[3].0 + 1, self.positions[3].1 - 1),
                        ];
                    } else {
                        return [
                            (self.positions[0].0 + 2, self.positions[0].1 - 2),
                            (self.positions[1].0 + 1, self.positions[1].1 - 1),
                            self.positions[2],
                            (self.positions[3].0 - 1, self.positions[3].1 + 1),
                        ];
                    }
                },
                Shapes::S => {
                    if self.rotation % 2 == 0 {
                        return [
                            self.positions[0],
                            (self.positions[1].0 - 1, self.positions[1].1 - 1),
                            (self.positions[2].0 + 2, self.positions[2].1),
                            (self.positions[3].0 + 1, self.positions[3].1 - 1),
                        ];
                    } else {
                        return [
                            self.positions[0],
                            (self.positions[1].0 + 1, self.positions[1].1 + 1),
                            (self.positions[2].0 - 2, self.positions[2].1),
                            (self.positions[3].0 - 1, self.positions[3].1 + 1),
                        ];
                    }
                }
                Shapes::Z => {
                    if self.rotation % 2 == 0 {
                        return [
                            (self.positions[0].0 + 1, self.positions[0].1 + 1),
                            self.positions[1],
                            (self.positions[2].0 + 1, self.positions[2].1 - 1),
                            (self.positions[3].0, self.positions[3].1 - 2),
                        ];
                    } else {
                        return [
                            (self.positions[0].0 - 1, self.positions[0].1 - 1),
                            self.positions[1],
                            (self.positions[2].0 - 1, self.positions[2].1 + 1),
                            (self.positions[3].0, self.positions[3].1 + 2),
                        ];
                    }
                }
                _ => {
                    let pivot = self.positions[1];
                    if r#move == Movement::RotateCw {
                        // some dark magic (symmetry)
                        return [
                            (pivot.0 + (pivot.1 - self.positions[0].1), pivot.1 + (self.positions[0].0 - pivot.0)),
                            pivot,
                            (pivot.0 + (pivot.1 - self.positions[2].1), pivot.1 + (self.positions[2].0 - pivot.0)),
                            (pivot.0 + (pivot.1 - self.positions[3].1), pivot.1 + (self.positions[3].0 - pivot.0)),
                        ];
                    } else {
                        // some of that same dark magic (still just symmetry)
                        return [
                            (pivot.0 + (self.positions[0].1 - pivot.1), pivot.1 + (pivot.0 - self.positions[0].0)),
                            pivot,
                            (pivot.0 + (self.positions[2].1 - pivot.1), pivot.1 + (pivot.0 - self.positions[2].0)),
                            (pivot.0 + (self.positions[3].1 - pivot.1), pivot.1 + (pivot.0 - self.positions[3].0)),
                        ];
                    }
                }
            }
        }
    }
}
