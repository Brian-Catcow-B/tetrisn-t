use crate::game::board;
use crate::game::board::BoardDim;
use crate::game::tile::Tile;

pub type KeyCode = u64;
pub type GamepadId = u64;
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Button {
    DPadLeft,
    DPadRight,
    DPadDown,
    East,
    South,
    North,
    West,
    Start,
}
pub type AxisValue = f64;
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    LeftStickX,
    LeftStickY,
}

pub enum PlacementX {
    FlushLeft,
    Left,
    Center,
    Right,
    FlushRight,
}

pub enum PlacementY {
    FlushTop,
    Top,
    Center,
    Bottom,
    FlushBottom,
}

pub enum Size {
    VerySmall,
    Small,
    Normal,
    Large,
    VeryLarge,
}

#[derive(PartialEq, Eq)]
pub enum TextColor {
    White,
    Black,
    Green,
}

#[derive(PartialEq, Eq)]
pub enum LayoutDirection {
    LeftRight,
    UpDown,
}

pub struct TextArray {
    pub strings: Vec<String>,
    pub colors: Vec<TextColor>,
    pub placement: (PlacementX, PlacementY),
    pub size: Size,
    pub active: bool,
    pub needs_redraw: bool,
}

impl TextArray {
    pub fn new(
        strings: Vec<String>,
        colors: Vec<TextColor>,
        placement: (PlacementX, PlacementY),
        size: Size,
        active: bool,
    ) -> Self {
        Self {
            strings,
            placement,
            size,
            color,
            active,
            needs_redraw: active,
        }
    }

    pub fn change_string(&mut self, index: usize, string: String) {
        self.strings[index] = string;
        self.needs_redraw = self.active;
    }

    pub fn add_string(&mut self, string: String, color: TextColor) {
        self.strings.push(string);
        self.colors.push(color);
        self.needs_redraw = self.active;
    }

    pub fn change_color(&mut self, index: usize, color: TextColor) {
        if self.colors[index] != color {
            self.colors[index] = color;
            self.needs_redraw = self.active;
        }
    }

    pub fn set_active_state(&mut self, active: bool) {
        if self.active != active {
            self.active = active;
            self.needs_redraw = true;
        }
    }
}

pub struct DrawBoard {
    pub width: BoardDim,
    pub height: BoardDim,
    pub matrix: Vec<Vec<Tile>>,
    pub needs_redraw: bool,
}

impl Default for DrawBoard {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            matrix: vec![],
            needs_redraw: true,
        }
    }
}

impl From<&board::BoardClassic> for DrawBoard {
    fn from(board: &board::BoardClassic) -> Self {
        let mut draw_board = DrawBoard::default();
        for y in 0..board.height {
            draw_board.matrix.push(vec![]);
            for x in 0..board.width {
                draw_board.matrix[y as usize].push(board.matrix[y as usize][x as usize]);
            }
        }
        draw_board.width = board.width;
        draw_board.height = board.height;
        draw_board
    }
}

impl PartialEq for DrawBoard {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if self.matrix[y as usize][x as usize] != other.matrix[y as usize][x as usize] {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for DrawBoard {}

pub struct DrawContext {
    pub vec_text_array: Vec<TextArray>,
    pub vec_draw_board: Vec<DrawBoard>,
}

impl Default for DrawContext {
    fn default() -> Self {
        Self {
            vec_text_array: vec![],
            vec_draw_board: vec![],
        }
    }
}

impl DrawContext {
    pub fn needs_redraw(&self) -> bool {
        for t in self.vec_text_array.iter() {
            if t.needs_redraw {
                return true;
            }
        }
        for b in self.vec_draw_board.iter() {
            if b.needs_redraw {
                return true;
            }
        }
        false
    }
}
