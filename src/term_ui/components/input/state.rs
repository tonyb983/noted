// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::RangeInclusive;

use unicode_width::UnicodeWidthStr;

pub struct InputState {
    text: String,
    history: Vec<String>,
    position: usize,
    searching: Option<usize>,
    selection: Option<RangeInclusive<usize>>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            history: Vec::new(),
            position: 0,
            searching: None,
            selection: None,
        }
    }

    pub fn width(&self) -> usize {
        self.text.width()
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_at_end() {
            self.text.push(c);
        } else {
            self.text.insert(self.position, c);
        }
        self.position += char::len_utf8(c);
    }

    pub fn backspace_char(&mut self) -> Option<char> {
        if self.cursor_at_end() {
            if let Some(ch) = self.text.pop() {
                self.position -= char::len_utf8(ch);
                Some(ch)
            } else {
                None
            }
        } else if self.cursor_at_start() {
            None
        } else {
            let prev = self.text.floor_char_boundary(self.position - 1);
            let ch = self.text.remove(prev);
            self.position -= char::len_utf8(ch);
            Some(ch)
        }
    }

    pub fn delete_char(&mut self) -> Option<char> {
        if self.cursor_at_end() {
            return None;
        }
        let next = self.text.ceil_char_boundary(self.position);
        if next == self.text.len() {
            self.text.pop()
        } else {
            Some(self.text.remove(next))
        }
    }

    pub fn push_str(&mut self, text: &str) {
        if self.cursor_at_end() {
            self.text.push_str(text);
        } else {
            self.text.insert_str(self.position, text);
        }
        self.position += text.len();
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn get_pos(&self) -> usize {
        self.position
    }

    pub fn set_pos(&mut self, position: usize) {
        let pos = if position > self.text.len() {
            self.text.len()
        } else {
            position
        };
        self.position = pos;
    }

    pub fn move_pos_left(&mut self) {
        if self.position > 0 {
            self.set_pos(self.position - 1);
            self.selection = None;
        }
    }

    pub fn move_pos_right(&mut self) {
        self.set_pos(self.position + 1);
        self.selection = None;
    }

    pub fn move_pos_end(&mut self) {
        self.set_pos(self.text.len());
        self.selection = None;
    }

    pub fn move_pos_home(&mut self) {
        self.set_pos(0);
        self.selection = None;
    }

    pub fn grow_selection_left(&mut self) {
        if let Some(range) = &self.selection {
            if *range.start() > 0 {
                self.selection = Some(RangeInclusive::new(range.start() - 1, *range.end()));
            }
        } else {
            self.selection = Some(RangeInclusive::new(self.position, self.position));
        }
    }

    pub fn grow_selection_right(&mut self) {
        if let Some(range) = &self.selection {
            if *range.end() < self.text.len() {
                self.selection = Some(RangeInclusive::new(*range.start(), range.end() + 1));
            }
        } else {
            self.selection = Some(RangeInclusive::new(self.position, self.position));
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn remove_selected(&mut self) -> Option<String> {
        if let Some(range) = &self.selection {
            let mut text = String::new();
            for i in *range.start()..*range.end() {
                text.push(self.text.remove(*range.start()));
            }
            self.position = *range.start();
            self.selection = None;
            Some(text)
        } else {
            None
        }
    }

    pub fn get_selected_text(&self) -> Option<String> {
        self.selection
            .as_ref()
            .map(|range| self.text.as_str()[*range.start()..*range.end()].to_string())
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn enter_current(&mut self) -> String {
        let text = self.text.clone();
        self.history.push(text.clone());
        self.text.clear();
        self.position = 0;
        self.selection = None;
        text
    }

    pub fn cursor_at_start(&self) -> bool {
        self.position == 0
    }

    pub fn cursor_at_end(&self) -> bool {
        self.position == self.text.len()
    }

    pub fn search_history(&mut self) {
        if self.history.is_empty() {
            self.searching = None;
            return;
        }

        // TODO: Handle case where we are searching all history, should be set by the first if block
        // TODO: Handle case where we are searching prefix, should be set by the loop.

        if self.text.is_empty() {
            if let Some(text) = self.history.last() {
                self.text = text.clone();
                self.set_pos(self.text.len());
            }
            return;
        }

        for text in self.history.iter().rev() {
            if text.starts_with(&self.text) {
                self.text = text.clone();
                self.set_pos(self.text.len());
                return;
            }
        }
    }
}
