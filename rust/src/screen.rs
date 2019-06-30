//! # A Streets and Alleys screen

//! This module defines the screen and the game play that defines the game.

// Copyright (c) 2019 John D. Ramsdell
// Permission to use, copy, modify, and distribute this software and its
// documentation for any purpose and without fee is hereby granted,
// provided that the above copyright notice appear in all copies.  John
// Ramsdell makes no representations about the suitability of this
// software for any purpose.  It is provided "as is" without express or
// implied warranty.

use super::board::*;
use super::window::Window;

/// The structure that holds static information about the display.
pub struct Screen {
    w: Window,
    version: &'static str,
    prompt: i32,  // Row for the prompt
    status: i32,  // Row for the status line
    command: i32, // Row for commands
    title: i32,   // Row for title
}

// Heights of various screen areas
const PROMPT_HEIGHT: i32 = 1;
const STATUS_HEIGHT: i32 = 1;
const COMMAND_HEIGHT: i32 = 2;
const BOARD_HEIGHT: i32 = 19;
const TITLE_HEIGHT: i32 = 1;

// Width giving location of a stack
const STACK_INDENT: i32 = 11;
// Width giving the space used for a card
const CARD_SIZE: i32 = 6;

use Ans::*;

// Used to report the result of a game
enum Ans {
    Win,
    Lose,
}

use Resize::*;

// Used to report the result of a request to resize a game
enum Resize {
    Rank(usize),
    Again,
    Quit,
}

impl Screen {
    /// Create a fresh screen and compute layout parameters
    pub fn new(w: Window, version: &'static str) -> Screen {
        let max_y = w.getmaxy();
        let prompt = max_y - PROMPT_HEIGHT;
        let status = prompt - STATUS_HEIGHT;
        let command = status - COMMAND_HEIGHT;
        let board = command - BOARD_HEIGHT;
        let title = board - TITLE_HEIGHT;
        Screen {
            w,
            version,
            prompt,
            status,
            command,
            title,
        }
    }

    /// Play games stating with games of the given size
    pub fn play(&mut self, nranks: usize) {
        let mut b = Board::new(nranks);

        loop {
            // Play one game
            let status = self.play_one_game(&mut b);
            self.clear_status();
            match status {
                Win => self.w.addstr("You won!"),
                Lose => self.w.addstr("You lose."),
            };
            // Ask what to do next
            self.clear_prompt();
            self.w.addstr(
                "Press space to play again, \
                 x to exit, or r to change game size. ",
            );
            loop {
                match self.w.getch() {
                    'x' => return,
                    ' ' => break,
                    'r' => match self.resize() {
                        Rank(rank) => {
                            b = Board::new(rank);
                            break;
                        }
                        Quit => return,
                        Again => break,
                    },
                    _ => continue, // Invalid response, try again
                }
            }
            b.clear(); // Make board ready for next deal
        }
    }

    fn play_one_game(&mut self, b: &mut Board) -> Ans {
        b.deal();
        self.show_game(b);
        loop {
            if b.is_done() {
                return Win;
            }
            if self.pick_up_card(b) {
                return Lose;
            }
        }
    }

    // Display a game

    fn show_game(&mut self, b: &Board) {
        self.w.clear();
        // Title
        self.w.mov(self.title, STACK_INDENT);
        self.w.addstr("Streets and Alleys");
        // Board
        self.show_board(b);
        // Commands
        self.w.mov(self.command, 0);
        self.w.addstr("Commands:");
        for i in -1..STACKS as i32 {
            self.goto_stack_top(i, 0);
            self.w.addch(i2char(i + 1));
            self.w.addch(',');
        }
        self.goto_stack_top(8, 0);
        self.w.addstr("q, or ?.");
        // Status
        self.w.mov(self.status, 0);
        self.w.addstr("Status:");
        self.clear_status();
        self.w.addstr("Fresh display.  Type ? for help.");
        // Prompt
        self.w.mov(self.prompt, 0);
        self.w.addstr("Prompt:");
    }

    fn show_board(&mut self, b: &Board) {
        // Foundations
        for i in 0..SUITS {
            self.show_foundation(b, i);
        }
        // Stacks
        for i in 0..STACKS {
            let mut j = 1;
            for c in b.stack_iter(i) {
                self.goto_stack_top(i as i32, j);
                self.show_card(*c);
                j = j + 1;
            }
        }
    }

    fn show_foundation(&mut self, b: &Board, i: usize) {
        self.goto_foundation(i as i32);
        self.show_card(b.foundation_ref(i));
    }

