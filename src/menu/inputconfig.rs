use ggez::event::KeyCode;
use ggez::graphics::{self, DrawParam, Scale, Text, TextFragment};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::game::GameMode;
use crate::inputs::{Input, KeyboardControlScheme};
use crate::movement::Movement;
use crate::movement::CONVERSION_FAILED_MOVEMENT_FROM_MENUITEMTRIGGER;

use crate::menu::menuhelpers::GAME_MODE_UNEXPECTEDLY_NONE;
use crate::menu::menuhelpers::{MenuGameOptions, MenuItem, MenuItemTrigger, MenuItemValueType};
use crate::menu::menuhelpers::{DARK_GRAY, HELP_RED, LIGHT_GRAY};
use crate::menu::menuhelpers::{SUB_TEXT_SCALE_DOWN, TEXT_SCALE_DOWN};

use std::convert::TryFrom;

const MAX_NON_START_INPUTS_PER_PLAYER: usize = 8;

static KEY_UNEXPECTEDLY_NONE: &str =
    "[!] KeyCode of most recently pressed key is unexpectedly None";

pub struct InputConfigMenu {
    // logic
    selection: usize,
    player_num: u8,
    sub_selection_keyboard: usize,
    sub_selection_keyboard_flag: bool,
    pub most_recently_pressed_key: Option<KeyCode>,
    vec_used_keycode: Vec<KeyCode>,
    keycode_conflict_flag: bool,
    // text
    vec_menu_items_main: Vec<MenuItem>,
    // subtext
    vec_menu_items_keycode: Vec<MenuItem>,
    input_uninitialized_text: Text,
    keycode_conflict_text: Text,
    is_gamepad_text: Text,
}

impl InputConfigMenu {
    pub fn new(game_options: &MenuGameOptions, window_dimensions: (f32, f32)) -> Self {
        let mut vec_used_keycode: Vec<KeyCode> = vec![];
        // gather what the starting used keycodes should be
        for ctrls in game_options.arr_controls.iter() {
            for key_move_pair in (ctrls.0).vec_keycode_movement_pair.iter() {
                vec_used_keycode.push(key_move_pair.0);
            }
        }
        // main MenuItems
        let mut vec_menu_items_main: Vec<MenuItem> = vec![
            MenuItem::new(
                "Back",
                MenuItemValueType::None,
                0,
                None,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
                MenuItemTrigger::Back,
            ),
            MenuItem::new(
                "Player Number: ",
                MenuItemValueType::PlayerNum,
                0,
                None,
                window_dimensions.1,
                TEXT_SCALE_DOWN,
                MenuItemTrigger::SubSelection,
            ),
        ];
        vec_menu_items_main[0].set_select(true);

        // keycode MenuItems
        let mut vec_menu_items_keycode: Vec<MenuItem> =
            Vec::with_capacity(MAX_NON_START_INPUTS_PER_PLAYER);

        match game_options.game_mode {
            GameMode::None => {}
            GameMode::Classic => Self::setup_classic_mode_subtext(
                &mut vec_menu_items_keycode,
                &game_options,
                window_dimensions,
            ),
            GameMode::Rotatris => Self::setup_rotatris_mode_subtext(
                &mut vec_menu_items_keycode,
                &game_options,
                window_dimensions,
            ),
        }
        // Self::setup_rotatris_mode_subtext(&mut vec_menu_items_keycode, game_options, window_dimensions);
        Self {
            selection: 0,
            player_num: 0,
            sub_selection_keyboard: 0,
            sub_selection_keyboard_flag: false,
            most_recently_pressed_key: None,
            vec_used_keycode,
            keycode_conflict_flag: false,
            // text
            vec_menu_items_main,
            // subtext
            vec_menu_items_keycode,
            input_uninitialized_text: Text::new(
                TextFragment::new("No Controls\nKeyboard: Space/Enter\nGamepad: 'G'")
                    .color(HELP_RED)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ),
            keycode_conflict_text: Text::new(
                TextFragment::new("[!] Redundant KeyCode; ignoring")
                    .color(HELP_RED)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ),
            is_gamepad_text: Text::new(
                TextFragment::new("Set to Gamepad\n\n\nSee README for help")
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            ),
        }
    }

    pub fn update_game_mode(
        &mut self,
        window_dimensions: (f32, f32),
        game_options: &mut MenuGameOptions,
    ) {
        self.vec_menu_items_keycode.clear();
        let game_mode = game_options.game_mode;
        game_options.reset_controls();
        self.vec_used_keycode.clear();
        match game_mode {
            GameMode::None => panic!(GAME_MODE_UNEXPECTEDLY_NONE),
            GameMode::Classic => Self::setup_classic_mode_subtext(
                &mut self.vec_menu_items_keycode,
                &game_options,
                window_dimensions,
            ),
            GameMode::Rotatris => Self::setup_rotatris_mode_subtext(
                &mut self.vec_menu_items_keycode,
                &game_options,
                window_dimensions,
            ),
        }
    }

