//!
//! Implementation of a simple dummy game.
//!
//! The goal of the agent is to advance it's points to exactly 11.
//! In each turn the agent can choose to add a number between 3 and 5; when 
//! the sum is below 11 the agent can take another turn; if it is exactly 11
//! the agent wins and gains a reward of 1; if it is above 11 the agent 
//! looses and gains a final reward of -1.
//!
//! Potential, equally good sequences of action which let the agent win are thus
//! e.g. [3, 3, 3, 2]; [5, 3, 3] or [4, 4, 3].
//!

use std::fmt;
use mcts::{GameAction, Game};

const WINNING_SUM :u32 = 11;
const DRAW_MIN :u32 = 3;
const DRAW_MAX :u32 = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Action {
    add: u32
}
impl GameAction for Action {}

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

impl Game<Action> for MiniGame {
    fn allowed_actions(&self) -> Vec<Action> {
        let mut moves = Vec::new();

        if self.sum < WINNING_SUM {
            for add in DRAW_MIN..DRAW_MAX {
                moves.push(Action{add: add});
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

    fn make_move(&mut self, a_move: &Action) {
        self.sum = self.sum + a_move.add;
    }
}
