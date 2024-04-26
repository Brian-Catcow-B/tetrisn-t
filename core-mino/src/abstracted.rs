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
