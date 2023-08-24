    // when drawing, we make an unscaled board with the tiles (so the pixel dimensions are 8 * num_tiles_wide by 8 * num_tiles_tall)
    // with the top left of the board at (0, 0), which is the top left corner of the screen;
    // then when we actually draw the board, we scale it to the appropriate size and place the top left corner of the board at the appropriate place;
    // there's a sprite batch for each players' tiles and one more for the empty tiles, which is constant, and the player tiles are drawn after so they are on top
    pub fn draw(&mut self, ctx: &mut Context) {
        // constants used throughout draw
        let height_buffer = self.bh.get_height_buffer();
        let width = self.bh.get_width();
        let height = self.bh.get_height();

        // start doing drawing stuff
        graphics::clear(ctx, graphics::Color::BLACK);
        let (window_width, window_height) = graphics::size(ctx);
        if self.game_over_flag && self.game_over_delay == 0 {
            // DRAW GAME OVER
            self.draw_text(
                ctx,
                &self.game_over_text,
                0.4,
                &(window_width, window_height),
            );
            self.draw_text(
                ctx,
                &self.game_info_text,
                0.55,
                &(window_width, window_height),
            );
        } else if self.pause_flags.0 {
            // DRAW PAUSE
            self.draw_text(ctx, &self.pause_text, 0.4, &(window_width, window_height));
        } else {
            // DRAW GAME

            // ghost tile highlights
            if self.determine_ghost_tile_locations {
                self.batch_highlight_ghost_tile.clear();
                for piece_positions in self.bh.get_ghost_highlight_positions().iter() {
                    for pos in piece_positions.iter().take(4) {
                        let center = width / 2;
                        let is_center_even = (center + 1) % 2;
                        let (y_draw_pos, x_draw_pos) = match self.gravity_direction {
                            // account for the gravity direction in how to draw it (rotatris)
                            Movement::Down => (pos.0, pos.1),
                            Movement::Left => (center * 2 - pos.1 - is_center_even, pos.0),
                            Movement::Up => (
                                center * 2 - pos.0 - is_center_even,
                                center * 2 - pos.1 - is_center_even,
                            ),
                            Movement::Right => (pos.1, center * 2 - pos.0 - is_center_even),
                            _ => unreachable!(
                                "[!] Error: self.gravity_direction is {}",
                                self.gravity_direction as u8
                            ),
                        };
                        self.batch_highlight_ghost_tile
                            .add(graphics::DrawParam::new().dest(Point2::from_slice(&[
                                x_draw_pos as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                (y_draw_pos - height_buffer) as f32
                                    * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                            ])));
                    }
                }
            }

            // add each non-empty tile to the correct SpriteBatch
            for x in 0..width {
                for y in 0..height {
                    // actually go through and add tiles to a spritebatch
                    if !self.bh.get_empty_from_pos(y + height_buffer, x) {
                        // account for the gravity direction in how to draw it (rotatris)
                        let center = width / 2;
                        let is_center_even = (center + 1) % 2;
                        let (y_draw_pos, x_draw_pos) = match self.gravity_direction {
                            Movement::Down => (y, x),
                            Movement::Left => (center * 2 - x - is_center_even, y),
                            Movement::Up => (
                                center * 2 - y - is_center_even,
                                center * 2 - x - is_center_even,
                            ),
                            Movement::Right => (x, center * 2 - y - is_center_even),
                            _ => unreachable!(
                                "[!] Error: self.gravity_direction is {}",
                                self.gravity_direction as u8
                            ),
                        };
                        // create the proper DrawParam and add to the spritebatch
                        let player_tile = graphics::DrawParam::new().dest(Point2::from_slice(&[
                            x_draw_pos as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                            y_draw_pos as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                        ]));
                        if self.num_players > 1 {
                            let player = self.bh.get_player_from_pos(y + height_buffer, x);
                            self.vec_batch_player_piece[player as usize].add(player_tile);
                        } else {
                            let shape: Shapes = self.bh.get_shape_from_pos(y + height_buffer, x);
                            if shape == Shapes::J || shape == Shapes::S {
                                self.vec_batch_player_piece[0].add(player_tile);
                            } else if shape == Shapes::L || shape == Shapes::Z {
                                self.vec_batch_player_piece[1].add(player_tile);
                            } else if shape == Shapes::I || shape == Shapes::O || shape == Shapes::T
                            {
                                self.vec_batch_player_piece[2].add(player_tile);
                            }
                        }
                        // highlight if active
                        if self.bh.get_active_from_pos(y + height_buffer, x) {
                            self.batch_highlight_active_tile.add(player_tile);
                        }
                    }
                }
            }

            // line clear highlights
            if let Some(classic) = &self.bh.classic {
                for full_line in classic.vec_full_lines.iter() {
                    if full_line.lines_cleared_together < 4 {
                        // standard clear animation

                        let y = (full_line.row - height_buffer) as usize;
                        let board_max_index_remainder_2 = (width - 1) % 2;
                        // go from the middle to the outside and reach the end right before full_line.clear_delay reaches 0
                        for x in (width / 2)..width {
                            let highlight_pos_right =
                                graphics::DrawParam::new().dest(Point2::from_slice(&[
                                    x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                ]));
                            let highlight_pos_left =
                                graphics::DrawParam::new().dest(Point2::from_slice(&[
                                    (width as f32 - (x + board_max_index_remainder_2) as f32)
                                        * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                ]));

                            self.batch_highlight_clearing_standard_tile
                                .add(highlight_pos_right);
                            self.batch_highlight_clearing_standard_tile
                                .add(highlight_pos_left);

                            if ((x as f32) / (width as f32) - 0.5) * 2.0
                                > 1.0 - (full_line.clear_delay as f32 / CLEAR_DELAY_CLASSIC as f32)
                            {
                                break;
                            }
                        }
                    } else {
                        // tetrisnt clear animation

                        let y = (full_line.row - height_buffer) as usize;
                        let board_max_index_remainder_2 = (width - 1) % 2;
                        // go from the middle to the outside and reach the end right before full_line.clear_delay reaches 0
                        for x in (width / 2)..width {
                            let highlight_pos_right =
                                graphics::DrawParam::new().dest(Point2::from_slice(&[
                                    x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                ]));
                            let highlight_pos_left =
                                graphics::DrawParam::new().dest(Point2::from_slice(&[
                                    (width as f32 - (x + board_max_index_remainder_2) as f32)
                                        * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                    y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                ]));

                            self.batch_highlight_clearing_tetrisnt_tile
                                .add(highlight_pos_right);
                            self.batch_highlight_clearing_tetrisnt_tile
                                .add(highlight_pos_left);

                            if ((x as f32) / (width as f32) - 0.5) * 2.0
                                > 1.0 - (full_line.clear_delay as f32 / CLEAR_DELAY_CLASSIC as f32)
                            {
                                break;
                            }
                        }
                    }
                }
            }

            // next pieces
            let mut color_number_singleplayer = 2;
            let next_piece = self.vec_next_piece[0].shape;
            if next_piece == Shapes::J || next_piece == Shapes::S {
                color_number_singleplayer = 0;
            } else if next_piece == Shapes::L || next_piece == Shapes::Z {
                color_number_singleplayer = 1;
            }
            for player in &mut self.vec_players {
                if player.redraw_next_piece_flag {
                    // if we need to redraw, clear the next piece sprite batch and rebuild it
                    player.redraw_next_piece_flag = false;
                    if self.num_players > 1 {
                        self.vec_batch_next_piece[player.player_num as usize].clear();
                        for x in 0u8..4u8 {
                            for y in 0u8..2u8 {
                                if self.vec_next_piece[player.player_num as usize].matrix
                                    [y as usize][x as usize]
                                {
                                    let next_tile =
                                        graphics::DrawParam::new().dest(Point2::from_slice(&[
                                            x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                            y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                        ]));
                                    self.vec_batch_next_piece[player.player_num as usize]
                                        .add(next_tile);
                                }
                            }
                        }
                    } else {
                        for x in 0..3 {
                            self.vec_batch_next_piece[x].clear();
                        }
                        for x in 0u8..4u8 {
                            for y in 0u8..2u8 {
                                if self.vec_next_piece[player.player_num as usize].matrix
                                    [y as usize][x as usize]
                                {
                                    let next_tile =
                                        graphics::DrawParam::new().dest(Point2::from_slice(&[
                                            x as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                            y as f32 * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                        ]));
                                    self.vec_batch_next_piece[color_number_singleplayer]
                                        .add(next_tile);
                                }
                            }
                        }
                    }
                }
            }

            let scaled_tile_size = self.tile_size / TILE_SIZE_DOWN_SCALE;

            // draw each SpriteBatch
            let board_top_left_corner = window_width / 2.0
                - (scaled_tile_size * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32 * width as f32 / 2.0);
            // empty tiles
            graphics::draw(
                ctx,
                &self.batch_empty_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // ghost pieces
            graphics::draw(
                ctx,
                &self.batch_highlight_ghost_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // player tiles
            for player in 0..std::cmp::max(self.num_players, 3) {
                graphics::draw(
                    ctx,
                    &self.vec_batch_player_piece[player as usize],
                    DrawParam::new()
                        .dest(Point2::from_slice(&[
                            board_top_left_corner,
                            NON_BOARD_SPACE_U as f32 * self.tile_size,
                        ]))
                        .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
                )
                .unwrap();
            }
            // active tile highlights
            graphics::draw(
                ctx,
                &self.batch_highlight_active_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // clearing tile standard highlights
            graphics::draw(
                ctx,
                &self.batch_highlight_clearing_standard_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // clearing tile tetrisnt highlights
            graphics::draw(
                ctx,
                &self.batch_highlight_clearing_tetrisnt_tile,
                DrawParam::new()
                    .dest(Point2::from_slice(&[
                        board_top_left_corner,
                        NON_BOARD_SPACE_U as f32 * self.tile_size,
                    ]))
                    .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
            )
            .unwrap();
            // next piece tiles
            for player in self.vec_players.iter() {
                if self.num_players > 1 {
                    graphics::draw(
                        ctx,
                        &self.vec_batch_next_piece[player.player_num as usize],
                        DrawParam::new()
                            .dest(Point2::from_slice(&[
                                board_top_left_corner
                                    + (player.spawn_column - 2) as f32
                                        * scaled_tile_size
                                        * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                (NON_BOARD_SPACE_U - BOARD_NEXT_PIECE_SPACING) as f32
                                    * self.tile_size,
                            ]))
                            .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
                    )
                    .unwrap();
                } else {
                    let spawn_column = player.spawn_column;
                    graphics::draw(
                        ctx,
                        &self.vec_batch_next_piece[color_number_singleplayer],
                        DrawParam::new()
                            .dest(Point2::from_slice(&[
                                board_top_left_corner
                                    + (spawn_column - 2) as f32
                                        * scaled_tile_size
                                        * NUM_PIXEL_ROWS_PER_TILEGRAPHIC as f32,
                                (NON_BOARD_SPACE_U - BOARD_NEXT_PIECE_SPACING) as f32
                                    * self.tile_size,
                            ]))
                            .scale(Vector2::from_slice(&[scaled_tile_size, scaled_tile_size])),
                    )
                    .unwrap();
                }
            }
            // score text; TODO: perhaps make a separate function for something based on the bottom,
            // or just figure out how to do this better so we don't divide out by the window_height
            self.draw_text(
                ctx,
                &self.game_info_text,
                1.0 - ((NON_BOARD_SPACE_D as f32 * self.tile_size) / window_height),
                &(window_width, window_height),
            );

            // clear player sprite batches
            for player in 0..std::cmp::max(self.num_players, 3) {
                self.vec_batch_player_piece[player as usize].clear();
            }
            // clear highlight active tile sprite batch
            self.batch_highlight_active_tile.clear();
            // clear highlight clearing tile standard sprite batch
            self.batch_highlight_clearing_standard_tile.clear();
            // clear highlight clearing tile tetrisnt sprite batch
            self.batch_highlight_clearing_tetrisnt_tile.clear();
        }
    }

    fn draw_text(
        &self,
        ctx: &mut Context,
        text_var: &Text,
        vertical_position: f32,
        window_dimensions: &(f32, f32),
    ) {
        let text_var_dimensions = text_var.dimensions(ctx);
        graphics::draw(
            ctx,
            text_var,
            DrawParam::new().dest(Point2::from_slice(&[
                (window_dimensions.0 - text_var_dimensions.w as f32) / 2.0,
                (window_dimensions.1 - text_var_dimensions.h as f32) * vertical_position,
            ])),
        )
        .unwrap();
    }