    fn setup_classic_mode_subtext(
        vec_to_add_to: &mut Vec<MenuItem>,
        game_options: &MenuGameOptions,
        window_dimensions: (f32, f32),
    ) {
        Self::setup_left_right_down_subtext(vec_to_add_to, game_options, window_dimensions);
        Self::setup_rotate_piece_subtext(vec_to_add_to, game_options, window_dimensions);
    }

    fn setup_rotatris_mode_subtext(
        vec_to_add_to: &mut Vec<MenuItem>,
        game_options: &MenuGameOptions,
        window_dimensions: (f32, f32),
    ) {
        Self::setup_left_right_down_subtext(vec_to_add_to, game_options, window_dimensions);
        Self::setup_rotate_piece_subtext(vec_to_add_to, game_options, window_dimensions);
        Self::setup_rotate_board_subtext(vec_to_add_to, game_options, window_dimensions);
    }

    fn setup_left_right_down_subtext(
        vec_to_add_to: &mut Vec<MenuItem>,
        game_options: &MenuGameOptions,
        window_dimensions: (f32, f32),
    ) {
        vec_to_add_to.push(MenuItem::new(
            "Left:     ",
            MenuItemValueType::KeyCode,
            0,
            (game_options.arr_controls[0].0).keycode_from_movement(Movement::Left),
            window_dimensions.1,
            SUB_TEXT_SCALE_DOWN,
            MenuItemTrigger::KeyLeft,
        ));
        vec_to_add_to.push(MenuItem::new(
            "Right:    ",
            MenuItemValueType::KeyCode,
            0,
            (game_options.arr_controls[0].0).keycode_from_movement(Movement::Right),
            window_dimensions.1,
            SUB_TEXT_SCALE_DOWN,
            MenuItemTrigger::KeyRight,
        ));
        vec_to_add_to.push(MenuItem::new(
            "Down:     ",
            MenuItemValueType::KeyCode,
            0,
            (game_options.arr_controls[0].0).keycode_from_movement(Movement::Down),
            window_dimensions.1,
            SUB_TEXT_SCALE_DOWN,
            MenuItemTrigger::KeyDown,
        ));
    }

    fn setup_rotate_piece_subtext(
        vec_to_add_to: &mut Vec<MenuItem>,
        game_options: &MenuGameOptions,
        window_dimensions: (f32, f32),
    ) {
        vec_to_add_to.push(MenuItem::new(
            "RotateCw:  ",
            MenuItemValueType::KeyCode,
            0,
            (game_options.arr_controls[0].0).keycode_from_movement(Movement::RotateCw),
            window_dimensions.1,
            SUB_TEXT_SCALE_DOWN,
            MenuItemTrigger::KeyRotateCw,
        ));
        vec_to_add_to.push(MenuItem::new(
            "RotateCcw: ",
            MenuItemValueType::KeyCode,
            0,
            (game_options.arr_controls[0].0).keycode_from_movement(Movement::RotateCcw),
            window_dimensions.1,
            SUB_TEXT_SCALE_DOWN,
            MenuItemTrigger::KeyRotateCcw,
        ));
    }

    fn setup_rotate_board_subtext(
        vec_to_add_to: &mut Vec<MenuItem>,
        game_options: &MenuGameOptions,
        window_dimensions: (f32, f32),
    ) {
        vec_to_add_to.push(MenuItem::new(
            "BoardCw:  ",
            MenuItemValueType::KeyCode,
            0,
            (game_options.arr_controls[0].0).keycode_from_movement(Movement::BoardCw),
            window_dimensions.1,
            SUB_TEXT_SCALE_DOWN,
            MenuItemTrigger::KeyBoardCw,
        ));
        vec_to_add_to.push(MenuItem::new(
            "BoardCcw: ",
            MenuItemValueType::KeyCode,
            0,
            (game_options.arr_controls[0].0).keycode_from_movement(Movement::BoardCcw),
            window_dimensions.1,
            SUB_TEXT_SCALE_DOWN,
            MenuItemTrigger::KeyBoardCcw,
        ));
    }

