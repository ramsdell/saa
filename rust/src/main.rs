// Command line processing for Streets and Alleys

// Copyright (c) 2019 John D. Ramsdell
// Permission to use, copy, modify, and distribute this software and its
// documentation for any purpose and without fee is hereby granted,
// provided that the above copyright notice appear in all copies.  John
// Ramsdell makes no representations about the suitability of this
// software for any purpose.  It is provided "as is" without express or
// implied warranty.

extern crate saa;

use std::env;

const DEFAULT_RANKS: usize = 11;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        usage(&args[0]);
    } else if args.len() == 1 {
        saa::run(DEFAULT_RANKS, VERSION);
    } else {
        // args.len() == 2
        match usize::from_str_radix(&args[1], 10) {
            Ok(ranks) => {
                if ranks >= 5 && ranks <= 13 {
                    saa::run(ranks, VERSION)
                } else {
                    usage(&args[0])
                }
            }
            Err(_) => usage(&args[0]),
        }
    }
}

fn usage(prog: &str) {
    println!(
        "       Streets and Alleys version {}

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
  ?    Print this help and then refresh screen.

Usage: {} [number_of_ranks].
The number of ranks must be between 5 and 13.
The default number of ranks is {}.",
        VERSION, prog, DEFAULT_RANKS
    );
}
