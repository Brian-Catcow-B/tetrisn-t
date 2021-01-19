use ggez::event::KeyCode;
use ggez::graphics::{self, DrawParam};
use ggez::graphics::{Scale, Text, TextFragment};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::inputs::{Input, KeyboardControlScheme};

use crate::menu::MAX_NUM_PLAYERS;

use crate::menu::SUB_TEXT_SCALE_DOWN;
use crate::menu::TEXT_SCALE_DOWN;

use crate::menu::DARK_GRAY;
use crate::menu::HELP_RED;
use crate::menu::LIGHT_GRAY;
use crate::menu::SELECT_GREEN;

const NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES: u8 = 2;
#[repr(u8)]
enum InputConfigMenuOption {
    Back,
    PlayerInput,
}

// currently `Start` is, for keyboards, always `ESC`, and alternate controls are only for Left/Right/Down for controllers, so don't include that in the number of entries for keyboards
const NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES: u8 = 5;
#[repr(u8)]
enum InputConfigMenuSubOptionKeyboard {
    Left,
    Right,
    Down,
    RotateCw,
    RotateCcw,
}

pub struct InputConfigMenu {
    // logic
    selection: u8,
    player_num: u8,
    sub_selection_keyboard: u8,
    sub_selection_keyboard_flag: bool,
    pub most_recently_pressed_key: Option<KeyCode>,
    vec_used_keycode: Vec<KeyCode>,
    keycode_conflict_flag: bool,
    pub arr_split_controls: [(
        Option<(
            Option<KeyCode>,
            Option<KeyCode>,
            Option<KeyCode>,
            Option<KeyCode>,
            Option<KeyCode>,
        )>,
        bool,
    ); MAX_NUM_PLAYERS as usize],
    // text
    back_text: Text,
    player_num_text: Text,
    // subtext
    input_uninitialized_text: Text,
    keycode_conflict_text: Text,
    is_gamepad_text: Text,
    k_left_text: Text,
    k_right_text: Text,
    k_down_text: Text,
    k_rotate_cw_text: Text,
    k_rotate_ccw_text: Text,
    k_start_text: Text,
}

