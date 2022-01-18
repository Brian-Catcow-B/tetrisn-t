use ggez::{graphics, Context};

use crate::game::board::BoardDim;
use crate::game::Shapes;

pub const NUM_PIXEL_ROWS_PER_TILEGRAPHIC: u16 = 8u16;

const DARK_GRAY: (u8, u8, u8, u8) = (20u8, 20u8, 20u8, 0xffu8);
const GRAY: (u8, u8, u8, u8) = (60u8, 60u8, 60u8, 0xffu8);
const WHITE: (u8, u8, u8, u8) = (255u8, 255u8, 255u8, 0xffu8);

// average with black
const PLAYER_TILE_DARKEN: [f32; 3] = [0.80, 0.70, 0.50];

// [0][1][1][2][2][1][1][0]
// [1][-][-][-][-][-][-][1]
// [1][-][-][-][-][-][-][1]
// [2][-][-][-][-][-][-][2]
// [2][-][-][-][-][-][-][2]
// [1][-][-][-][-][-][-][1]
// [1][-][-][-][-][-][-][1]
// [0][1][1][2][2][1][1][0]

// average with white
const PLAYER_TILE_BRIGHTEN: [f32; 3] = [0.40, 0.25, 0.10];

// [-][-][-][-][-][-][-][-]
// [-][-][-][2][2][-][-][-]
// [-][-][2][1][1][2][-][-]
// [-][2][1][0][0][1][2][-]
// [-][2][1][0][0][1][2][-]
// [-][-][2][1][1][2][-][-]
// [-][-][-][2][2][-][-][-]
// [-][-][-][-][-][-][-][-]

// this one is actually opacity out of 0xff, since it's drawn over the top of a player's active piece's tiles
const PLAYER_TILE_ACTIVE_HIGHLIGHT: [u8; 3] = [0x50, 0x09, 0x00];

// [0][1][1][1][1][1][1][0]
// [1][1][1][2][2][1][1][1]
// [1][1][2][2][2][2][1][1]
// [1][2][2][2][2][2][2][1]
// [1][2][2][2][2][2][2][1]
// [1][1][2][2][2][2][1][1]
// [1][1][1][2][2][1][1][1]
// [0][1][1][1][1][1][1][0]

// this one is actually opacity out of 0xff, since it's drawn over the top of clearing tiles' sprites
const CLEARING_TILE_STANDARD_HIGHLIGHT: [u8; 3] = [0x60, 0x10, 0x05];

// [0][1][1][1][1][1][1][0]
// [1][1][1][2][2][1][1][1]
// [1][1][2][2][2][2][1][1]
// [1][2][2][2][2][2][2][1]
// [1][2][2][2][2][2][2][1]
// [1][1][2][2][2][2][1][1]
// [1][1][1][2][2][1][1][1]
// [0][1][1][1][1][1][1][0]

// this one is actually opacity out of 0xff, since it's drawn over the top of clearing tiles' sprites (when 4 lines are cleared at once)
const CLEARING_TILE_TETRISNT_HIGHLIGHT: [u8; 3] = [0xa0, 0x30, 0x10];

// [0][1][1][1][1][1][1][0]
// [1][1][1][2][2][1][1][1]
// [1][1][2][2][2][2][1][1]
// [1][2][2][2][2][2][2][1]
// [1][2][2][2][2][2][2][1]
// [1][1][2][2][2][2][1][1]
// [1][1][1][2][2][1][1][1]
// [0][1][1][1][1][1][1][0]

// this one is also opacity out of 0xff, since it's drawn over the empty tiles the piece would go to were it to go straight down
const GHOST_TILE_HIGHLIGHT: [u8; 4] = [0x20, 0x04, 0x00, 0x00];

// [0][0][0][0][0][0][0][0]
// [0][1][1][1][1][1][1][0]
// [0][1][2][2][2][2][1][0]
// [0][1][2][3][3][2][1][0]
// [0][1][2][3][3][2][1][0]
// [0][1][2][2][2][2][1][0]
// [0][1][1][1][1][1][1][0]
// [0][0][0][0][0][0][0][0]