    fn goto_foundation(&mut self, i: i32) {
        self.goto_stack_top(-1, 2 * (i + 1));
    }

    fn goto_stack_top(&mut self, p: i32, h: i32) {
        self.w
            .mov(self.command - h, STACK_INDENT + CARD_SIZE * (p + 1));
    }

    fn erase_top_of_stack(&mut self, b: &Board, p: usize) {
        self.goto_stack_top(p as i32, b.stack_len(p) as i32);
        self.w.addstr("  ");
    }

    fn show_top_of_stack(&mut self, b: &Board, p: usize) {
        self.goto_stack_top(p as i32, b.stack_len(p) as i32);
        self.show_card(b.last_card(p).expect("no card to show on stack"));
    }

    fn show_card(&mut self, c: Card) {
        self.w.addch(show_suit(c));
        self.w.addch(show_rank(c));
    }

    // Clear status and prompt

    fn clear_status(&mut self) {
        self.w.mov(self.status, STACK_INDENT);
        self.w.clrtoeol();
    }

    fn clear_prompt(&mut self) {
        self.w.mov(self.prompt, STACK_INDENT);
        self.w.clrtoeol();
    }

    // Read and process one move.  Return true to quit.

    fn pick_up_card(&mut self, b: &mut Board) -> bool {
        self.clear_prompt();
        self.w.addstr("Move from stack ");
        let from = self.get_cmd();
        match from {
            '1' => self.place_card(b, from),
            '2' => self.place_card(b, from),
            '3' => self.place_card(b, from),
            '4' => self.place_card(b, from),
            '5' => self.place_card(b, from),
            '6' => self.place_card(b, from),
            '7' => self.place_card(b, from),
            '8' => self.place_card(b, from),
            'q' => return true,
            '?' => return self.help(b),
            _ => {
                self.clear_status();
                self.w.addstr("Bad input.  Type ? for help.");
                return false;
            }
        }
    }

    // Get place to put card

    fn place_card(&mut self, b: &mut Board, from: char) -> bool {
        let s = char2u(from) - 1; // Stack with picked up card
        match b.last_card(s) {
            None => {
                // Stack has no cards!
                self.clear_status();
                self.w.addstr("There is no card in stack ");
                self.w.addch(from);
                self.w.addch('.');
                return false;
            }
            Some(c) => {
                self.clear_prompt();
                self.w.addstr("Move ");
                self.show_card(c);
                self.w.addstr(" from stack ");
                self.w.addch(from);
                self.w.addstr(" to ");
                let to = self.get_cmd();
                match to {
                    // Move card to destination
                    '0' => return self.move_to_foundation(b, s, c),
                    '1' => return self.move_to_stack(b, from, s, c, to),
                    '2' => return self.move_to_stack(b, from, s, c, to),
                    '3' => return self.move_to_stack(b, from, s, c, to),
                    '4' => return self.move_to_stack(b, from, s, c, to),
                    '5' => return self.move_to_stack(b, from, s, c, to),
                    '6' => return self.move_to_stack(b, from, s, c, to),
                    '7' => return self.move_to_stack(b, from, s, c, to),
                    '8' => return self.move_to_stack(b, from, s, c, to),
                    'q' => return true,
                    '?' => return self.help(b),
                    _ => {
                        self.clear_status();
                        self.w.addstr("Bad input.  Type ? for help.");
                        return false;
                    }
                }
            }
        }
    }

    // Implement aliases for commands
    fn get_cmd(&self) -> char {
        match self.w.getch() {
            ' ' => return '0', // Aliases for use when there
            'j' => return '1', // is no numeric keyad.
            'k' => return '2',
            'l' => return '3',
            ';' => return '4',
            'u' => return '5',
            'i' => return '6',
            'o' => return '7',
            'p' => return '8',
            c => return c,
        }
    }

    fn move_to_foundation(&mut self, b: &mut Board, s: usize, c: Card) -> bool {
        self.show_card(c);
        let to = card2suit(c);
        if c == SUITS + b.foundation_ref(to) {
            self.erase_top_of_stack(b, s);
            b.pop_card(s);
            b.foundation_set(to, c);
            self.show_foundation(b, to);
            self.clear_status();
            self.w.addstr("The ");
            self.show_card(c);
            self.w.addstr(" was");
        } else {
            // Cannot move card to foundation
            self.clear_status();
            self.w.addstr("The ");
            self.show_card(c);
            self.w.addstr(" cannot be");
        }
        self.w.addstr(" moved to the foundation.");
        false
    }

