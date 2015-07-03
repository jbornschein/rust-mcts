
extern crate argparse;
extern crate mcts;
extern crate time;

use time::now;
use argparse::{ArgumentParser, StoreTrue, Store};

use mcts::mcts::{Game, MCTS};
use mcts::twofortyeight::TwoFortyEight;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut verbose = false;
    let mut time_per_move = 1.0;
    let mut ensemble_size = 10;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("2048 playing.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut time_per_move)
            .add_option(&["--time-per-second", "-t"], Store,
            "Time budget per move (in seconds)");
        ap.refer(&mut ensemble_size)
            .add_option(&["--ensemble_size", "-e"], Store,
            "Ensemble size.");
        ap.parse_args_or_exit();
    }

    let n_samples = 10;
    let ms_per_move = (time_per_move * 1000.) as i64;

    println!("Playing 2048\n");
    println!("Time per move: {} s", time_per_move);
    println!("Ensemble size: {}", ensemble_size);
    println!("");

    // Create a game and a MCTS solver
    let mut game = TwoFortyEight::new();
    let mut mcts = MCTS::new(&game, ensemble_size);
    println!("{}", game);

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
                println!("\n... moving {:?}: {}", action, game);
            },
            None => break
        }
    }
}
