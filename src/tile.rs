use ggez::{Context, graphics};

pub const NUM_PIXEL_ROWS_PER_TILEGRAPHIC: u16 = 8u16;

const BASE_PLAYER_COLOR: (u8, u8, u8, u8) = (69u8, 125u8, 225u8, 0xffu8);

const BLACK: (u8, u8, u8, u8) = (0u8, 0u8, 0u8, 0xffu8);
const DARK_GRAY: (u8, u8, u8, u8) = (60u8, 60u8, 60u8, 0xffu8);
const GRAY: (u8, u8, u8, u8) = (120u8, 120u8, 120u8, 0xffu8);
const WHITE: (u8, u8, u8, u8) = (255u8, 255u8, 255u8, 0xffu8);

const PLAYER_TILE_DARKEN_0: f32 = 0.95;
const PLAYER_TILE_DARKEN_1: f32 = 0.85;
const PLAYER_TILE_DARKEN_2: f32 = 0.70;

// [0][1][1][2][2][1][1][0]
// [1][-][-][-][-][-][-][1]
// [1][-][-][-][-][-][-][1]
// [2][-][-][-][-][-][-][2]
// [2][-][-][-][-][-][-][2]
// [1][-][-][-][-][-][-][1]
// [1][-][-][-][-][-][-][1]
// [0][1][1][2][2][1][1][0]

const PLAYER_TILE_BRIGHTEN_0: f32 = 0.40;
const PLAYER_TILE_BRIGHTEN_1: f32 = 0.25;
const PLAYER_TILE_BRIGHTEN_2: f32 = 0.10;

// [-][-][-][-][-][-][-][-]
// [-][-][-][2][2][-][-][-]
// [-][-][2][1][1][2][-][-]
// [-][2][1][0][0][1][2][-]
// [-][2][1][0][0][1][2][-]
// [-][-][2][1][1][2][-][-]
// [-][-][-][2][2][-][-][-]
// [-][-][-][-][-][-][-][-]


#[derive(Clone)]
pub struct Tile {
    pub empty: bool,
    pub active: bool,
    pub player: u8,
}

impl Tile {
    pub fn new(empty: bool, active: bool, player: u8) -> Self {
        Self {
            empty,
            active,
            player,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            empty: true,
            active: false,
            player: 0xffu8,
        }
    }

    // TODO: figure out how to fill the rest of the `Self` arguments as what they are and optimize modify_fill and modify_empty
    pub fn modify_fill(&mut self, active: bool, player: u8) -> Self {
        Self {
            empty: false,
            active,
            player,
        }
    }

    pub fn modify_empty(&mut self) -> Self {
        Self {
            empty: false,
            active: false,
            player: 0xffu8,
        }
    }
}

pub struct TileGraphic {
    pub 
    image: graphics::Image,
}

impl TileGraphic {
    fn pack_color_buf(color_buf: &[(u8, u8, u8, u8); NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize]) -> [u8; NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * 4] {
        let mut buf: [u8; NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * 4] = [0; NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * 4];
        for color_index in 0..NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize {
            buf[4 * color_index] = color_buf[color_index].0;
            buf[4 * color_index + 1] = color_buf[color_index].1;
            buf[4 * color_index + 2] = color_buf[color_index].2;
            buf[4 * color_index + 3] = color_buf[color_index].3;
        }
        buf
    }

