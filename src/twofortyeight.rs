
use std::fmt;
use std::iter;
use rand::random;

use mcts::{GameAction, Game};


pub const WIDTH: usize = 4;
pub const HEIGHT: usize = 4;

#[derive(Debug, Clone)]
/// Implementation of the 2048 board game.
///
pub struct TwoFortyEight {
    score: f32,
    board: [u16; WIDTH*HEIGHT]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Up, Down, Left, Right
}

impl GameAction for Action {}


impl TwoFortyEight {
    /// Create a new empty game
    pub fn new() -> TwoFortyEight {
        TwoFortyEight {
            score: 0.0,
            board: [0; WIDTH*HEIGHT]
        }
    }

    ///
    pub fn merge_vec(vec: &Vec<u16>) -> Vec<u16> {

        // first, remove zeros
        let orig_len = vec.len();
        let vec = vec.iter().map(|t| *t).filter(|&t| t > 0).collect::<Vec<u16>>();

        let mut merged = Vec::new();
        let mut next = 0;
        for t in vec {
            if t == next {
                merged.push(t+next);
                next = 0;
            } else {
                if next != 0 {
                    merged.push(next);
                }
                next = t;
            }
        }
        if next != 0 {
            merged.push(next);
        }
        for _ in 0..(orig_len-merged.len()) {
            merged.push(0);
        }
        merged
    }

    ///
    pub fn get_tile(&self, row: usize, col: usize) -> u16 {
        let idx = row * WIDTH + col;
        self.board[idx]
    }

    ///
    pub fn set_tile(&mut self, row: usize, col: usize, num: u16) {
        let idx = row * WIDTH + col;
        self.board[idx] = num;
    }

    ///
    pub fn board_full(&self) -> bool {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if self.get_tile(row, col) == 0 {
                    return false;
                }
            }
        }
        true
    }

    /// Place a 2 into some random empty tile
    pub fn random_spawn(&mut self) {
        assert!(!self.board_full());

        loop {
            let row = random::<usize>() % HEIGHT;
            let col = random::<usize>() % WIDTH;
            if self.get_tile(row, col) == 0 {
                self.set_tile(row, col, 2);
                break;
            }
        }
    }

}

impl Game<Action> for TwoFortyEight {

    /// Return a list with all allowed actions given the current game state.
    fn allowed_actions(&self) -> Vec<Action> {
        vec![Action::Up, Action::Down, Action::Left, Action::Right]
    }

    /// Change the current game state according to the given action.
    fn make_move(&mut self, action: &Action) {
        start, ostride, istride, match action {
            Action::Up    => ( 0,  1,  4)
            Action::Down  => (12,  1, -4)
            Action::Left  => ( 0,  4,  1)
            Action::Right => (15, -4, -1)
        }

        assert!(HEIGHT == WIDTH);

        for i in 0..HEIGHT {
            for j in 0..HEIGHT {
                 XXX
            }
        }
    }

    /// Reward for the player when reaching the current game state.
    fn reward(&self) -> f32 {
        self.score
    }
}


impl fmt::Display for TwoFortyEight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // XXX could be much nicer XXX
        try!(writeln!(f, "score={}", self.score));
        for _ in 0..WIDTH {
            try!(write!(f, "|{: ^5}", "-----"));
        }
        try!(f.write_str("|\n"));
        for row in 0..HEIGHT {
            for _ in 0..WIDTH {
                try!(write!(f, "|{: ^5}", ""));
            }
            try!(f.write_str("|\n"));
            for col in 0..WIDTH {
                let tile =  self.get_tile(row, col);
                if tile == 0 {
                    try!(write!(f, "|{: ^5}", ""));
                } else {
                    try!(write!(f, "|{: ^5}", tile));
                }
            }
            try!(f.write_str("|\n"));
            for _ in 0..WIDTH {
                try!(write!(f, "|{: ^5}", ""));
            }
            try!(f.write_str("|\n"));
            for _ in 0..WIDTH {
                try!(write!(f, "|{: ^5}", "-----"));
            }
            try!(f.write_str("|\n"));
        }
        f.write_str("")
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use test::Bencher;

    use mcts::*;
    use twofortyeight::*;

    #[test]
    fn test_new() {
        let game = TwoFortyEight::new();

        assert_eq!(game.reward(), 0.);
    }

    #[test]
    fn test_display() {
        let coords = vec![(0, 1, 2), (2, 2, 4), (3, 1, 2048)];

        // Set given tiles
        let mut game = TwoFortyEight::new();
        for (row, col, num) in coords.clone() {
            game.set_tile(row, col, num);
        }

        println!("{}", game);
    }

    #[test]
    fn test_setget_tile() {
        let mut game = TwoFortyEight::new();

        let coords = vec![(0, 1, 2), (2, 2, 4), (3, 1, 16)];

        // Set given tiles
        for (row, col, num) in coords.clone() {
            game.set_tile(row, col, num);
        }

        // Check given tiles
        for (row, col, num) in coords.clone() {
            assert_eq!(game.get_tile(row, col), num);
        }
    }

    #[test]
    fn test_random_spawn() {
        let mut game = TwoFortyEight::new();

        for _ in 0..WIDTH*HEIGHT {
            assert!(!game.board_full());
            game.random_spawn();
        }
        assert!(game.board_full());
    }

    #[test]
    fn test_merge_vec()
    {
        let test_cases = vec![
            (vec![1, 2, 0, 0, 4], vec![1, 2, 4, 0, 0]),
            (vec![1, 2, 2, 0, 4], vec![1, 4, 4, 0, 0]),
            (vec![1, 2, 2, 2, 4], vec![1, 4, 2, 4, 0])
        ];

        for (input, should) in test_cases {
            let  output = TwoFortyEight::merge_vec(&input);
            println!("merge_vec({:?}) => {:?}  (should be {:?})", input, output, should);
        }
    }

    #[bench]
    fn bench_random_spawn(b: &mut Bencher) {
        b.iter(|| {
            let mut game = TwoFortyEight::new();
            for _ in 0..(WIDTH*HEIGHT/2) {
                game.random_spawn()
            }
        })
    }

    #[bench]
    fn bench_board_full(b: &mut Bencher) {
        let mut game = TwoFortyEight::new();

        for _ in 0..3 {
            game.random_spawn()
        }

        b.iter(|| game.board_full())
    }
}
