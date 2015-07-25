#![feature(test)]

extern crate test;
extern crate mcts;



/////////////////////////////////////////////////////////////////////////////
// MCTS benchmarks
mod utils {
    use test::Bencher;
    use mcts::utils::*;

    #[bench]
    fn bench_choose_random10(b: &mut Bencher) {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        b.iter(|| choose_random(&vec))
    }
}

/////////////////////////////////////////////////////////////////////////////
// MCTS benchmarks
mod mcts_alg {
    use test::Bencher;

    use mcts::mcts::*;
    use mcts::minigame::*;

    #[bench]
    fn bench_playout(b: &mut Bencher) {
        let game = MiniGame::new();
        b.iter(|| playout(&game))
    }

    #[bench]
    fn bench_expected(b: &mut Bencher) {
        let game = MiniGame::new();
        b.iter(|| expected_reward(&game, 100))
    }

    #[bench]
    fn bench_search(b: &mut Bencher) {
        let game = MiniGame::new();
        let mut mcts = MCTS::new(&game, 1);

        b.iter(|| mcts.search(10, 1.0))
    }
}

/////////////////////////////////////////////////////////////////////////////
// Adversarial2048 benchmarks

mod adv2048 {
    use test::Bencher;

    use mcts::mcts::playout;
    use mcts::adv2048::*;

    #[bench]
    fn bench_playout(b: &mut Bencher) {
        let game = Adversarial2048::new();
        b.iter(|| playout(&game));
    }

    #[bench]
    fn bench_allowed_spawn_actions(b: &mut Bencher) {
        let mut game = Adversarial2048::new();
        for _ in 0..8 {
            game.random_spawn();
        }
        b.iter(|| game.allowed_spawn_actions());
    }

    #[bench]
    fn bench_allowed_player_actions(b: &mut Bencher) {
        let mut game = Adversarial2048::new();
        for _ in 0..8 {
            game.random_spawn();
        }
        b.iter(|| game.allowed_player_actions());
    }

    #[bench]
    fn random_spawn_until_full(b: &mut Bencher) {
        b.iter(|| {
            let mut game = Adversarial2048::new();
            while !game.board_full() {
                game.random_spawn()
            }
        })
    }

    #[bench]
    fn board_full(b: &mut Bencher) {
        let mut game = Adversarial2048::new();

        for _ in 0..(WIDTH*HEIGHT/2) {
            game.random_spawn()
        }

        b.iter(|| game.board_full())
    }
}

/////////////////////////////////////////////////////////////////////////////
// TwoFortyEight benchmarks

mod twofortyeight {
    use test::Bencher;

    use mcts::mcts::*;
    use mcts::twofortyeight::*;

    #[bench]
    fn bench_playout(b: &mut Bencher) {
        let game = TwoFortyEight::new();
        b.iter(|| playout(&game));
    }

    #[bench]
    fn bench_allowed_actions(b: &mut Bencher) {
        let game = TwoFortyEight::new();
        b.iter(|| game.allowed_actions());
    }

    #[bench]
    fn random_spawn_until_full(b: &mut Bencher) {
        b.iter(|| {
            let mut game = TwoFortyEight::new();
            while !game.board_full() {
                game.random_spawn()
            }
        })
    }

    #[bench]
    fn board_full(b: &mut Bencher) {
        let mut game = TwoFortyEight::new();

        for _ in 0..(WIDTH*HEIGHT/2) {
            game.random_spawn()
        }

        b.iter(|| game.board_full())
    }
}
