#![feature(test)]

//! Implementation of the Monte Carlo Tree Search algorithm.
//!
//! This implementation follows closely the terminology introduced in [1]
//! and implements the basic algorithm described in section 3.
//! To use this module you need to implement two things: a `Game` trait and
//! a matching `Move` trait that describes possible actions in your `Game`.
//!
//! [1] A Survey of Monte Carlo Tree Search Methods

extern crate test;
extern crate rand;

pub mod minigame;
pub mod tictactoe;
pub mod mcts;
mod utils;
