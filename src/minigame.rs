//!
//! Implementation of a minimal game
//!

use std::fmt;
use mcts::{Action, Game};

const WINNING_SUM :u32 = 11;
const DRAW_MIN :u32 = 3;
const DRAW_MAX :u32 = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MiniMove {
    add: u32
}
impl Action for MiniMove {}

#[derive(Debug, Clone)]
pub struct MiniGame {
    sum: u32
}

impl MiniGame {
    pub fn new() -> MiniGame {
        return MiniGame {sum: 0};
    }
}

impl fmt::Display for MiniGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "sum={}", self.sum)
    }
}

impl Game<MiniMove> for MiniGame {
    fn allowed_moves(&self) -> Vec<MiniMove> {
        let mut moves = Vec::new();

        if self.sum < WINNING_SUM {
            for add in DRAW_MIN..DRAW_MAX {
                moves.push(MiniMove{add: add});
            }
        }
        moves
    }

    fn reward(&self) -> f32 {
             if self.sum <  WINNING_SUM {  0. }
        else if self.sum == WINNING_SUM {  1. }
        else if self.sum >  WINNING_SUM { -1. }
        else { panic!("Huh?") }
    }

    fn make_move(&mut self, a_move: &MiniMove) {
        self.sum = self.sum + a_move.add;
    }
}