    fn move_to_stack(&mut self, b: &mut Board, from: char, s: usize, c: Card, to: char) -> bool {
        let t = char2u(to) - 1;
        // Can card be moved to this stack?
        let can_move = match b.last_card(t) {
            None => true,
            Some(d) => card2rank(d) == 1 + card2rank(c),
        };
        if can_move {
            self.erase_top_of_stack(b, s);
            b.pop_card(s);
            b.push_card(t, c);
            self.show_top_of_stack(b, t);
            self.clear_status();
            self.w.addstr("Moved the ");
            self.show_card(c);
        } else {
            self.clear_status();
            self.w.addstr("The ");
            self.show_card(c);
            self.w.addstr(" cannot be moved");
        }
        self.w.addstr(" from stack ");
        self.w.addch(from);
        self.w.addstr(" to stack ");
        self.w.addch(to);
        self.w.addstr(".");
        false
    }

    // Help

    fn help(&mut self, b: &Board) -> bool {
        self.w.clear();
        self.w.addstr("       Streets and Alleys version ");
        self.w.addstr(self.version);
        self.w.addstr(
            "

There are eight stacks of cards and a foundation for each suit.  A
card may be moved from the top of a stack to its foundation or to
the top of another stack.  The object of the game is to order the
cards in each stack so that each card is covered only by cards of
lesser rank. The ace has the smallest rank and the king has the
greatest rank.

A card may be moved to its foundation when the card's predecessor of
the same suit is there.  A card may be moved to a stack when the top
card of the stack has rank one greater than the card being moved.  A
card can always be moved to an empty stack.

Commands:                              Command Aliases:

  0    Select a foundation.              <space> = 0,
  1-8  Select a stack.                   j = 1, k = 2, l = 3, ; = 4,
  q    Quit the game.                    u = 5, i = 6, o = 7, p = 8.
  r    Restore a game from a file.
  s    Save a game in a file.
  ?    Print this help and then refresh screen.",
        );
        self.w.mov(self.prompt, 0);
        self.w.addstr("Type space for more about the program. ");
        if self.w.getch() == ' ' {
            self.show_auth();
        }
        self.show_game(b);
        false
    }

    // Give additional hints and show author

    fn show_auth(&mut self) {
        self.w.clear();
        self.w.addstr(
            "The program normally uses 52 cards or 13 ranks.  A full sized game is
quite difficult, so beginners should play smaller games.  The number
of ranks used in a game can be selected by quitting out of the current
game and typing r at the restart game prompt.  Alternatively, the
program can be given a command line argument specifying the number of
ranks to be used.



Streets and Alleys version ",
        );
        self.w.addstr(self.version);
        self.w.addstr(
            " was written by John D. Ramsdell.

Copyright (c) 2019 John D. Ramsdell
Permission to use, copy, modify, and distribute this software and
its documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies.  John
Ramsdell makes no representations about the suitability of this
software for any purpose.  It is provided \"as is\" without express or
implied warranty.",
        );
        self.w.mov(self.prompt, 0);
        self.w.addstr("Type any character to continue the game. ");
        self.w.getch();
    }

    // Change the number of ranks used in a game

    // This view just needs a status and a prompt field

    fn resize(&mut self) -> Resize {
        self.w.clear();
        self.w.mov(self.title, STACK_INDENT);
        self.w.addstr("Streets and Alleys");
        self.w.mov(self.status, 0);
        self.w.addstr("Status:");
        self.w.mov(self.prompt, 0);
        self.w.addstr("Prompt:");
        loop {
            self.clear_status();
            self.w
                .addstr("Changing the number of ranks used in a game.");
            self.clear_prompt();
            self.w.addstr(
                "Press one of 5,..., 9, t, j, q, k \
                 to select the largest rank. ",
            );
            match self.w.getch() {
                '5' => return Rank(5),
                '6' => return Rank(6),
                '7' => return Rank(7),
                '8' => return Rank(8),
                '9' => return Rank(9),
                't' => return Rank(10),
                'j' => return Rank(11),
                'q' => return Rank(12),
                'k' => return Rank(13),
                _ => (),
            }
            self.clear_status();
            self.w.addstr("Bad Input.");
            self.clear_prompt();
            self.w.addstr(
                "Type space to try again, \
                 x to exit program, others play game. ",
            );
            match self.w.getch() {
                ' ' => continue,
                'x' => return Quit,
                _ => return Again,
            }
        }
    }
}

// Translate between ints and chars

fn i2char(i: i32) -> char {
    match i {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => panic!("Bad integer for i2char"),
    }
}

fn char2u(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Bad character for char2u"),
    }
}