    pub fn update(&mut self, input: &Input, game_options: &mut MenuGameOptions) -> bool {
        if !self.sub_selection_keyboard_flag {
            // NOT the input box

            if input.keydown_right.1 {
                self.vec_menu_items_main[self.selection].inc_or_dec(true);
                self.player_num = self.get_player_num();
                self.update_all_sub_text_strings(game_options);
            }

            if input.keydown_left.1 {
                self.vec_menu_items_main[self.selection].inc_or_dec(false);
                self.player_num = self.get_player_num();
                self.update_all_sub_text_strings(game_options);
            }

            if input.keydown_down.1 {
                self.vec_menu_items_main[self.selection].set_select(false);
                self.selection = (self.selection + 1) % self.vec_menu_items_main.len();
                self.vec_menu_items_main[self.selection].set_select(true);
                self.most_recently_pressed_key = None;
            }

            if input.keydown_up.1 {
                self.vec_menu_items_main[self.selection].set_select(false);
                self.selection = if self.selection == 0 {
                    self.vec_menu_items_main.len() - 1
                } else {
                    self.selection - 1
                };
                self.vec_menu_items_main[self.selection].set_select(true);
                self.most_recently_pressed_key = None;
            }

            // special case, set player's controls to gamepad ('G' was pressed)
            if input.keydown_rotate_cw.1
                && self.vec_menu_items_main[self.selection].trigger == MenuItemTrigger::SubSelection
            {
                game_options.arr_controls[self.player_num as usize].1 = true;
                self.remove_from_used_keycodes(
                    &game_options.arr_controls[self.player_num as usize].0,
                );
                game_options.arr_controls[self.player_num as usize].0 =
                    KeyboardControlScheme::default();
            }

            // 'Space' or 'Return' was pressed
            if input.keydown_start.1 {
                if self.vec_menu_items_main[self.selection].trigger == MenuItemTrigger::Back {
                    self.sub_selection_keyboard = 0;
                    return true;
                } else if self.vec_menu_items_main[self.selection].trigger
                    == MenuItemTrigger::SubSelection
                {
                    self.most_recently_pressed_key = None;
                    self.remove_from_used_keycodes(
                        &game_options.arr_controls[self.player_num as usize].0,
                    );
                    game_options.arr_controls[self.player_num as usize].0 =
                        KeyboardControlScheme::default();
                    self.update_all_sub_text_strings(game_options);
                    self.sub_selection_keyboard_flag = true;
                    self.vec_menu_items_keycode[self.sub_selection_keyboard].set_select(true);
                }
            }

            // 'Escape' was pressed
            if input.keydown_rotate_ccw.1 {
                if self.vec_menu_items_main[self.selection].trigger == MenuItemTrigger::SubSelection
                {
                    // remove input stuff from selection if we are on the SubSelection option
                    self.remove_from_used_keycodes(
                        &game_options.arr_controls[self.player_num as usize].0,
                    );
                    game_options.arr_controls[self.player_num as usize].0 =
                        KeyboardControlScheme::default();
                    game_options.arr_controls[self.player_num as usize].1 = false;
                    self.most_recently_pressed_key = None;
                } else {
                    return true;
                }
            }
        } else if self.sub_selection_keyboard_flag && self.most_recently_pressed_key.is_some() {
            // input box with some key press

            // first check if the KeyCode is 'Escape', and if it is, just delete the layout entry and go out of the subselection section
            // second check if the KeyCode was already used. If it was, set the error message flag to true
            if input.keydown_rotate_ccw.1 {
                // escape
                self.vec_menu_items_keycode[self.sub_selection_keyboard].set_select(false);
                self.keycode_conflict_flag = false;
                self.sub_selection_keyboard = 0;
                self.sub_selection_keyboard_flag = false;
                // the user was in the middle of creating keyboard controls when they hit 'Escape', so pop however many KeyCode's off vec_used_keycode that the user set up
                for _ in 0..(game_options.arr_controls[self.player_num as usize].0).len() {
                    self.vec_used_keycode.pop();
                }
                game_options.arr_controls[self.player_num as usize].0 =
                    KeyboardControlScheme::default();
            } else if self
                .vec_used_keycode
                .contains(&self.most_recently_pressed_key.expect(KEY_UNEXPECTEDLY_NONE))
            {
                // user tried to press a key that is currently assigned
                self.keycode_conflict_flag = true;
            } else {
                // no conflict, enter KeyCode of key pressed
                self.keycode_conflict_flag = false;
                let key: KeyCode = self.most_recently_pressed_key.expect(KEY_UNEXPECTEDLY_NONE);
                (game_options.arr_controls[self.player_num as usize].0).add_pair(
                    key,
                    Movement::try_from(
                        self.vec_menu_items_keycode[self.sub_selection_keyboard].trigger,
                    )
                    .expect(CONVERSION_FAILED_MOVEMENT_FROM_MENUITEMTRIGGER),
                );
                self.vec_menu_items_keycode[self.sub_selection_keyboard].set_keycode(Some(key));
                self.vec_used_keycode.push(key);
                self.vec_menu_items_keycode[self.sub_selection_keyboard].set_select(false);
                if self.sub_selection_keyboard < self.vec_menu_items_keycode.len() - 1 {
                    self.sub_selection_keyboard += 1;
                    self.vec_menu_items_keycode[self.sub_selection_keyboard].set_select(true);
                } else {
                    self.sub_selection_keyboard = 0;
                    self.sub_selection_keyboard_flag = false;
                }
            }
            self.most_recently_pressed_key = None;
        }
        false
    }

