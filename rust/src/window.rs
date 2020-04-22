//! # A wrapper around the ncurses-rs crate
//!
//! This module exports just the curses interface used by Streets and
//! Alleys.

// Copyright (c) 2019 John D. Ramsdell
// Permission to use, copy, modify, and distribute this software and its
// documentation for any purpose and without fee is hereby granted,
// provided that the above copyright notice appear in all copies.  John
// Ramsdell makes no representations about the suitability of this
// software for any purpose.  It is provided "as is" without express or
// implied warranty.

extern crate ncurses;

use ncurses::*;

/// The struct that holds a curses wndow
pub struct Window {
    w: WINDOW,
}

impl Window {
    /// Create a window
    pub fn new() -> Window {
        Window { w: initscr() }
    }

    /// Get the height of the window
    pub fn getmaxy(&self) -> i32 {
        ncurses::getmaxy(self.w)
    }

    /// Move cursor
    pub fn mov(&self, y: i32, x: i32) -> i32 {
        wmove(self.w, y, x)
    }

    /// Write a charater at the cursor
    pub fn addch(&self, ch: char) -> i32 {
        waddch(self.w, ch as chtype)
    }

    /// Write a string at the cursor
    pub fn addstr(&self, s: &str) -> i32 {
        waddstr(self.w, s)
    }

    /// Clear window
    pub fn clear(&self) -> i32 {
        wclear(self.w)
    }

    /// Clear to end-of-line
    pub fn clrtoeol(&self) -> i32 {
        wclrtoeol(self.w)
    }

    /// Get an ASCII char
    pub fn getch(&self) -> char {
        let i = wgetch(self.w);
        if i < 0 || i >= 128 {
            panic!("Non-ASCII in getch")
        }
        // Hack alert!
        // Convert i32 to char through u8
        let u = i as u8;
        u as char
    }

    /// Disable line buffering and erase/kill character-processing
    pub fn cbreak() -> i32 {
        ncurses::cbreak()
    }

    /// Don't show key presses
    pub fn noecho() -> i32 {
        ncurses::noecho()
    }

    /// Close window
    pub fn endwin() -> i32 {
        ncurses::endwin()
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}
