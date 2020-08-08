use crate::controls::{Input, ControlScheme};
use crate::piece::{Piece, Shapes};

pub const SPAWN_DELAY: i16 = 20i16;

pub struct Player {
    pub player_num: u8,
    pub control_scheme: ControlScheme,
    pub input: Input,
    pub spawn_piece_flag: bool,
    pub spawn_column: u8,
    pub spawn_delay: i16,
    pub next_piece: Piece,
    pub redraw_next_piece_flag: bool,
}

impl Player {
    pub fn new(player_num: u8, control_scheme: ControlScheme, spawn_column: u8) -> Self {
        Self {
            player_num,
            control_scheme,
            input: Input::new(),
            spawn_piece_flag: true,
            spawn_column,
            spawn_delay: SPAWN_DELAY,
            next_piece: Piece::new_next(Shapes::None),
            redraw_next_piece_flag: true,
        }
    }
}