// defined player colors, otherwise it uses a generated color using BASE_PLAYER_COLOR based on player number
const NUM_PLAYERCOLORS: u8 = 7;
const PLAYER_RGBA: [(u8, u8, u8, u8); NUM_PLAYERCOLORS as usize] = [
    (69u8, 125u8, 225u8, 0xffu8),
    (240u8, 40u8, 40u8, 0xffu8),
    (80u8, 200u8, 60u8, 0xffu8),
    (230u8, 230u8, 50u8, 0xffu8),
    (220u8, 150u8, 70u8, 0xffu8),
    (125u8, 125u8, 125u8, 0xffu8),
    (230u8, 100u8, 210u8, 0xffu8),
];

const BASE_PLAYER_COLOR: (u8, u8, u8, u8) = (25u8, 80u8, 212u8, 0xffu8);

#[derive(Copy, Clone)]
pub struct Tile {
    pub empty: bool,
    pub active: bool,
    pub player: u8,
    pub shape: Shapes,
}

impl Tile {
    pub fn new(empty: bool, active: bool, player: u8, shape: Shapes) -> Self {
        Self {
            empty,
            active,
            player,
            shape,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            empty: true,
            active: false,
            player: 0xffu8,
            shape: Shapes::None,
        }
    }
}

pub struct TileGraphic {
    pub image: graphics::Image,
}

impl TileGraphic {
    fn pack_color_buf(
        color_buf: &[(u8, u8, u8, u8);
             NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize
                 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize],
    ) -> [u8; NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * 4]
    {
        let mut buf: [u8; NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize
            * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize
            * 4] = [0; NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize
            * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize
            * 4];
        for color_index in
            0..NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize
        {
            buf[4 * color_index] = color_buf[color_index].0;
            buf[4 * color_index + 1] = color_buf[color_index].1;
            buf[4 * color_index + 2] = color_buf[color_index].2;
            buf[4 * color_index + 3] = color_buf[color_index].3;
        }
        buf
    }

