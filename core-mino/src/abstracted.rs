use crate::game::board;
use crate::game::board::BoardDim;
use crate::game::player::PlayerIdx;
use crate::game::tile::Tile;

pub type KeyCode = u64;

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

pub struct TextArray {
    pub strings: Vec<String>,
    pub placement: (PlacementX, PlacementY),
    pub size: Size,
    pub active: bool,
    pub needs_redraw: bool,
}

impl TextArray {
    pub fn new(
        strings: Vec<String>,
        placement: &(PlacementX, PlacementY),
        size: Size,
        active: bool,
    ) -> Self {
        Self {
            strings,
            placement,
            size,
            active,
            needs_redraw: active,
        }
    }

    pub fn change_string(&mut self, index: usize, text: String) {
        self.strings[index] = strings;
        self.needs_redraw = true;
    }

    pub fn add_string(&mut self, text: String) {
        self.strings.push(text);
        self.needs_redraw = true;
    }

    pub fn set_active_state(&mut self, active: bool) {
        if self.active != active {
            self.needs_redraw = active;
            self.active = active;
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

impl From<board::BoardClassic> for DrawBoard {
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
