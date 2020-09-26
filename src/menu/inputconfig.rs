use ggez::Context;
use ggez::graphics::{self, DrawParam};
use ggez::event::{Button, Axis, KeyCode};
use ggez::graphics::{Scale, Text, TextFragment};
use ggez::nalgebra::Point2;

use crate::inputs::Input;

use crate::menu::MAX_NUM_GAMEPAD_PROFILES;
use crate::menu::MAX_NUM_PLAYERS;

use crate::menu::TEXT_SCALE_DOWN;
use crate::menu::SUB_TEXT_SCALE_DOWN;
use crate::menu::MINI_TEXT_SCALE_DOWN;

use crate::menu::DARK_GRAY;
use crate::menu::LIGHT_GRAY;
use crate::menu::SELECT_GREEN;
use crate::menu::HELP_RED;

const SUB_TEXT_Y_TOP: f32 = 0.48;
const SUB_TEXT_Y_DIFF: f32 = 0.04;

const NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES: u8 = 3;
#[repr(u8)]
enum InputConfigMenuOption {
    Back,
    GamepadProfile,
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
const NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES: u8 = 9;
#[repr(u8)]
enum InputConfigMenuSubOptionGamepad {
    AxisLeft,
    AxisRight,
    AxisDown,
    ButtonLeft,
    ButtonRight,
    ButtonDown,
    ButtonRotateCw,
    ButtonRotateCcw,
    ButtonStart,
}

pub struct InputConfigMenu {
    // logic
    selection: u8,
    player_controls: u8,
    profile_num: u8,
    choose_profile_num: u8,
    choose_profile_flag: bool,
    sub_selection_keyboard: u8,
    sub_selection_keyboard_flag: bool,
    sub_selection_gamepad: u8,
    sub_selection_gamepad_flag: bool,
    pub most_recently_pressed_key: Option<KeyCode>,
    pub most_recently_pressed_gamepad_button: Option<Button>,
    pub most_recently_pressed_gamepad_axis: Option<(Axis, bool)>,
    pub gamepad_axis_wait: (bool, Option<(Axis, bool)>),
    vec_used_keycode: Vec<KeyCode>,
    keycode_conflict_flag: bool,
    button_conflict_flag: bool,
    axis_conflict_flag: bool,
    cant_skip_both_flag: bool,
    input_type_unknown_flag: bool,
    pub arr_controls: [(Option<(Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>, Option<u8>); MAX_NUM_PLAYERS as usize],
    pub arr_gamepad_profiles: [Option<((Option<Button>, Option<(Axis, bool)>), (Option<Button>, Option<(Axis, bool)>), (Option<Button>, Option<(Axis, bool)>), Option<Button>, Option<Button>, Option<Button>)>; MAX_NUM_GAMEPAD_PROFILES as usize],
    // text
    back_text: Text,
    gamepad_profile_text: Text,
    player_controls_text: Text,
    // subtext
    input_uninitialized_text: Text,
    gamepad_profile_uninitialized_text: Text,
    keycode_conflict_text: Text,
    button_conflict_text: Text,
    axis_conflict_text: Text,
    skip_button_axis_text: Text,
    cant_skip_both_text: Text,
    input_type_unknown_text: Text,
    choose_profile_text: Text,
    k_left_text: Text,
    k_right_text: Text,
    k_down_text: Text,
    k_rotate_cw_text: Text,
    k_rotate_ccw_text: Text,
    k_start_text: Text,
    g_axis_left_text: Text,
    g_axis_right_text: Text,
    g_axis_down_text: Text,
    g_button_left_text: Text,
    g_button_right_text: Text,
    g_button_down_text: Text,
    g_button_rotate_cw_text: Text,
    g_button_rotate_ccw_text: Text,
    g_button_start_text: Text,
}

impl InputConfigMenu {
    pub fn new(window_dimensions: (f32, f32), last_used_keyboard_controls: Vec<(u8, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>) -> Self {
        let mut vec_used_keycode: Vec<KeyCode> = vec![];
        let mut arr_controls: [(Option<(Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)>, Option<u8>); MAX_NUM_PLAYERS as usize] = [(None, None); MAX_NUM_PLAYERS as usize];
        for ctrls in last_used_keyboard_controls.iter() {
            if ctrls.0 < MAX_NUM_PLAYERS {
                arr_controls[ctrls.0 as usize].0 = Some((ctrls.1, ctrls.2, ctrls.3, ctrls.4, ctrls.5));
            }
            if let Some(key) = ctrls.1 {
                vec_used_keycode.push(key);
            }
            if let Some(key) = ctrls.2 {
                vec_used_keycode.push(key);
            }
            if let Some(key) = ctrls.3 {
                vec_used_keycode.push(key);
            }
            if let Some(key) = ctrls.4 {
                vec_used_keycode.push(key);
            }
            if let Some(key) = ctrls.5 {
                vec_used_keycode.push(key);
            }
        }
        let mut player_controls_text = Text::new(TextFragment::new("Player Number: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        player_controls_text.add(TextFragment::new(" 1").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        let mut gamepad_profile_text = Text::new(TextFragment::new("GamePad Profile: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        gamepad_profile_text.add(TextFragment::new(" 1").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN)));
        let mut choose_profile_text = Text::new(TextFragment::new("Profile:").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut k_left_text = Text::new(TextFragment::new("Left:     ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut k_right_text = Text::new(TextFragment::new("Right:    ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut k_down_text = Text::new(TextFragment::new("Down:     ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut k_rotate_cw_text = Text::new(TextFragment::new("RotateCw:  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut k_rotate_ccw_text = Text::new(TextFragment::new("RotateCcw:  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let k_start_text = Text::new(TextFragment::new("Start/Pause: Esc").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut g_axis_left_text = Text::new(TextFragment::new("Left (Axis):  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        let mut g_axis_right_text = Text::new(TextFragment::new("Right (Axis): ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        let mut g_axis_down_text = Text::new(TextFragment::new("Down (Axis):  ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        let mut g_button_left_text = Text::new(TextFragment::new("Left (Button): ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut g_button_right_text = Text::new(TextFragment::new("Right (Button): ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut g_button_down_text = Text::new(TextFragment::new("Down (Button): ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut g_button_rotate_cw_text = Text::new(TextFragment::new("RotateCw: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut g_button_rotate_ccw_text = Text::new(TextFragment::new("RotateCcw: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        let mut g_button_start_text = Text::new(TextFragment::new("Start/Pause: ").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        if last_used_keyboard_controls.is_empty() {
            // they'll get set to have the string "None" later in this case anyways
            k_left_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            k_right_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            k_down_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            k_rotate_cw_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            k_rotate_ccw_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        } else {
            k_left_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].1.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            k_right_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].2.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            k_down_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].3.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            k_rotate_cw_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].4.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
            k_rotate_ccw_text.add(TextFragment::new(format!("{:?}", last_used_keyboard_controls[0].5.expect("[!] Passed in Option<KeyCode> is None"))).color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        }
        choose_profile_text.add(TextFragment::new(" 1").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        g_axis_left_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        g_axis_right_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        g_axis_down_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / MINI_TEXT_SCALE_DOWN)));
        g_button_left_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        g_button_right_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        g_button_down_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        g_button_rotate_cw_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        g_button_rotate_ccw_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        g_button_start_text.add(TextFragment::new("").color(graphics::BLACK).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN)));
        Self {
            selection: 0,
            player_controls: 0,
            profile_num: 0,
            choose_profile_num: 0,
            choose_profile_flag: false,
            sub_selection_keyboard: 0,
            sub_selection_keyboard_flag: false,
            sub_selection_gamepad: 0,
            sub_selection_gamepad_flag: false,
            most_recently_pressed_key: None,
            most_recently_pressed_gamepad_button: None,
            most_recently_pressed_gamepad_axis: None,
            gamepad_axis_wait: (false, None),
            vec_used_keycode,
            keycode_conflict_flag: false,
            button_conflict_flag: false,
            axis_conflict_flag: false,
            cant_skip_both_flag: false,
            input_type_unknown_flag: false,
            arr_controls,
            arr_gamepad_profiles: [None; MAX_NUM_GAMEPAD_PROFILES as usize],
            // text
            back_text: Text::new(TextFragment::new("Back").color(SELECT_GREEN).scale(Scale::uniform(window_dimensions.1 / TEXT_SCALE_DOWN))),
            gamepad_profile_text,
            player_controls_text,
            // subtext
            input_uninitialized_text: Text::new(TextFragment::new("No Controls\nKeyboard: Space/Enter\nGamepad: 'G'").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            gamepad_profile_uninitialized_text: Text::new(TextFragment::new("Profile nonexistent\nCreate/edit: Space/Enter").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            keycode_conflict_text: Text::new(TextFragment::new("[!] Redundant KeyCode; ignoring").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            button_conflict_text: Text::new(TextFragment::new("[!] Redundant Button; ignoring").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            axis_conflict_text: Text::new(TextFragment::new("[!] Redundant Axis; ignoring").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            cant_skip_both_text: Text::new(TextFragment::new("[!] Can't skip both").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            input_type_unknown_text: Text::new(TextFragment::new("[!] Unknown input; see README").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            skip_button_axis_text: Text::new(TextFragment::new("Skip Button/Axis: Space/Enter").color(HELP_RED).scale(Scale::uniform(window_dimensions.1 / SUB_TEXT_SCALE_DOWN))),
            choose_profile_text,
            k_left_text,
            k_right_text,
            k_down_text,
            k_rotate_cw_text,
            k_rotate_ccw_text,
            k_start_text,
            g_axis_left_text,
            g_axis_right_text,
            g_axis_down_text,
            g_button_left_text,
            g_button_right_text,
            g_button_down_text,
            g_button_rotate_cw_text,
            g_button_rotate_ccw_text,
            g_button_start_text,
        }
    }

    pub fn update(&mut self, input: &Input) -> bool {
        if input.keydown_right.1 && !self.sub_selection_keyboard_flag {
            self.inc_or_dec_selection(true);
        }

        if input.keydown_left.1 && !self.sub_selection_keyboard_flag {
            self.inc_or_dec_selection(false);
        }

        if !self.sub_selection_keyboard_flag && !self.sub_selection_gamepad_flag && !self.choose_profile_flag {
            if input.keydown_down.1 {
                self.set_select(false);
                self.selection = (self.selection + 1) % NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES;
                self.set_select(true);
            }

            if input.keydown_up.1 {
                self.set_select(false);
                self.selection = if self.selection == 0 {NUM_INPUTCONFIGMENUOPTION_TEXT_ENTRIES - 1} else {self.selection - 1};
                self.set_select(true);
            }

            if input.keydown_rotate_cw.1 && self.selection == InputConfigMenuOption::PlayerInput as u8 {
                self.choose_profile_flag = true;
                self.set_select(true);
            }

            if input.keydown_start.1 {
                if self.selection == InputConfigMenuOption::Back as u8 {
                    self.sub_selection_keyboard = 0;
                    return true;
                } else if self.selection == InputConfigMenuOption::GamepadProfile as u8 {
                    self.most_recently_pressed_gamepad_axis = None;
                    self.most_recently_pressed_gamepad_button = None;
                    self.arr_gamepad_profiles[self.profile_num as usize] = Some(((None, None), (None, None), (None, None), None, None, None));
                    self.update_sub_text_strings_gamepad();
                    self.sub_selection_gamepad_flag = true;
                    self.set_select(true);
                } else if self.selection == InputConfigMenuOption::PlayerInput as u8 {
                    self.most_recently_pressed_key = None;
                    if let Some(ctrls) = self.arr_controls[self.player_controls as usize].0 {
                        self.remove_from_used_keycodes(ctrls);
                    }
                    self.arr_controls[self.player_controls as usize].0 = None;

                    self.arr_controls[self.player_controls as usize].0 = Some((None, None, None, None, None));
                    self.update_sub_text_strings_keyboard();
                    self.sub_selection_keyboard_flag = true;
                    self.set_select(true);
                }
            }
        } else if self.sub_selection_gamepad_flag {
            // if Space/Enter are pressed here, we want to skip the current selection if it's left/right/down axis or button unless that would leave None on both
            if input.keydown_start.1 {
                match self.sub_selection_gamepad {
                    x if x == InputConfigMenuSubOptionGamepad::AxisLeft as u8 => {
                        self.set_select(false);
                        if self.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                            self.sub_selection_gamepad += 1;
                            self.set_select(true);
                        } else {
                            self.sub_selection_gamepad = 0;
                            self.sub_selection_gamepad_flag = false;
                        }
                    },
                    x if x == InputConfigMenuSubOptionGamepad::AxisRight as u8 => {
                        self.set_select(false);
                        if self.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                            self.sub_selection_gamepad += 1;
                            self.set_select(true);
                        } else {
                            self.sub_selection_gamepad = 0;
                            self.sub_selection_gamepad_flag = false;
                        }
                    },
                    x if x == InputConfigMenuSubOptionGamepad::AxisDown as u8 => {
                        self.set_select(false);
                        if self.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                            self.sub_selection_gamepad += 1;
                            self.set_select(true);
                        } else {
                            self.sub_selection_gamepad = 0;
                            self.sub_selection_gamepad_flag = false;
                        }
                    },
                    x if x == InputConfigMenuSubOptionGamepad::ButtonLeft as u8 => {
                        if ((self.arr_gamepad_profiles[self.profile_num as usize].expect("[!] profile unexpectedly None").0).1).is_some() {
                            self.set_select(false);
                            if self.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                                self.sub_selection_gamepad += 1;
                                self.set_select(true);
                            } else {
                                self.sub_selection_gamepad = 0;
                                self.sub_selection_gamepad_flag = false;
                            }
                            self.cant_skip_both_flag = false;
                        } else {
                            self.cant_skip_both_flag = true;
                        }
                    },
                    x if x == InputConfigMenuSubOptionGamepad::ButtonRight as u8 => {
                        if ((self.arr_gamepad_profiles[self.profile_num as usize].expect("[!] profile unexpectedly None").1).1).is_some() {
                            self.set_select(false);
                            if self.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                                self.sub_selection_gamepad += 1;
                                self.set_select(true);
                            } else {
                                self.sub_selection_gamepad = 0;
                                self.sub_selection_gamepad_flag = false;
                            }
                            self.cant_skip_both_flag = false;
                        } else {
                            self.cant_skip_both_flag = true;
                        }
                    },
                    x if x == InputConfigMenuSubOptionGamepad::ButtonDown as u8 => {
                        if ((self.arr_gamepad_profiles[self.profile_num as usize].expect("[!] profile unexpectedly None").2).1).is_some() {
                            self.set_select(false);
                            if self.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                                self.sub_selection_gamepad += 1;
                                self.set_select(true);
                            } else {
                                self.sub_selection_gamepad = 0;
                                self.sub_selection_gamepad_flag = false;
                            }
                            self.cant_skip_both_flag = false;
                        } else {
                            self.cant_skip_both_flag = true;
                        }
                    },
                    _ => (),
                }

                self.most_recently_pressed_gamepad_axis = None;
                self.most_recently_pressed_gamepad_button = None;
            }

            if self.most_recently_pressed_gamepad_axis.is_some() {
                if self.most_recently_pressed_gamepad_axis == Some((Axis::Unknown, false))
                || self.most_recently_pressed_gamepad_axis == Some((Axis::Unknown, true)) {
                    self.input_type_unknown_flag = true;
                } else {
                    let mut not_on_axis_flag: bool = false;
                    if let Some(mut profile) = self.arr_gamepad_profiles[self.profile_num as usize].as_mut() {
                        match self.sub_selection_gamepad {
                            x if x == InputConfigMenuSubOptionGamepad::AxisLeft as u8 => {
                                (profile.0).1 = self.most_recently_pressed_gamepad_axis;
                            },
                            x if x == InputConfigMenuSubOptionGamepad::AxisRight as u8 => {
                                if self.most_recently_pressed_gamepad_axis != (profile.0).1 {
                                    (profile.1).1 = self.most_recently_pressed_gamepad_axis;
                                    self.axis_conflict_flag = false;
                                } else {
                                    self.most_recently_pressed_gamepad_axis = None;
                                    self.axis_conflict_flag = true;
                                }
                            },
                            x if x == InputConfigMenuSubOptionGamepad::AxisDown as u8 => {
                                if self.most_recently_pressed_gamepad_axis != (profile.0).1
                                && self.most_recently_pressed_gamepad_axis != (profile.1).1 {
                                    (profile.2).1 = self.most_recently_pressed_gamepad_axis;
                                    self.axis_conflict_flag = false;
                                } else {
                                    self.most_recently_pressed_gamepad_axis = None;
                                    self.axis_conflict_flag = true;
                                }
                            },
                            _ => not_on_axis_flag = true,
                        }
                    } else {
                        println!("[!] gamepad profile unexpectedly None");
                    }
                    if !not_on_axis_flag && !self.axis_conflict_flag {
                        self.set_select(false);
                        if self.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                            self.sub_selection_gamepad += 1;
                            self.set_select(true);
                        } else {
                            self.sub_selection_gamepad = 0;
                            self.sub_selection_gamepad_flag = false;
                        }
                    } else {
                        self.most_recently_pressed_gamepad_axis = None;
                    }
                    self.cant_skip_both_flag = false;
                    self.input_type_unknown_flag = false;
                }
                self.most_recently_pressed_key = None;
            }

            if self.most_recently_pressed_gamepad_button.is_some() {
                if self.most_recently_pressed_gamepad_button == Some(Button::Unknown) {
                    self.input_type_unknown_flag = true;
                } else {
                    let mut not_on_button_flag: bool = false;
                    if let Some(mut profile) = self.arr_gamepad_profiles[self.profile_num as usize].as_mut() {
                        match self.sub_selection_gamepad {
                            x if x == InputConfigMenuSubOptionGamepad::ButtonLeft as u8 => {
                                (profile.0).0 = self.most_recently_pressed_gamepad_button;
                            },
                            x if x == InputConfigMenuSubOptionGamepad::ButtonRight as u8 => {
                                if self.most_recently_pressed_gamepad_button != (profile.0).0 {
                                    (profile.1).0 = self.most_recently_pressed_gamepad_button;
                                    self.button_conflict_flag = false;
                                } else {
                                    self.most_recently_pressed_gamepad_button = None;
                                    self.button_conflict_flag = true;
                                }
                            },
                            x if x == InputConfigMenuSubOptionGamepad::ButtonDown as u8 => {
                                if self.most_recently_pressed_gamepad_button != (profile.0).0
                                && self.most_recently_pressed_gamepad_button != (profile.1).0 {
                                    (profile.2).0 = self.most_recently_pressed_gamepad_button;
                                    self.button_conflict_flag = false;
                                } else {
                                    self.most_recently_pressed_gamepad_button = None;
                                    self.button_conflict_flag = true;
                                }
                            },
                            x if x == InputConfigMenuSubOptionGamepad::ButtonRotateCw as u8 => {
                                if self.most_recently_pressed_gamepad_button != (profile.0).0
                                && self.most_recently_pressed_gamepad_button != (profile.1).0
                                && self.most_recently_pressed_gamepad_button != (profile.2).0 {
                                    profile.3 = self.most_recently_pressed_gamepad_button;
                                    self.button_conflict_flag = false;
                                } else {
                                    self.most_recently_pressed_gamepad_button = None;
                                    self.button_conflict_flag = true;
                                }
                            },
                            x if x == InputConfigMenuSubOptionGamepad::ButtonRotateCcw as u8 => {
                                if self.most_recently_pressed_gamepad_button != (profile.0).0
                                && self.most_recently_pressed_gamepad_button != (profile.1).0
                                && self.most_recently_pressed_gamepad_button != (profile.2).0
                                && self.most_recently_pressed_gamepad_button != profile.3 {
                                    profile.4 = self.most_recently_pressed_gamepad_button;
                                    self.button_conflict_flag = false;
                                } else {
                                    self.most_recently_pressed_gamepad_button = None;
                                    self.button_conflict_flag = true;
                                }
                            },
                            x if x == InputConfigMenuSubOptionGamepad::ButtonStart as u8 => {
                                if self.most_recently_pressed_gamepad_button != (profile.0).0
                                && self.most_recently_pressed_gamepad_button != (profile.1).0
                                && self.most_recently_pressed_gamepad_button != (profile.2).0
                                && self.most_recently_pressed_gamepad_button != profile.3
                                && self.most_recently_pressed_gamepad_button != profile.4 {
                                    profile.5 = self.most_recently_pressed_gamepad_button;
                                    self.button_conflict_flag = false;
                                } else {
                                    self.most_recently_pressed_gamepad_button = None;
                                    self.button_conflict_flag = true;
                                }
                            },
                            _ => not_on_button_flag = true,
                        }
                    } else {
                        println!("[!] gamepad profile unexpectedly None");
                    }
                    if !not_on_button_flag && !self.button_conflict_flag {
                        self.set_select(false);
                        if self.sub_selection_gamepad < NUM_INPUTCONFIGMENUSUBOPTIONGAMEPAD_TEXT_ENTRIES as u8 - 1 {
                            self.sub_selection_gamepad += 1;
                            self.set_select(true);
                        } else {
                            self.sub_selection_gamepad = 0;
                            self.sub_selection_gamepad_flag = false;
                        }
                    } else {
                        self.most_recently_pressed_gamepad_button = None;
                    }
                    self.cant_skip_both_flag = false;
                    self.input_type_unknown_flag = false;
                }

                self.most_recently_pressed_gamepad_axis = None;
                self.most_recently_pressed_key = None;
            }

            if self.most_recently_pressed_key == Some(KeyCode::Escape) {
                self.set_select(false);
                self.button_conflict_flag = false;
                self.axis_conflict_flag = false;
                self.arr_gamepad_profiles[self.profile_num as usize] = None;
                self.sub_selection_gamepad = 0;
                self.sub_selection_gamepad_flag = false;
            }
        } else if self.sub_selection_keyboard_flag {
            if self.most_recently_pressed_key.is_some() {
                // first check if the KeyCode is Escape, and if it is, just delete the layout entry and go out of the subselection section
                // second check if the KeyCode was already used. If it was, set the error message flag to true
                if self.most_recently_pressed_key == Some(KeyCode::Escape) {
                    self.set_select(false);
                    self.keycode_conflict_flag = false;
                    self.sub_selection_keyboard = 0;
                    self.sub_selection_keyboard_flag = false;
                    // the user was in the middle of creating keyboard controls when they hit Escape, so pop however many KeyCode's off vec_used_keycode as the user set up
                    if let Some(ctrls) = self.arr_controls[self.player_controls as usize].0 {
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
                    self.arr_controls[self.player_controls as usize].0 = None;
                } else if self.vec_used_keycode.contains(&self.most_recently_pressed_key.expect("[!] KeyCode of most recently pressed key is unexpectedly None")) {
                    self.keycode_conflict_flag = true;
                } else {
                    self.keycode_conflict_flag = false;
                    match (self.arr_controls[self.player_controls as usize].0).as_mut() {
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
                            println!("[!] arr_controls[{}].0 was unexpectedly None", self.player_controls);
                        }
                    }
                    self.set_select(false);
                    if self.sub_selection_keyboard < NUM_INPUTCONFIGMENUSUBOPTIONKEYBOARD_TEXT_ENTRIES as u8 - 1 {
                        self.sub_selection_keyboard += 1;
                        self.set_select(true);
                    } else {
                        self.sub_selection_keyboard = 0;
                        self.sub_selection_keyboard_flag = false;
                    }
                }
            }
        } else if self.choose_profile_flag {
            if input.keydown_start.1 {
                self.set_select(false);
                self.arr_controls[self.player_controls as usize].1 = Some(self.choose_profile_num);
                if let Some(ctrls) = self.arr_controls[self.player_controls as usize].0 {
                    self.remove_from_used_keycodes(ctrls);
                    self.arr_controls[self.player_controls as usize].0 = None;
                }
                self.choose_profile_flag = false;
            }
        }
        false
    }

    fn remove_from_used_keycodes(&mut self, ctrls: (Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>, Option<KeyCode>)) {
        let mut items_removed = 0;
        for used_key_idx in 0..self.vec_used_keycode.len() {
            if Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.0
            || Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.1
            || Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.2
            || Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.3
            || Some(self.vec_used_keycode[used_key_idx - items_removed]) == ctrls.4 {
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
        if !self.sub_selection_keyboard_flag && !self.sub_selection_gamepad_flag && !self.choose_profile_flag {
            match self.selection {
                x if x == InputConfigMenuOption::Back as u8 => {
                    if select_flag {
                        self.back_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                    } else {
                        self.back_text.fragments_mut()[0].color = Some(graphics::BLACK);
                    }
                },
                x if x == InputConfigMenuOption::GamepadProfile as u8 => {
                    if select_flag {
                        self.gamepad_profile_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.gamepad_profile_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                        self.gamepad_profile_text.fragments_mut()[1].text = format!("<{}>", self.profile_num + 1);
                    } else {
                        self.gamepad_profile_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.gamepad_profile_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.gamepad_profile_text.fragments_mut()[1].text = format!(" {}", self.profile_num + 1);
                    }
                },
                x if x == InputConfigMenuOption::PlayerInput as u8 => {
                    if select_flag {
                        self.player_controls_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.player_controls_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                        self.player_controls_text.fragments_mut()[1].text = format!("<{}>", self.player_controls + 1);
                    } else {
                        self.player_controls_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.player_controls_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        self.player_controls_text.fragments_mut()[1].text = format!(" {}", self.player_controls + 1);
                    }
                },
                _ => println!("[!] input_config_menu_option didn't find match"),
            }
        } else if self.sub_selection_gamepad_flag {
            match self.sub_selection_gamepad {
                x if x == InputConfigMenuSubOptionGamepad::AxisLeft as u8 => {
                    if select_flag {
                        self.g_axis_left_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_axis_left_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_axis_left_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_axis_left_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_axis {
                            Some(axis) => self.g_axis_left_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'}),
                            None => self.g_axis_left_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_axis = None;
                    }
                },
                x if x == InputConfigMenuSubOptionGamepad::AxisRight as u8 => {
                    if select_flag {
                        self.g_axis_right_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_axis_right_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_axis_right_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_axis_right_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_axis {
                            Some(axis) => self.g_axis_right_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'}),
                            None => self.g_axis_right_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_axis = None;
                    }
                },
                x if x == InputConfigMenuSubOptionGamepad::AxisDown as u8 => {
                    if select_flag {
                        self.g_axis_down_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_axis_down_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_axis_down_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_axis_down_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_axis {
                            Some(axis) => self.g_axis_down_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'}),
                            None => self.g_axis_down_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_axis = None;
                    }
                },
                x if x == InputConfigMenuSubOptionGamepad::ButtonLeft as u8 => {
                    if select_flag {
                        self.g_button_left_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_button_left_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_button_left_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_button_left_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_button {
                            Some(button) => self.g_button_left_text.fragments_mut()[1].text = format!("{:?}", button),
                            None => self.g_button_left_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_button = None;
                    }
                },
                x if x == InputConfigMenuSubOptionGamepad::ButtonRight as u8 => {
                    if select_flag {
                        self.g_button_right_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_button_right_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_button_right_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_button_right_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_button {
                            Some(button) => self.g_button_right_text.fragments_mut()[1].text = format!("{:?}", button),
                            None => self.g_button_right_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_button = None;
                    }
                },
                x if x == InputConfigMenuSubOptionGamepad::ButtonDown as u8 => {
                    if select_flag {
                        self.g_button_down_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_button_down_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_button_down_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_button_down_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_button {
                            Some(button) => self.g_button_down_text.fragments_mut()[1].text = format!("{:?}", button),
                            None => self.g_button_down_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_button = None;
                    }
                },
                x if x == InputConfigMenuSubOptionGamepad::ButtonRotateCw as u8 => {
                    if select_flag {
                        self.g_button_rotate_cw_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_button_rotate_cw_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_button_rotate_cw_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_button_rotate_cw_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_button {
                            Some(button) => self.g_button_rotate_cw_text.fragments_mut()[1].text = format!("{:?}", button),
                            None => self.g_button_rotate_cw_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_button = None;
                    }
                },
                x if x == InputConfigMenuSubOptionGamepad::ButtonRotateCcw as u8 => {
                    if select_flag {
                        self.g_button_rotate_ccw_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_button_rotate_ccw_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_button_rotate_ccw_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_button_rotate_ccw_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_button {
                            Some(button) => self.g_button_rotate_ccw_text.fragments_mut()[1].text = format!("{:?}", button),
                            None => self.g_button_rotate_ccw_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_button = None;
                    }
                },
                x if x == InputConfigMenuSubOptionGamepad::ButtonStart as u8 => {
                    if select_flag {
                        self.g_button_start_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                        self.g_button_start_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                    } else {
                        self.g_button_start_text.fragments_mut()[0].color = Some(graphics::BLACK);
                        self.g_button_start_text.fragments_mut()[1].color = Some(graphics::BLACK);
                        match self.most_recently_pressed_gamepad_button {
                            Some(button) => self.g_button_start_text.fragments_mut()[1].text = format!("{:?}", button),
                            None => self.g_button_start_text.fragments_mut()[1].text = format!("None"),
                        }
                        self.most_recently_pressed_gamepad_button = None;
                    }
                },
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
                },
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
                },
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
                },
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
                },
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
                },
                _ => println!("[!] input_config_menu_option didn't find match"),
            } 
        } else if self.choose_profile_flag {
            if select_flag {
                self.choose_profile_text.fragments_mut()[0].color = Some(SELECT_GREEN);
                self.choose_profile_text.fragments_mut()[1].color = Some(SELECT_GREEN);
                self.choose_profile_text.fragments_mut()[1].text = format!("<{}>", self.choose_profile_num + 1);
            } else {
                self.choose_profile_text.fragments_mut()[0].color = Some(graphics::BLACK);
                self.choose_profile_text.fragments_mut()[1].color = Some(graphics::BLACK);
                self.choose_profile_text.fragments_mut()[1].text = format!(" {}", self.choose_profile_num + 1);
                self.most_recently_pressed_key = None;
            }
        }
    }

    fn inc_or_dec_selection(&mut self, inc_flag: bool) {
        if !self.sub_selection_keyboard_flag && !self.sub_selection_gamepad_flag && !self.choose_profile_flag {
            if self.selection == InputConfigMenuOption::PlayerInput as u8 {
                if inc_flag {
                    self.player_controls = (self.player_controls + 1) % MAX_NUM_PLAYERS;
                } else {
                    self.player_controls = if self.player_controls == 0 {MAX_NUM_PLAYERS - 1} else {self.player_controls - 1};
                }
                // display player_controls + 1 because index by 1 to users
                self.player_controls_text.fragments_mut()[1].text = format!("<{}>", self.player_controls + 1);
                self.update_sub_text_strings_keyboard();
            } else if self.selection == InputConfigMenuOption::GamepadProfile as u8 {
                if inc_flag {
                    self.profile_num = (self.profile_num + 1) % MAX_NUM_GAMEPAD_PROFILES;
                } else {
                    self.profile_num = if self.profile_num == 0 {MAX_NUM_GAMEPAD_PROFILES - 1} else {self.profile_num - 1};
                }
                // display profile_num + 1 because index by 1 to users
                self.gamepad_profile_text.fragments_mut()[1].text = format!("<{}>", self.profile_num + 1);
                self.update_sub_text_strings_gamepad();
            }
        } else if self.choose_profile_flag {
            if inc_flag {
                self.choose_profile_num = (self.choose_profile_num + 1) % MAX_NUM_GAMEPAD_PROFILES;
            } else {
                self.choose_profile_num = if self.choose_profile_num == 0 {MAX_NUM_GAMEPAD_PROFILES - 1} else {self.choose_profile_num - 1};
            }
            self.choose_profile_text.fragments_mut()[1].text = format!("<{}>", self.choose_profile_num + 1);
        }
    }

    fn update_sub_text_strings_gamepad(&mut self) {
        if let Some(profile) = self.arr_gamepad_profiles[self.profile_num as usize] {
            // axes
            match (profile.0).1 {
                Some(axis) => {
                    self.g_axis_left_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'});
                },
                None => {
                    self.g_axis_left_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match (profile.1).1 {
                Some(axis) => {
                    self.g_axis_right_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'});
                },
                None => {
                    self.g_axis_right_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match (profile.2).1 {
                Some(axis) => {
                    self.g_axis_down_text.fragments_mut()[1].text = format!("{:?}{}", axis.0, if axis.1 {'+'} else {'-'});
                },
                None => {
                    self.g_axis_down_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            // buttons
            match (profile.0).0 {
                Some(button) => {
                    self.g_button_left_text.fragments_mut()[1].text = format!("{:?}", button);
                },
                None => {
                    self.g_button_left_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match (profile.1).0 {
                Some(button) => {
                    self.g_button_right_text.fragments_mut()[1].text = format!("{:?}", button);
                },
                None => {
                    self.g_button_right_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match (profile.2).0 {
                Some(button) => {
                    self.g_button_down_text.fragments_mut()[1].text = format!("{:?}", button);
                },
                None => {
                    self.g_button_down_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match profile.3 {
                Some(button) => {
                    self.g_button_rotate_cw_text.fragments_mut()[1].text = format!("{:?}", button);
                },
                None => {
                    self.g_button_rotate_cw_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match profile.4 {
                Some(button) => {
                    self.g_button_rotate_ccw_text.fragments_mut()[1].text = format!("{:?}", button);
                },
                None => {
                    self.g_button_rotate_ccw_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match profile.5 {
                Some(button) => {
                    self.g_button_start_text.fragments_mut()[1].text = format!("{:?}", button);
                },
                None => {
                    self.g_button_start_text.fragments_mut()[1].text = "None".to_string();
                }
            }
        }
    }

    fn update_sub_text_strings_keyboard(&mut self) {
        if let Some(ctrls) = self.arr_controls[self.player_controls as usize].0 {
            match ctrls.0 {
                Some(keycode) => {
                    self.k_left_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.k_left_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.1 {
                Some(keycode) => {
                    self.k_right_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.k_right_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.2 {
                Some(keycode) => {
                    self.k_down_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.k_down_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.3 {
                Some(keycode) => {
                    self.k_rotate_cw_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.k_rotate_cw_text.fragments_mut()[1].text = "None".to_string();
                }
            }
            match ctrls.4 {
                Some(keycode) => {
                    self.k_rotate_ccw_text.fragments_mut()[1].text = format!("{:?}", keycode);
                },
                None => {
                    self.k_rotate_ccw_text.fragments_mut()[1].text = "None".to_string();
                }
            }
        } else if let Some(profile) = self.arr_controls[self.player_controls as usize].1 {
            self.choose_profile_text.fragments_mut()[1].text = format!(" {}", profile + 1);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let window_dimensions = graphics::size(ctx);

        self.draw_text(ctx, &self.back_text, 0.1, &window_dimensions);
        self.draw_text(ctx, &self.gamepad_profile_text, 0.2, &window_dimensions);
        self.draw_text(ctx, &self.player_controls_text, 0.3, &window_dimensions);

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
            if !self.sub_selection_keyboard_flag && !self.sub_selection_gamepad_flag && !self.choose_profile_flag {
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
                ).unwrap();
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
                ).unwrap();
            }
            graphics::draw(ctx, &editing_indicator_rectangle, (Point2::new(0.0, 0.0),)).unwrap();

            if self.selection == InputConfigMenuOption::GamepadProfile as u8 {
                if self.button_conflict_flag {
                    self.draw_text(ctx, &self.button_conflict_text, 0.43, &window_dimensions);
                } else if self.axis_conflict_flag {
                    self.draw_text(ctx, &self.axis_conflict_text, 0.43, &window_dimensions);
                } else if self.cant_skip_both_flag {
                    self.draw_text(ctx, &self.cant_skip_both_text, 0.43, &window_dimensions);
                } else if self.input_type_unknown_flag {
                    self.draw_text(ctx, &self.input_type_unknown_text, 0.43, &window_dimensions);
                }

                if self.arr_gamepad_profiles[self.profile_num as usize].is_some() {
                    self.draw_text(ctx, &self.g_axis_left_text, SUB_TEXT_Y_TOP, &window_dimensions);
                    self.draw_text(ctx, &self.g_axis_right_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF, &window_dimensions);
                    self.draw_text(ctx, &self.g_axis_down_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 2.0, &window_dimensions);
                    self.draw_text(ctx, &self.g_button_left_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 3.0, &window_dimensions);
                    self.draw_text(ctx, &self.g_button_right_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 4.0, &window_dimensions);
                    self.draw_text(ctx, &self.g_button_down_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 5.0, &window_dimensions);
                    self.draw_text(ctx, &self.g_button_rotate_cw_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 6.0, &window_dimensions);
                    self.draw_text(ctx, &self.g_button_rotate_ccw_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 7.0, &window_dimensions);
                    self.draw_text(ctx, &self.g_button_start_text, SUB_TEXT_Y_TOP + SUB_TEXT_Y_DIFF * 8.0, &window_dimensions);
                } else {
                    self.draw_text(ctx, &self.gamepad_profile_uninitialized_text, 0.5, &window_dimensions);
                }

                if self.sub_selection_gamepad_flag {
                    self.draw_text(ctx, &self.skip_button_axis_text, 0.85, &window_dimensions);
                }
            } else if self.selection == InputConfigMenuOption::PlayerInput as u8 {
                if self.keycode_conflict_flag {
                    self.draw_text(ctx, &self.keycode_conflict_text, 0.43, &window_dimensions);
                }

                if self.choose_profile_flag {
                    self.draw_text(ctx, &self.choose_profile_text, 0.5, &window_dimensions);
                } else {
                    if (self.arr_controls[self.player_controls as usize].0).is_some() {
                        self.draw_text(ctx, &self.k_left_text, 0.5, &window_dimensions);
                        self.draw_text(ctx, &self.k_right_text, 0.55, &window_dimensions);
                        self.draw_text(ctx, &self.k_down_text, 0.6, &window_dimensions);
                        self.draw_text(ctx, &self.k_rotate_cw_text, 0.65, &window_dimensions);
                        self.draw_text(ctx, &self.k_rotate_ccw_text, 0.7, &window_dimensions);
                        self.draw_text(ctx, &self.k_start_text, 0.75, &window_dimensions);
                    } else if (self.arr_controls[self.player_controls as usize].1).is_some() {
                        self.draw_text(ctx, &self.choose_profile_text, 0.5, &window_dimensions);
                    } else {
                        self.draw_text(ctx, &self.input_uninitialized_text, 0.5, &window_dimensions);
                    }
                }
            }
        }
    }

    fn draw_text(&self, ctx: &mut Context, text_var: &Text, vertical_position: f32, window_dimensions: &(f32, f32)) {
        let (text_width, text_height) = text_var.dimensions(ctx);
        graphics::draw(ctx, text_var, DrawParam::new()
        .dest(Point2::new((window_dimensions.0 - text_width as f32) / 2.0, (window_dimensions.1 - text_height as f32) * vertical_position))
        ).unwrap();
    }
}