    pub fn new_empty(ctx: &mut Context) -> Self {
        // create a buffer of (u8, u8, u8, u8), because rgba, big enough to hold each pixel
        let mut pixel_color_buf: [(u8, u8, u8, u8);
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize] =
            [GRAY;
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize];
        for row_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
            for col_index in 0..NUM_PIXEL_ROWS_PER_TILEGRAPHIC {
                pixel_color_buf
                    [(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = DARK_GRAY;
                pixel_color_buf
                    [(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = DARK_GRAY;
                // flipped for symmetry
            }
        }
        Self {
            image: graphics::Image::from_rgba8(
                ctx,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                &TileGraphic::pack_color_buf(&pixel_color_buf),
            )
            .expect("Failed to create background tile image"),
        }
    }

    pub fn new_player(ctx: &mut Context, player: u8) -> Self {
        let player_color: (u8, u8, u8, u8) = if player < NUM_PLAYERCOLORS {
            PLAYER_RGBA[player as usize]
        } else {
            // procedurally generate colors beyond NUM_PLAYERCOLORS via multiplication by the player number, then avoid u8 overflow
            (
                (((player - NUM_PLAYERCOLORS + 1) as usize * BASE_PLAYER_COLOR.0 as usize) % 0xff)
                    as u8,
                (((player - NUM_PLAYERCOLORS + 1) as usize * BASE_PLAYER_COLOR.1 as usize) % 0xff)
                    as u8,
                (((player - NUM_PLAYERCOLORS + 1) as usize * BASE_PLAYER_COLOR.2 as usize) % 0xff)
                    as u8,
                BASE_PLAYER_COLOR.3,
            )
        };
        // create a buffer of (u8, u8, u8, u8), because rgba, big enough to hold each pixel
        let mut pixel_color_buf: [(u8, u8, u8, u8);
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize] =
            [player_color;
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize];
        // corner
        for row_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
            for col_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf
                    [(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN[0]) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[0]) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[0]) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }
        // two pixels around the corner
        for row_index in &[
            1,
            2,
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2,
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3,
        ] {
            for col_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf
                    [(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN[1]) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[1]) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[1]) * player_color.2 as f32) as u8,
                    0xff,
                );
                // flipped for symmetry
                pixel_color_buf
                    [(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN[1]) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[1]) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[1]) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }
        // the rest across that edge
        for row_index in &[3, 4] {
            for col_index in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf
                    [(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN[2]) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[2]) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[2]) * player_color.2 as f32) as u8,
                    0xff,
                );
                // flipped for symmetry
                pixel_color_buf
                    [(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = (
                    ((1.0 - PLAYER_TILE_DARKEN[2]) * player_color.0 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[2]) * player_color.1 as f32) as u8,
                    ((1.0 - PLAYER_TILE_DARKEN[2]) * player_color.2 as f32) as u8,
                    0xff,
                );
            }
        }

        // center square
        for row_index in &[3, 4] {
            for col_index in &[3, 4] {
                pixel_color_buf
                    [(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    (PLAYER_TILE_BRIGHTEN[0] * WHITE.0 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[0]) * player_color.0 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[0] * WHITE.1 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[0]) * player_color.1 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[0] * WHITE.2 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[0]) * player_color.2 as f32)
                        as u8,
                    0xff,
                );
            }
        }
        // around center square
        for row_index in &[2, 5] {
            for col_index in &[3, 4] {
                pixel_color_buf
                    [(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    (PLAYER_TILE_BRIGHTEN[1] * WHITE.0 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[1]) * player_color.0 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[1] * WHITE.1 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[1]) * player_color.1 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[1] * WHITE.2 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[1]) * player_color.2 as f32)
                        as u8,
                    0xff,
                );
                // flipped for symmetry
                pixel_color_buf
                    [(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = (
                    (PLAYER_TILE_BRIGHTEN[1] * WHITE.0 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[1]) * player_color.0 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[1] * WHITE.1 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[1]) * player_color.1 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[1] * WHITE.2 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[1]) * player_color.2 as f32)
                        as u8,
                    0xff,
                );
            }
        }
        // outer center
        for row_index in &[1, 6] {
            for col_index in &[3, 4] {
                pixel_color_buf
                    [(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.0 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.0 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.1 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.1 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.2 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.2 as f32)
                        as u8,
                    0xff,
                );
                // flipped for symmetry
                pixel_color_buf
                    [(row_index + col_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC) as usize] = (
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.0 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.0 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.1 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.1 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.2 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.2 as f32)
                        as u8,
                    0xff,
                );
            }
        }
        for row_index in &[2, 5] {
            for col_index in &[2, 5] {
                pixel_color_buf
                    [(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC + col_index) as usize] = (
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.0 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.0 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.1 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.1 as f32)
                        as u8,
                    (PLAYER_TILE_BRIGHTEN[2] * WHITE.2 as f32
                        + (1.0 - PLAYER_TILE_BRIGHTEN[2]) * player_color.2 as f32)
                        as u8,
                    0xff,
                );
            }
        }

        Self {
            image: graphics::Image::from_rgba8(
                ctx,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                &TileGraphic::pack_color_buf(&pixel_color_buf),
            )
            .expect("Failed to create player piece tile image"),
        }
    }

    pub fn new_active_highlight(ctx: &mut Context) -> Self {
        // create a buffer of (u8, u8, u8, u8), because rgba, big enough to hold each pixel
        let mut pixel_color_buf: [(u8, u8, u8, u8);
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize] =
            [WHITE;
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize];

        // corners (0)
        for x in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
            for y in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[0];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[0];
            }
        }

        // edges (1)
        for x in 1..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1 {
            for y in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[1];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[1];
            }
        }

        // around the corners (1)
        for x in &[
            1,
            2,
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3,
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2,
        ] {
            for y in &[1, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[1];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[1];
            }
        }

        // inside (2)
        for x in 3..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3 {
            for y in &[1, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[2];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[2];
            }
        }
        for x in 2..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2 {
            for y in 2..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2 {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    PLAYER_TILE_ACTIVE_HIGHLIGHT[2];
            }
        }

        Self {
            image: graphics::Image::from_rgba8(
                ctx,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                &TileGraphic::pack_color_buf(&pixel_color_buf),
            )
            .expect("Failed to create active player tile highlight image"),
        }
    }

    pub fn new_clear_standard_highlight(ctx: &mut Context) -> Self {
        // create a buffer of (u8, u8, u8, u8), because rgba, big enough to hold each pixel
        let mut pixel_color_buf: [(u8, u8, u8, u8);
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize] =
            [WHITE;
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize];

        // corners (0)
        for x in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
            for y in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[0];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[0];
            }
        }

        // edges (1)
        for x in 1..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1 {
            for y in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[1];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[1];
            }
        }

        // around the corners (1)
        for x in &[
            1,
            2,
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3,
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2,
        ] {
            for y in &[1, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[1];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[1];
            }
        }

        // inside (2)
        for x in 3..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3 {
            for y in &[1, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[2];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[2];
            }
        }
        for x in 2..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2 {
            for y in 2..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2 {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_STANDARD_HIGHLIGHT[2];
            }
        }

        Self {
            image: graphics::Image::from_rgba8(
                ctx,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                &TileGraphic::pack_color_buf(&pixel_color_buf),
            )
            .expect("Failed to create tile clearing standard highlight image"),
        }
    }

    pub fn new_clear_tetrisnt_highlight(ctx: &mut Context) -> Self {
        // create a buffer of (u8, u8, u8, u8), because rgba, big enough to hold each pixel
        let mut pixel_color_buf: [(u8, u8, u8, u8);
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize] =
            [WHITE;
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize];

        // corners (0)
        for x in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
            for y in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[0];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[0];
            }
        }

        // edges (1)
        for x in 1..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1 {
            for y in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[1];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[1];
            }
        }

        // around the corners (1)
        for x in &[
            1,
            2,
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3,
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2,
        ] {
            for y in &[1, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[1];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[1];
            }
        }

        // inside (2)
        for x in 3..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3 {
            for y in &[1, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[2];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[2];
            }
        }
        for x in 2..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2 {
            for y in 2..NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2 {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    CLEARING_TILE_TETRISNT_HIGHLIGHT[2];
            }
        }

        Self {
            image: graphics::Image::from_rgba8(
                ctx,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                &TileGraphic::pack_color_buf(&pixel_color_buf),
            )
            .expect("Failed to create tile clearing tetrisnt highlight image"),
        }
    }

    pub fn new_ghost_highlight(ctx: &mut Context) -> Self {
        // create a buffer of (u8, u8, u8, u8), because rgba, big enough to hold each pixel
        let mut pixel_color_buf: [(u8, u8, u8, u8);
            NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize] =
            [WHITE;
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize];

        // outer (0)
        for x in 0..=NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1 {
            for y in &[0, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    GHOST_TILE_HIGHLIGHT[0];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    GHOST_TILE_HIGHLIGHT[0];
            }
        }

        // between (1)
        for x in 1..=NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2 {
            for y in &[1, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 2] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    GHOST_TILE_HIGHLIGHT[1];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    GHOST_TILE_HIGHLIGHT[1];
            }
        }

        // inner (2)
        for x in 2..=NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3 {
            for y in &[2, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 3] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    GHOST_TILE_HIGHLIGHT[2];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    GHOST_TILE_HIGHLIGHT[2];
            }
        }

        // middle (3)
        for x in 3..=NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 4 {
            for y in &[3, NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 4] {
                pixel_color_buf[(x + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * y) as usize].3 =
                    GHOST_TILE_HIGHLIGHT[3];
                pixel_color_buf[(y + NUM_PIXEL_ROWS_PER_TILEGRAPHIC * x) as usize].3 =
                    GHOST_TILE_HIGHLIGHT[3];
            }
        }

        Self {
            image: graphics::Image::from_rgba8(
                ctx,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                NUM_PIXEL_ROWS_PER_TILEGRAPHIC,
                &TileGraphic::pack_color_buf(&pixel_color_buf),
            )
            .expect("Failed to create tile ghost tile highlight image"),
        }
    }

    pub fn get_size(
        window_width: f32,
        window_height: f32,
        board_width: BoardDim,
        board_height: BoardDim,
    ) -> f32 {
        std::cmp::min(
            window_height as u32 / board_height as u32,
            window_width as u32 / board_width as u32,
        ) as f32
    }

    pub fn _print_image_buf(self, ctx: &mut Context) {
        let image_buf: Vec<u8> = self
            .image
            .to_rgba8(ctx)
            .expect("Failed to create image buffer");
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
