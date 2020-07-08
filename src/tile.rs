use ggez::{Context, graphics};

pub const NUM_PIXEL_ROWS_PER_TILEGRAPHIC: u16 = 8;
const GRAY: (u8, u8, u8) = (150u8, 150u8, 150u8);
const DARK_GRAY: (u8, u8, u8) = (84u8, 84u8, 84u8);

#[derive(Clone)]
pub struct Tile {
    empty: bool,
    active: bool,
    player: u8,
}

impl Tile {
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
            active: active,
            player: player,
        }
    }

    pub fn modify_empty(&mut self) -> Self {
        Self {
            empty: false,
            active: false,
            player: self.player,
        }
    }
}

pub struct TileGraphic {
    pub 
    image: graphics::Image,
}

impl TileGraphic {
    pub fn new_empty(ctx: &mut Context) -> Self {
        // create a pixel buffer big enough to hold 4 u8's for each pixel because rgba
        let mut pixel_buf: [u8; 4 * (NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize) * (NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize)] = [0u8; 4 * (NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize) * (NUM_PIXEL_ROWS_PER_TILEGRAPHIC as usize)];
        for row_index in 0..NUM_PIXEL_ROWS_PER_TILEGRAPHIC {
            for col_index in 0..NUM_PIXEL_ROWS_PER_TILEGRAPHIC {
                if row_index == 0 || row_index == NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1 || col_index == 0 || col_index == NUM_PIXEL_ROWS_PER_TILEGRAPHIC - 1 {
                    pixel_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC * 4 + col_index * 4 + 0) as usize] = DARK_GRAY.0;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC * 4 + col_index * 4 + 1) as usize] = DARK_GRAY.1;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC * 4 + col_index * 4 + 2) as usize] = DARK_GRAY.2;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC * 4 + col_index * 4 + 3) as usize] = 0xff;
                } else {
                    pixel_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC * 4 + col_index * 4 + 0) as usize] = GRAY.0;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC * 4 + col_index * 4 + 1) as usize] = GRAY.1;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC * 4 + col_index * 4 + 2) as usize] = GRAY.2;
                    pixel_buf[(row_index * NUM_PIXEL_ROWS_PER_TILEGRAPHIC * 4 + col_index * 4 + 3) as usize] = 0xff;
                }
            }
        }
        Self{
            image: graphics::Image::from_rgba8(ctx, NUM_PIXEL_ROWS_PER_TILEGRAPHIC, NUM_PIXEL_ROWS_PER_TILEGRAPHIC, &pixel_buf).expect("Failed to create background tile image"),
        }
    }

    pub fn get_size(ctx: &mut Context, board_width: u8, board_height: u8) -> f32 {
        std::cmp::min(graphics::size(ctx).1 as u32 / board_height as u32, graphics::size(ctx).0 as u32 / board_width as u32) as f32
    }

    pub fn _print_image_buf(self, ctx: &mut Context) {
        let image_buf: Vec<u8> = self.image.to_rgba8(ctx).expect("Failed to create image buffer");
        for index in 0..image_buf.len() {
            if index % 4 == 0 {
                if index % 32 == 0 {
                    print!("\n");
                } else {
                    if index != 0 {
                        print!(" ");
                    }
                }
            }
            print!("{:02x}", image_buf[index]);
        }
        print!("\n");
    }
}