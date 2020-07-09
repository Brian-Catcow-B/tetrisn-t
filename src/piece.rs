use crate::board::BOARD_HEIGHT_BUFFER_U;

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum Shapes {
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
}

pub struct Piece {
    shape: Shapes,
    positions: [(u8, u8); 4],
    rotation: u8, // 0, 1, 2, 3: 0, 90, 180, 270; CW
    player: u8,
}

impl Piece {
    pub fn new(shape: Shapes, player: u8) -> Self {
        Self {
            shape: shape,
            positions: [(0xff, 0xff); 4],
            rotation: 0,
            player: player,
        }
    }

    pub fn spawn(&mut self, spawn_column: u8) {
        match self.shape {
            Shapes::I => {
                self.positions = [
                    (spawn_column - 2, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][3][#]
                    (spawn_column - 1, 0 + BOARD_HEIGHT_BUFFER_U), // [0][1][2][3] | [#][#][2][#]
                    (spawn_column + 0, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][1][#]
                    (spawn_column + 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][0][#]
                ]
            },
            Shapes::O => {
                self.positions = [
                    (spawn_column - 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#]
                    (spawn_column - 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][0][1][#]
                    (spawn_column + 0, 1 + BOARD_HEIGHT_BUFFER_U), // [#][2][3][#]
                    (spawn_column + 0, 1 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#]
                ]
            },
            Shapes::T => {
                self.positions = [
                    (spawn_column - 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][0][#] | [#][#][3][#] | [#][#][2][#]
                    (spawn_column + 0, 0 + BOARD_HEIGHT_BUFFER_U), // [#][0][1][2] | [#][3][1][#] | [#][2][1][0] | [#][#][1][3]
                    (spawn_column + 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][3][#] | [#][#][2][#] | [#][#][#][#] | [#][#][0][#]
                    (spawn_column + 0, 1 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][#][#] | [#][#][#][#] | [#][#][#][#]
                ]
            },
            Shapes::J => {
                self.positions = [
                    (spawn_column - 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][0][#] | [#][3][#][#] | [#][#][2][3]
                    (spawn_column + 0, 0 + BOARD_HEIGHT_BUFFER_U), // [#][0][1][2] | [#][#][1][#] | [#][2][1][0] | [#][#][1][#]
                    (spawn_column + 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][3] | [#][3][2][#] | [#][#][#][#] | [#][#][0][#]
                    (spawn_column + 1, 1 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][#][#] | [#][#][#][#] | [#][#][#][#]
                ]
            },
            Shapes::L => {
                self.positions = [
                    (spawn_column - 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][3][0][#] | [#][#][#][3] | [#][#][2][#]
                    (spawn_column + 0, 0 + BOARD_HEIGHT_BUFFER_U), // [#][0][1][2] | [#][#][1][#] | [#][2][1][0] | [#][#][1][#]
                    (spawn_column + 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][3][#][#] | [#][#][2][#] | [#][#][#][#] | [#][#][0][3]
                    (spawn_column - 1, 1 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][#][#] | [#][#][#][#] | [#][#][#][#]
                ]
            },
            Shapes::S => {
                self.positions = [
                    (spawn_column + 0, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][1][#]
                    (spawn_column + 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][0][1] | [#][#][0][3]
                    (spawn_column - 1, 1 + BOARD_HEIGHT_BUFFER_U), // [#][2][3][#] | [#][#][#][2]
                    (spawn_column + 0, 1 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][#][#]
                ]
            },
            Shapes::Z => {
                self.positions = [
                    (spawn_column - 1, 0 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][#][3]
                    (spawn_column + 0, 0 + BOARD_HEIGHT_BUFFER_U), // [#][0][1][#] | [#][#][1][2]
                    (spawn_column + 0, 1 + BOARD_HEIGHT_BUFFER_U), // [#][#][2][3] | [#][#][0][#]
                    (spawn_column + 1, 1 + BOARD_HEIGHT_BUFFER_U), // [#][#][#][#] | [#][#][#][#]
                ]
            },
        }
    }

    pub fn tile_pos(&mut self, r#move: Movement) -> [(u8, u8); 4] {
        // for movements and rotations, we don't have to worry about integer underflow because we will assume the board width is nowhere close to 0xff
        if r#move == Movement::None {
            return self.positions;
        } else if r#move == Movement::Left {
            return [
                (self.positions[0].0 - 1, self.positions[0].1),
                (self.positions[1].0 - 1, self.positions[1].1),
                (self.positions[2].0 - 1, self.positions[2].1),
                (self.positions[3].0 - 1, self.positions[3].1),
            ];
        } else if r#move == Movement::Right {
            return [
                (self.positions[0].0 + 1, self.positions[0].1),
                (self.positions[1].0 + 1, self.positions[1].1),
                (self.positions[2].0 + 1, self.positions[2].1),
                (self.positions[3].0 + 1, self.positions[3].1),
            ];
        } else if r#move == Movement::Down {
            return [
                (self.positions[0].0, self.positions[0].1 + 1),
                (self.positions[1].0, self.positions[1].1 + 1),
                (self.positions[2].0, self.positions[2].1 + 1),
                (self.positions[3].0, self.positions[3].1 + 1),
            ];
        } else {
            match self.shape {
                Shapes::O => return self.positions,
                Shapes::I => {
                    if self.rotation == 0 {
                        self.rotation = 1;
                        return [
                            (self.positions[0].0 + 2, self.positions[0].1 - 2),
                            (self.positions[1].0 + 1, self.positions[1].1 - 1),
                            self.positions[2],
                            (self.positions[3].0 - 1, self.positions[3].1 + 1),
                        ];
                    } else {
                        self.rotation = 0;
                        return [
                            (self.positions[0].0 - 2, self.positions[0].1 + 2),
                            (self.positions[1].0 - 1, self.positions[1].1 + 1),
                            self.positions[2],
                            (self.positions[3].0 + 1, self.positions[3].1 - 1),
                        ];
                    }
                },
                Shapes::S => {
                    if self.rotation == 0 {
                        self.rotation = 1;
                        return [
                            self.positions[0],
                            (self.positions[1].0 - 1, self.positions[1].1 - 1),
                            (self.positions[2].0 + 2, self.positions[2].1 + 0),
                            (self.positions[3].0 + 1, self.positions[3].1 - 1),
                        ];
                    } else {
                        self.rotation = 0;
                        return [
                            self.positions[0],
                            (self.positions[1].0 + 1, self.positions[1].1 + 1),
                            (self.positions[2].0 - 2, self.positions[2].1 - 0),
                            (self.positions[3].0 - 1, self.positions[3].1 + 1),
                        ];
                    }
                }
                Shapes::Z => {
                    if self.rotation == 0 {
                        self.rotation = 1;
                        return [
                            (self.positions[0].0 + 1, self.positions[0].1 + 1),
                            self.positions[1],
                            (self.positions[2].0 + 1, self.positions[2].1 - 1),
                            (self.positions[3].0 - 2, self.positions[3].1 + 0),
                        ];
                    } else {
                        self.rotation = 0;
                        return [
                            (self.positions[0].0 - 1, self.positions[0].1 - 1),
                            self.positions[1],
                            (self.positions[2].0 - 1, self.positions[2].1 + 1),
                            (self.positions[3].0 + 2, self.positions[3].1 + 0),
                        ];
                    }
                }
                _ => {
                    let pivot = self.positions[1];
                    if r#move == Movement::RotateCw {
                        self.rotation = (self.rotation + 1) % 4;
                        // some dark magic (symmetry)
                        return [
                            (pivot.0 + (pivot.1 - self.positions[0].1), pivot.1 + (pivot.0 - self.positions[0].0)),
                            pivot,
                            (pivot.0 + (pivot.1 - self.positions[2].1), pivot.1 + (pivot.0 - self.positions[2].0)),
                            (pivot.0 + (pivot.1 - self.positions[3].1), pivot.1 + (pivot.0 - self.positions[3].0)),
                        ];
                    } else {
                        self.rotation = (self.rotation + 3) % 4;
                        // some of that same dark magic (still just symmetry)
                        return [
                            (pivot.0 + (self.positions[0].1 - pivot.1), pivot.1 + (self.positions[0].0 - pivot.0)),
                            pivot,
                            (pivot.0 + (self.positions[2].1 - pivot.1), pivot.1 + (self.positions[2].0 - pivot.0)),
                            (pivot.0 + (self.positions[3].1 - pivot.1), pivot.1 + (self.positions[3].0 - pivot.0)),
                        ];
                    }
                }
            }
        }
    }
}