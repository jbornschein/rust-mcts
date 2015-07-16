
extern crate argparse;
extern crate time;
extern crate mcts;

use argparse::{ArgumentParser, StoreTrue, Store};

use mcts::mcts::{Game, MCTS};
use mcts::adv2048::Adversarial2048;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut repeats = 1;
    let mut verbose = false;
    let mut time_per_move = 1.0;
    let mut ensemble_size = 1;

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
        ap.refer(&mut repeats)
            .add_option(&["--repeat", "-r"], Store,
            "Numer of games to play.");
        ap.parse_args_or_exit();
    }

    println!("Playing 2048\n");
    println!("Time per move: {} s", time_per_move);
    println!("Ensemble size: {}", ensemble_size);
    println!("");

    // Summary statistics
    let mut sum_moves = 0.;
    let mut sum_score = 0.;
    let mut sum_moves_sq = 0.;
    let mut sum_score_sq = 0.;

    // Play repeat games in total...
    for _ in 0..repeats {
        // Create a game and a MCTS solver
        let mut game = Adversarial2048::new();
        let mut mcts = MCTS::new(&game, ensemble_size);

        println!("{}", game);
        loop {
            mcts.search_time(time_per_move, 1.0);

            if verbose {
                println!("{:?}", mcts.tree_statistics());
            }

            let action = mcts.best_action();
            match action {
                Some(action) => {
                    game.make_move(&action);
                    game.random_spawn();
                    mcts.advance_game(&game);
                    println!("\n... moved {:?}: {}", action, game);
                },
                None => break
            }
        }

        // Update summary statistics
        sum_moves += game.moves as f32;
        sum_score += game.score as f32;
        sum_moves_sq += (game.moves * game.moves) as f32;
        sum_score_sq += (game.score * game.score) as f32;
    }

    if repeats > 1 {
        let frepeats = repeats as f32;
        let avg_moves = sum_moves / frepeats;
        let avg_score = sum_score / frepeats;
        let avg_moves_err = ((sum_moves_sq - sum_moves.powi(2)) / ((frepeats-1.) * frepeats)).sqrt();
        let avg_score_err = ((sum_score_sq - sum_score.powi(2)) / ((frepeats-1.) * frepeats)).sqrt();

        println!("Played {} games.", repeats);
        println!("  Average # moves: {} (+/- {})", avg_moves, avg_moves_err);
        println!("  Average Score:   {} (+/- {})", avg_score, avg_score_err);
    }
}