impl InputConfigMenu {
    pub fn new(
        window_dimensions: (f32, f32),
        last_used_arr_controls: [(Option<KeyboardControlScheme>, bool); MAX_NUM_PLAYERS as usize],
    ) -> Self {
        let mut vec_used_keycode: Vec<KeyCode> = vec![];
        let mut arr_split_controls: [(
            Option<(
                Option<KeyCode>,
                Option<KeyCode>,
                Option<KeyCode>,
                Option<KeyCode>,
                Option<KeyCode>,
            )>,
            bool,
        ); MAX_NUM_PLAYERS as usize] = [(None, false); MAX_NUM_PLAYERS as usize];
        for (idx, ctrls) in last_used_arr_controls.iter().enumerate() {
            if let Some(k_ctrls) = ctrls.0 {
                arr_split_controls[idx].0 = Some(k_ctrls.split());
                vec_used_keycode.push(k_ctrls.left);
                vec_used_keycode.push(k_ctrls.right);
                vec_used_keycode.push(k_ctrls.down);
                vec_used_keycode.push(k_ctrls.rotate_cw);
                vec_used_keycode.push(k_ctrls.rotate_ccw);
            }
            arr_split_controls[idx].1 = ctrls.1;
        }
        let mut player_num_text = Text::new(
            TextFragment::new("Player Number: ")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
        );
        player_num_text.add(
            TextFragment::new(" 1")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
        );
        let mut k_left_text = Text::new(
            TextFragment::new("Left:     ")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
        );
        let mut k_right_text = Text::new(
            TextFragment::new("Right:    ")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
        );
        let mut k_down_text = Text::new(
            TextFragment::new("Down:     ")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
        );
        let mut k_rotate_cw_text = Text::new(
            TextFragment::new("RotateCw:  ")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
        );
        let mut k_rotate_ccw_text = Text::new(
            TextFragment::new("RotateCcw:  ")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
        );
        let k_start_text = Text::new(
            TextFragment::new("Start/Pause: Esc")
                .color(graphics::BLACK)
                .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
        );
        if let Some(ctrls) = last_used_arr_controls[0].0 {
            k_left_text.add(
                TextFragment::new(format!("{:?}", ctrls.left))
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
            k_right_text.add(
                TextFragment::new(format!("{:?}", ctrls.right))
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
            k_down_text.add(
                TextFragment::new(format!("{:?}", ctrls.down))
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
            k_rotate_cw_text.add(
                TextFragment::new(format!("{:?}", ctrls.rotate_cw))
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
            k_rotate_ccw_text.add(
                TextFragment::new(format!("{:?}", ctrls.rotate_ccw))
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
        } else {
            k_left_text.add(
                TextFragment::new("None")
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
            k_right_text.add(
                TextFragment::new("None")
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
            k_down_text.add(
                TextFragment::new("None")
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
            k_rotate_cw_text.add(
                TextFragment::new("None")
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
            k_rotate_ccw_text.add(
                TextFragment::new("None")
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)),
            );
        }
        Self {
            selection: 0,
            player_num: 0,
            sub_selection_keyboard: 0,
            sub_selection_keyboard_flag: false,
            most_recently_pressed_key: None,
            vec_used_keycode,
            keycode_conflict_flag: false,
            arr_split_controls,
            // text
            back_text: Text::new(
                TextFragment::new("Back")
                    .color(SELECT_GREEN)
                    .scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)),
            ),
            player_num_text,
            // subtext
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
            k_left_text,
            k_right_text,
            k_down_text,
            k_rotate_cw_text,
            k_rotate_ccw_text,
            k_start_text,
        }
    }

    pub fn update(&mut self, input: &Input) -> bool {
        if input.keydown_right.1 && !self.sub_selection_keyboard_flag {
            self.inc_or_dec_selection(true);
        }

        if input.keydown_left.1 && !self.sub_selection_keyboard_flag {
            self.inc_or_dec_selection(false);
        }

        if !self.sub_selection_keyboard_flag {
            if input.keydown_down.1 {
                self.set_select(false);
                self.selection = (self.selection + 1) % NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES;
                self.set_select(true);
                self.most_recently_pressed_key = None;
            }

            if input.keydown_up.1 {
                self.set_select(false);
                self.selection = if self.selection == 0 {
                    NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES - 1
                } else {
                    self.selection - 1
                };
                self.set_select(true);
                self.most_recently_pressed_key = None;
            }

            if input.keydown_rotate_cw.1
                && self.selection == InputConfigMenuOption::PlayerInput as u8
            {
                self.arr_split_controls[self.player_num as usize].1 = true;
                if let Some(ctrls) = self.arr_split_controls[self.player_num as usize].0 {
                    self.remove_from_used_keycodes(&ctrls);
                    self.arr_split_controls[self.player_num as usize].0 = None;
                }
            }

            if input.keydown_rotate_ccw.1 {
                return true;
            }

            if input.keydown_start.1 {
                if self.selection == InputConfigMenuOption::Back as u8 {
                    self.sub_selection_keyboard = 0;
                    return true;
                } else if self.selection == InputConfigMenuOption::PlayerInput as u8 {
                    self.most_recently_pressed_key = None;
                    if let Some(ctrls) = self.arr_split_controls[self.player_num as usize].0 {
                        self.remove_from_used_keycodes(&ctrls);
                    }
                    self.arr_split_controls[self.player_num as usize].0 = None;

                    self.arr_split_controls[self.player_num as usize].0 =
                        Some((None, None, None, None, None));
                    self.update_sub_text_strings();
                    self.sub_selection_keyboard_flag = true;
                    self.set_select(true);
                }
            }

            // remove input stuff from selection if we are on PlayerInput and Escape is pressed
            if self.selection == InputConfigMenuOption::PlayerInput as u8
                && input.keydown_rotate_ccw.1
            {
                if let Some(ctrls) = self.arr_split_controls[self.player_num as usize].0 {
                    self.remove_from_used_keycodes(&ctrls);
                    self.arr_split_controls[self.player_num as usize].0 = None;
                }
                self.arr_split_controls[self.player_num as usize].1 = false;
                self.most_recently_pressed_key = None;
            }
        } else if self.sub_selection_keyboard_flag {
            if self.most_recently_pressed_key.is_some() {
                // first check if the KeyCode is Escape, and if it is, just delete the layout entry and go out of the subselection section
                // second check if the KeyCode was already used. If it was, set the error message flag to true
                if input.keydown_rotate_ccw.1 {
                    self.set_select(false);
                    self.keycode_conflict_flag = false;
                    self.sub_selection_keyboard = 0;
                    self.sub_selection_keyboard_flag = false;
                    // the user was in the middle of creating keyboard controls when they hit Escape, so pop however many KeyCode's off vec_used_keycode as the user set up
                    if let Some(ctrls) = self.arr_split_controls[self.player_num as usize].0 {
                        if (ctrls.3).is_some() {
                            for _ in 1..=4 {
                                self.vec_used_keycode.pop();
                            }
                        } else if (ctrls.2).is_some() {
                            for _ in 1..=3 {
                                self.vec_used_keycode.pop();
                            }
                        } else if (ctrls.1).is_some() {
                            for _ in 1..=2 {
                                self.vec_used_keycode.pop();
                            }
                        } else if (ctrls.0).is_some() {
                            self.vec_used_keycode.pop();
                        }
                    }
                    self.arr_split_controls[self.player_num as usize].0 = None;
                    self.most_recently_pressed_key = None;
                } else if self.vec_used_keycode.contains(
                    &self
                        .most_recently_pressed_key
                        .expect("[!] KeyCode of most recently pressed key is unexpectedly None"),
                ) {
                    self.keycode_conflict_flag = true;
                } else {
                    self.keycode_conflict_flag = false;
                    match (self.arr_split_controls[self.player_num as usize].0).as_mut() {
                        Some(mut ctrls) => {
                            match self.sub_selection_keyboard {
                                x if x == InputConfigMenuSubOptionKeyboard::Left as u8 => {
                                    ctrls.0 = self.most_recently_pressed_key;
                                    self.vec_used_keycode.push(self.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                x if x == InputConfigMenuSubOptionKeyboard::Right as u8 => {
                                    ctrls.1 = self.most_recently_pressed_key;
                                    self.vec_used_keycode.push(self.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                x if x == InputConfigMenuSubOptionKeyboard::Down as u8 => {
                                    ctrls.2 = self.most_recently_pressed_key;
                                    self.vec_used_keycode.push(self.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                x if x == InputConfigMenuSubOptionKeyboard::RotateCw as u8 => {
                                    ctrls.3 = self.most_recently_pressed_key;
                                    self.vec_used_keycode.push(self.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                x if x == InputConfigMenuSubOptionKeyboard::RotateCcw as u8 => {
                                    ctrls.4 = self.most_recently_pressed_key;
                                    self.vec_used_keycode.push(self.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None"));
                                },
                                _ => println!("[!] couldn't get correct tuple index to set most recently pressed key"),
                            }
                        },
                        None => {
                            println!("[!] arr_split_controls[{}].0 was unexpectedly None", self.player_num);
                        }
                    }
                    self.set_select(false);
                    if self.sub_selection_keyboard
                        < NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES as u8 - 1
                    {
                        self.sub_selection_keyboard += 1;
                        self.set_select(true);
                    } else {
                        self.sub_selection_keyboard = 0;
                        self.sub_selection_keyboard_flag = false;
                    }
                }
            }
        }
        false
    }

    fn remove_from_used_keycodes(
        &mut self,
        ctrls: &(
            Option<KeyCode>,
            Option<KeyCode>,
            Option<KeyCode>,
            Option<KeyCode>,
            Option<KeyCode>,
        ),
    ) {
        let mut items_removed = 0;
        for used_key_idx in 0..self.vec_used_keycode.len() {
            if Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.0
                || Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.1
                || Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.2
                || Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.3
                || Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.4
            {
                self.vec_used_keycode.remove(used_key_idx - items_removed);
                items_removed += 1;
                // we only need to get rid of NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES
                if items_removed >= NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES as usize {
                    return;
                }
            }
        }
    }

    fn set_select(&mut self, select_flag: bool) {
        if !self.sub_selection_keyboard_flag {
            match self.selection {
                x if x == InputConfigMenuOption::Back as u8 => {
                    if select_flag {
                        self.back_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                    } else {
                        self.back_text.fragments_mut()[0].color = Some(graphics::BLACK);
                    }
                }
                x if x == InputConfigMenuOption::PlayerInput as u8 => {
                    if select_flag {
                        self.player_num_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.player_num_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                        self.player_num_text.fragments_mut()[1].text =
                            format!("<{}>", self.player_num + 1);
                    } else {
                        self.player_num_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.player_num_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.player_num_text.fragments_mut()[1].text =
                            format!(" {}", self.player_num + 1);
                    }
                }
                _ => println!("[!] input_config_menu_option didn't find match"),
            }
        } else if self.sub_selection_keyboard_flag {
            match self.sub_selection_keyboard {
                x if x == InputConfigMenuSubOptionKeyboard::Left as u8 => {
                    if select_flag {
                        self.k_left_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.k_left_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.k_left_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.k_left_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.k_left_text.fragments_mut()[1].text = format!(" {:?}", self.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                        self.most_recently_pressed_key = None;
                    }
                }
                x if x == InputConfigMenuSubOptionKeyboard::Right as u8 => {
                    if select_flag {
                        self.k_right_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.k_right_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.k_right_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.k_right_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.k_right_text.fragments_mut()[1].text = format!(" {:?}", self.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                        self.most_recently_pressed_key = None;
                    }
                }
                x if x == InputConfigMenuSubOptionKeyboard::Down as u8 => {
                    if select_flag {
                        self.k_down_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.k_down_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.k_down_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.k_down_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.k_down_text.fragments_mut()[1].text = format!(" {:?}", self.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                        self.most_recently_pressed_key = None;
                    }
                }
                x if x == InputConfigMenuSubOptionKeyboard::RotateCw as u8 => {
                    if select_flag {
                        self.k_rotate_cw_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.k_rotate_cw_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.k_rotate_cw_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.k_rotate_cw_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.k_rotate_cw_text.fragments_mut()[1].text = format!(" {:?}", self.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                        self.most_recently_pressed_key = None;
                    }
                }
                x if x == InputConfigMenuSubOptionKeyboard::RotateCcw as u8 => {
                    if select_flag {
                        self.k_rotate_ccw_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.k_rotate_ccw_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.k_rotate_ccw_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.k_rotate_ccw_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.k_rotate_ccw_text.fragments_mut()[1].text = format!(" {:?}", self.most_recently_pressed_key.expect("[!] was setting keycode text, but most_recently_pressed_key == None"));
                        self.most_recently_pressed_key = None;
                    }
                }
                _ => println!("[!] input_config_menu_option didn't find match"),
            }
        }
    }

    fn inc_or_dec_selection(&mut self, inc_flag: bool) {
        if !self.sub_selection_keyboard_flag {
            if self.selection == InputConfigMenuOption::PlayerInput as u8 {
                if inc_flag {
                    self.player_num = (self.player_num + 1) % MAX_NUM_PLAYERS;
                } else {
                    self.player_num = if self.player_num == 0 {
                        MAX_NUM_PLAYERS - 1
                    } else {
                        self.player_num - 1
                    };
                }
                // display player_num + 1 because index by 1 to users
                self.player_num_text.fragments_mut()[1].text = format!("<{}>", self.player_num + 1);
                self.update_sub_text_strings();
            }
        }
    }

    fn update_sub_text_strings(&mut self) {
        if let Some(ctrls) = self.arr_split_controls[self.player_num as usize].0 {
            match ctrls.0 {
                Some(keycode) => {
                    self.k_left_text.fragments_mut()[1].text = format!("{:?}", keycode);
                }
                None => {
                    self.k_left_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.1 {
                Some(keycode) => {
                    self.k_right_text.fragments_mut()[1].text = format!("{:?}", keycode);
                }
                None => {
                    self.k_right_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.2 {
                Some(keycode) => {
                    self.k_down_text.fragments_mut()[1].text = format!("{:?}", keycode);
                }
                None => {
                    self.k_down_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.3 {
                Some(keycode) => {
                    self.k_rotate_cw_text.fragments_mut()[1].text = format!("{:?}", keycode);
                }
                None => {
                    self.k_rotate_cw_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.4 {
                Some(keycode) => {
                    self.k_rotate_ccw_text.fragments_mut()[1].text = format!("{:?}", keycode);
                }
                None => {
                    self.k_rotate_ccw_text.fragments_mut()[1].text = "None".to_string();
                }
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let window_dimensions = graphics::size(ctx);

        self.draw_text(ctx, &self.back_text, 0.1, &window_dimensions);
        self.draw_text(ctx, &self.player_num_text, 0.3, &window_dimensions);

        // display nothing special on InputConfigMenuOption::Back, so just draw the extra stuff when it's not on InputConfigMenuOption::Back
        // and then later determine which of the other InputConfigMenuOption's it is for the specifics
        if self.selection != InputConfigMenuOption::Back as u8 {
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

            if self.selection == InputConfigMenuOption::PlayerInput as u8 {
                if self.keycode_conflict_flag {
                    self.draw_text(ctx, &self.keycode_conflict_text, 0.43, &window_dimensions);
                }

                if (self.arr_split_controls[self.player_num as usize].0).is_some() {
                    self.draw_text(ctx, &self.k_left_text, 0.5, &window_dimensions);
                    self.draw_text(ctx, &self.k_right_text, 0.55, &window_dimensions);
                    self.draw_text(ctx, &self.k_down_text, 0.6, &window_dimensions);
                    self.draw_text(ctx, &self.k_rotate_cw_text, 0.65, &window_dimensions);
                    self.draw_text(ctx, &self.k_rotate_ccw_text, 0.7, &window_dimensions);
                    self.draw_text(ctx, &self.k_start_text, 0.75, &window_dimensions);
                } else if self.arr_split_controls[self.player_num as usize].1 {
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