    pub fn new_empty(ctx: &mut Context) -> Self {
        // create a buffer of (u8, u8, u8, u8), because rgba, big enough to hold each pixel
        let mut pixel_color_buf: [(u8, u8, u8, u8); NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize] = [GRAY; NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize];
        for row_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
            for col_index in 0..NUM_PIXEL_ROWS_PER_TILEGRAPHIC {
                pixel_color_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = DARK_GRAY;
                pixel_color_buf[(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = DARK_GRAY; // flipped for symmetry
            }
        }
        Self{
            image: graphics::Image::from_rgba8(ctx, NUM_PIXEL_ROWS_PER_TILEGRAPHIC, NUM_PIXEL_ROWS_PER_TILEGRAPHIC, &TileGraphic::pack_color_buf(&pixel_color_buf)).expect("Failed to create background tile image"),
        }
    }

    pub fn new_player(ctx: &mut Context, player: u8) -> Self {
        let player_color: (u8, u8, u8, u8) = ((player + 1) * BASE_PLAYER_COLOR.0, (player + 1) * BASE_PLAYER_COLOR.1, (player + 1) * BASE_PLAYER_COLOR.2, BASE_PLAYER_COLOR.3);
        // create a buffer of (u8, u8, u8, u8), because rgba, big enough to hold each pixel
        let mut pixel_color_buf: [(u8, u8, u8, u8); NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize] = [player_color; NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize];
        // corner
        for row_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
            for col_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN_0) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_0) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_0) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }
        // two pixels around the corner
        for row_index in &[1, 2, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3] {
            for col_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN_1) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_1) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_1) * player_color.2 as f32) as u8,
                    0xff,
                );
                // flipped for symmetry
                pixel_color_buf[(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN_1) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_1) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_1) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }
        // the rest across that edge
        for row_index in &[3, 4] {
            for col_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN_2) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_2) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_2) * player_color.2 as f32) as u8,
                    0xff,
                );
                // flipped for symmetry
                pixel_color_buf[(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN_2) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_2) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN_2) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }

        // center square
        for row_index in &[3, 4] {
            for col_index in &[3, 4] {
                pixel_color_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    (PLAYER_TILE_BRIGHTEN_0 * WHITE.0 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_0) * player_color.0 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_0 * WHITE.1 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_0) * player_color.1 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_0 * WHITE.2 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_0) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }
        // around center square
        for row_index in &[2, 5] {
            for col_index in &[3, 4] {
                pixel_color_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    (PLAYER_TILE_BRIGHTEN_1 * WHITE.0 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_1) * player_color.0 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_1 * WHITE.1 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_1) * player_color.1 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_1 * WHITE.2 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_1) * player_color.2 as f32) as u8,
                    0xff,
                );
                // flipped for symmetry
                pixel_color_buf[(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = (
                    (PLAYER_TILE_BRIGHTEN_1 * WHITE.0 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_1) * player_color.0 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_1 * WHITE.1 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_1) * player_color.1 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_1 * WHITE.2 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_1) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }
        // outer center
        for row_index in &[1, 6] {
            for col_index in &[3, 4] {
                pixel_color_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.0 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.0 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.1 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.1 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.2 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.2 as f32) as u8,
                    0xff,
                );
                // flipped for symmetry
                pixel_color_buf[(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = (
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.0 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.0 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.1 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.1 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.2 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }
        for row_index in &[2, 5] {
            for col_index in &[2, 5] {
                pixel_color_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.0 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.0 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.1 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.1 as f32) as u8,
                    (PLAYER_TILE_BRIGHTEN_2 * WHITE.2 as f32 + (1.0 - PLAYER_TILE_BRIGHTEN_2) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }

        Self{
            image: graphics::Image::from_rgba8(ctx, NUM_PIXEL_ROWS_PER_TILEGRAPHIC, NUM_PIXEL_ROWS_PER_TILEGRAPHIC, &TileGraphic::pack_color_buf(&pixel_color_buf)).expect("Failed to create background tile image"),
        }
    }

    pub fn get_size(ctx: &mut Context, board_width: u8, board_height: u8) -> f32 {
        std::cmp::min(graphics::size(ctx).1 as u32 / board_height as u32, graphics::size(ctx).0 as u32 / board_width as u32) as f32
    }

    pub fn _print_image_buf(self, ctx: &mut Context) {
        let image_buf: Vec<u8> = self.image.to_rgba8(ctx).expect("Failed to create image buffer");
        for (index, image) in image_buf.iter().enumerate() {
            if index % 4 == 0 {
                if index % 32 == 0 {
                    print!("\n");
                } else if index != 0 {
                    print!(" ");
                }
            }
            print!("{:02x}", image);
        }
        print!("\n");
    }
}
