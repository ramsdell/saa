//! # A Streets and Alleys board

//! This module defines a card and its operations.  It then describes
//! an allowed configuration of the cards that make up a board.

// Copyright (c) 2019 John D. Ramsdell
// Permission to use, copy, modify, and distribute this software and its
// documentation for any purpose and without fee is hereby granted,
// provided that the above copyright notice appear in all copies.  John
// Ramsdell makes no representations about the suitability of this
// software for any purpose.  It is provided "as is" without express or
// implied warranty.

extern crate rand;

use rand::Rng; // For shuffling a deck of cards

/// Number of suits (4)
pub const SUITS: usize = 4;

/// Number of stacks of cards
pub const STACKS: usize = 2 * SUITS;

/// Maximum number of ranks (13)
pub const MAX_RANKS: usize = 13;

/// A card is an unsigned integer less than 56.  Cards less than 4 are
/// blank cards used to represent the absence of a card in a given suit.
/// For all cards c, c == card2suit(c) * card2rank(c).
pub type Card = usize;

/// The suit associated with a card
///
/// The suit associated with a card is an unsigned integer less than
/// 4.
pub fn card2suit(c: Card) -> usize {
    c % SUITS
}

/// The rank associated with a card
///
/// The rank associated with a card is an unsigned integer less than
/// 14.  A king is has rank 13, and a blank card has rank 0.
pub fn card2rank(c: Card) -> usize {
    c / SUITS
}

/// Show suit as a char
pub fn show_suit(c: Card) -> char {
    match card2suit(c) {
        0 => 'C',
        1 => 'D',
        2 => 'H',
        3 => 'S',
        _ => panic!("bad suit"),
    }
}

// Wish I could use this, but w.addch munges it.
//
// pub fn show_suit(c: Card) -> char {
//     match card2suit(c) {
//         0 => '♣', // U+2663
//         1 => '♦', // U+2666
//         2 => '♥', // U+2665
//         3 => '♠', // U+2660
//         _ => panic!("bad suit"),
//     }
// }

/// Show rank as a char
pub fn show_rank(c: Card) -> char {
    match card2rank(c) {
        0 => '-',
        1 => 'A',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'T',
        11 => 'J',
        12 => 'Q',
        13 => 'K',
        _ => panic!("bad rank"),
    }
}

/// A Streets and Alleys board
pub struct Board {
    ranks: usize,
    stack: [Vec<Card>; STACKS],
    foundation: [Card; SUITS],
}

impl Board {
    /// Create a fresh board using a given ranks worth of cards
    ///
    /// The board has 8 stacks and 4 foundations.
    pub fn new(ranks: usize) -> Board {
        if ranks <= MAX_RANKS {
            let mut b = Board {
                ranks,
                stack: [
                    Vec::with_capacity(ranks),
                    Vec::with_capacity(ranks),
                    Vec::with_capacity(ranks),
                    Vec::with_capacity(ranks),
                    Vec::with_capacity(ranks),
                    Vec::with_capacity(ranks),
                    Vec::with_capacity(ranks),
                    Vec::with_capacity(ranks),
                ],
                foundation: [0; SUITS],
            };
            for i in 0..SUITS {
                // Add blank cards
                b.foundation[i] = i;
            }
            b
        } else {
            panic!("bad number of ranks when creating a board")
        }
    }

    /// Is the game done?
    pub fn is_done(&self) -> bool {
        for s in self.stack.iter() {
            if !is_stack_done(s) {
                return false;
            }
        }
        true
    }

    /// Add a card to the top of a stack
    pub fn push_card(&mut self, s: usize, c: Card) {
        self.stack[s].push(c)
    }

    /// Return the length of a stack
    pub fn stack_len(&self, s: usize) -> usize {
        self.stack[s].len()
    }

    /// Remove a card from the top of a stack (None if empty)
    pub fn pop_card(&mut self, s: usize) -> Option<Card> {
        self.stack[s].pop()
    }

    /// Return the card on the top of a stack (None if empty)
    pub fn last_card(&self, s: usize) -> Option<Card> {
        match self.stack[s].last() {
            Some(c) => Some(*c),
            None => None,
        }
    }

    /// Return an iterator for the cards in a stack
    pub fn stack_iter(&self, s: usize) -> std::slice::Iter<usize> {
        self.stack[s].iter()
    }

    /// Return the top card in a foundation
    pub fn foundation_ref(&self, r: usize) -> Card {
        self.foundation[r]
    }

    /// Set the top card in a foundation
    pub fn foundation_set(&mut self, r: usize, c: Card) {
        self.foundation[r] = c
    }

    /// Clear a board making it ready for a new deal
    pub fn clear(&mut self) {
        for s in &mut self.stack {
            s.clear();
        }
        for i in 0..SUITS {
            self.foundation[i] = i;
        }
    }

    /// Create and Shuffle the cards in a board
    ///
    /// Assumes the board is freshly made or has been cleared.
    pub fn deal(&mut self) {
        let size = self.ranks * SUITS;
        let mut deck: Vec<Card> = Vec::with_capacity(size);

        // Create cards
        for i in 0..size {
            deck.push(i + SUITS);
        }

        // Shuffle
        for i in 0..size {
            let j = rand::thread_rng().gen_range(0, size);
            deck.swap(i, j);
        }

        // Deal cards
        for (i, c) in deck.iter().enumerate() {
            self.push_card(i % STACKS, *c);
        }
    }
}

// A stack is done if no card is out of order.
fn is_stack_done(s: &[usize]) -> bool {
    match s.len() {
        0 => true,
        1 => true,
        n => {
            for i in 1..n {
                if card2rank(s[i - 1]) <= card2rank(s[i]) {
                    return false;
                }
            }
            true
        }
    }
}
