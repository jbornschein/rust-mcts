
extern crate mcts;
extern crate time;

use time::now;

use mcts::mcts::{Game, MCTS};
use mcts::twofortyeight::TwoFortyEight;

fn main() {
    let ensamble_size = 15;
    let n_samples = 100;
    let ms_per_move = 2000;

    // Create a game and a MCTS solver
    let mut game = TwoFortyEight::new();
    let mut mcts = MCTS::new(&game, ensamble_size);

    loop {

        let t0 = time::now();
        while (time::now()-t0).num_milliseconds() < ms_per_move {
            mcts.search(n_samples, 1.);
        };

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
