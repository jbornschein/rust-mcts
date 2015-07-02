
extern crate mcts;

use mcts::mcts::{Game, MCTS};
use mcts::twofortyeight::TwoFortyEight;

fn main() {
    let ensamble_size = 10;
    let n_samples = 100;

    // Create a game and a MCTS solver
    let mut game = TwoFortyEight::new();
    let mut mcts = MCTS::new(&game, ensamble_size);

    loop {
        mcts.search(n_samples, 1.);
        let action = mcts.best_action();

        match action {
            Some(action) => {
                game.make_move(&action);
                mcts.advance_game(&game);
                println!("{:?}\n{}", action, game);
            },
            None => break
        }
    }
}
