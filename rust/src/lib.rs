//! # A Streets and Alleys runner

//! This module runs the game given the information gleaned from
//! command line argument processing.

// Copyright (c) 2019 John D. Ramsdell
// Permission to use, copy, modify, and distribute this software and its
// documentation for any purpose and without fee is hereby granted,
// provided that the above copyright notice appear in all copies.  John
// Ramsdell makes no representations about the suitability of this
// software for any purpose.  It is provided "as is" without express or
// implied warranty.

pub mod board;
pub mod screen;
pub mod window;

use screen::Screen;
use window::Window;

/// Run the game given an initial rank and a version string
pub fn run(nranks: usize, version: &'static str) {
    let w = Window::new();
    Window::cbreak();
    Window::noecho();

    Screen::new(w, version).play(nranks);

    Window::endwin();
}