    fn get_player_num(&self) -> u8 {
        for item in self.vec_menu_items_main.iter() {
            if item.value_type == MenuItemValueType::PlayerNum {
                return item.value;
            }
        }
        0u8
    }

    fn remove_from_used_keycodes(&mut self, k_ctrl_scheme: &KeyboardControlScheme) {
        for k_m_pair in k_ctrl_scheme.vec_keycode_movement_pair.iter() {
            let mut items_removed = 0;
            for used_key_idx in 0..self.vec_used_keycode.len() {
                if k_m_pair.0 == self.vec_used_keycode[used_key_idx - items_removed] {
                    self.vec_used_keycode.remove(used_key_idx - items_removed);
                    items_removed += 1;
                    // we only need to get rid of k_ctrl_scheme.len()
                    if items_removed >= k_ctrl_scheme.len() {
                        return;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn update_all_sub_text_strings(&mut self, game_options: &MenuGameOptions) {
        for item in self.vec_menu_items_keycode.iter_mut() {
            let desired_movement = Movement::try_from(item.trigger)
                .expect(CONVERSION_FAILED_MOVEMENT_FROM_MENUITEMTRIGGER);
            item.set_keycode(None);
            for kmp in (game_options.arr_controls[self.player_num as usize].0)
                .vec_keycode_movement_pair
                .iter()
            {
                if kmp.1 == desired_movement {
                    item.set_keycode(Some(kmp.0));
                    break;
                }
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, game_options: &MenuGameOptions) {
        let window_dimensions = graphics::size(ctx);

        // always drawn stuff

        for (index, item) in self.vec_menu_items_main.iter().enumerate() {
            self.draw_text(
                ctx,
                &item.text,
                0.1 + 0.2 * index as f32,
                &window_dimensions,
            );
        }

        // display nothing special on InputConfigMenuOption::Back, so just draw the extra stuff when it's not on InputConfigMenuOption::Back
        // and then later determine which of the other InputConfigMenuOption's it is for the specifics
        if self.vec_menu_items_main[self.selection].trigger != MenuItemTrigger::Back {
            // draw a rectangle containing the subtexts for choosing controls
            // with a color based on whether or not the user is editing controls
            let editing_indicator_rectangle: graphics::Mesh;
            let rect_w = window_dimensions.0 / 2.0;
            let rect_h = window_dimensions.1 / 2.0;
            let rect_x = (window_dimensions.0 - rect_w) / 2.0;
            let rect_y = window_dimensions.1 * 0.4;
            if !self.sub_selection_keyboard_flag {
                editing_indicator_rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect {
                        x: rect_x,
                        y: rect_y,
                        w: rect_w,
                        h: rect_h,
                    },
                    DARK_GRAY,
                )
                .unwrap();
            } else {
                editing_indicator_rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect {
                        x: rect_x,
                        y: rect_y,
                        w: rect_w,
                        h: rect_h,
                    },
                    LIGHT_GRAY,
                )
                .unwrap();
            }
            graphics::draw(ctx, &editing_indicator_rectangle, (Point2::new(0.0, 0.0),)).unwrap();

            if self.vec_menu_items_main[self.selection].trigger == MenuItemTrigger::SubSelection {
                if self.keycode_conflict_flag {
                    self.draw_text(ctx, &self.keycode_conflict_text, 0.43, &window_dimensions);
                }

                if self.sub_selection_keyboard_flag
                    || !(game_options.arr_controls[self.player_num as usize].0).is_empty()
                {
                    for (index, item) in self.vec_menu_items_keycode.iter().enumerate() {
                        self.draw_text(
                            ctx,
                            &item.text,
                            0.5 + 0.05 * index as f32,
                            &window_dimensions,
                        );
                    }
                } else if game_options.arr_controls[self.player_num as usize].1 {
                    self.draw_text(ctx, &self.is_gamepad_text, 0.63, &window_dimensions);
                } else {
                    self.draw_text(ctx, &self.input_uninitialized_text, 0.5, &window_dimensions);
                }
            }
        }
    }

    fn draw_text(
        &self,
        ctx: &mut Context,
        text_var: &Text,
        vertical_position: f32,
        window_dimensions: &(f32, f32),
    ) {
        let (text_width, text_height) = text_var.dimensions(ctx);
        graphics::draw(
            ctx,
            text_var,
            DrawParam::new().dest(Point2::new(
                (window_dimensions.0 - text_width as f32) / 2.0,
                (window_dimensions.1 - text_height as f32) * vertical_position,
            )),
        )
        .unwrap();
    }
}
