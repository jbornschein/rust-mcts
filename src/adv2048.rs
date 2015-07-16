
use std::fmt;

use mcts::{GameAction, Game};
use utils::choose_random;

pub const WIDTH: usize = 4;
pub const HEIGHT: usize = 4;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Possible player moves for the 2048 game.
///
/// One of Up, Down. Left or Right.
pub enum Direction {
    Up, Down, Left, Right
}

/// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Possible Spawn possible_actions
pub type SpawnPosition = usize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Game actions for Adversarial2048.
///
/// This contains eiher a player move (Up, Down, Left, or Right) or a tile
/// spawning pseudo move. Tile spawning is modeled as an (adversarial) move
/// so that we can use straight forward MCTS without any explicit
/// determinization to get rid of the randomness in the game.
/// Determinization would require us to use ensambling to evaluate more than
/// one possible future.
pub enum Action {
    PlayerAction(Direction),
    SpawnAction(SpawnPosition),
}

impl GameAction for Action {}


#[derive(Clone)]
/// Implementation of the 2048 game mechanics.
///
/// After initialization the game receives an alternating sequence of
/// PlayerAction and SpawnAction.
pub struct Adversarial2048 {
    board: [u16; WIDTH*HEIGHT],
    last_action: Option<Action>,
    pub score: f32,
    pub moves: usize,
}

impl Adversarial2048 {
    /// Create a new empty game
    pub fn empty() -> Adversarial2048 {
        Adversarial2048 {
            score: 0.0,
            moves: 0,
            board: [0; WIDTH*HEIGHT],
            last_action: None,
        }
    }

    // Create a new game with two random two's in it.
    pub fn new() -> Adversarial2048 {
        let mut game = Adversarial2048::empty();
        game.random_spawn();
        game.random_spawn();
        game
    }

    #[inline]
    ///
    pub fn get_tile(&self, row: usize, col: usize) -> u16 {
        let idx = row * WIDTH + col;
        self.board[idx]
    }

    #[inline]
    ///
    pub fn set_tile(&mut self, row: usize, col: usize, num: u16) {
        let idx = row * WIDTH + col;
        self.board[idx] = num;
    }

    /// Merge a vector according to the 2048 rules to the left.
    fn merge_vec(vec: &Vec<u16>) -> (Vec<u16>, f32, bool) {
        let mut points = 0.0;

        // first, remove zeros
        let orig_len = vec.len();
        let filtered_vec = vec.iter()
                    .filter(|&t| *t > 0)
                    .map(|&t| t)
                    .collect::<Vec<u16>>();

        // Remove duplicates
        let mut merged = Vec::with_capacity(HEIGHT);
        let mut next = 0;
        for t in filtered_vec {
            if t == next {
                merged.push(2*t);
                next = 0;
                points += 2.* (t as f32);
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

        // Make sure we keep the original length and notice any changes
        let changed = orig_len != merged.len();
        for _ in 0..(orig_len-merged.len()) {
            merged.push(0);
        }
        (merged, points, changed)
    }


    /// Shift and merge the board in the given direction
    fn shift_and_merge(board: [u16; WIDTH*HEIGHT], direction: Direction) -> ([u16; WIDTH*HEIGHT], Option<f32>) {
        let (start, ostride, istride) = match direction {
            Direction::Up    => ( 0,  1,  4),
            Direction::Down  => (12,  1, -4),
            Direction::Left  => ( 0,  4,  1),
            Direction::Right => (15, -4, -1),
        };

        let start = start as isize;
        let ostride = ostride as isize;
        let istride = istride as isize;
        assert!(HEIGHT == WIDTH);

        let mut new_board = [0; WIDTH*HEIGHT];
        let mut all_points = 0.0;    //  points we accumulate
        let mut any_changed = false;  // did any of the vectors change?

        for outer in 0..(HEIGHT as isize) {
            let mut vec = Vec::with_capacity(HEIGHT);
            for inner in 0..(HEIGHT as isize) {
                let idx = start + outer*ostride + inner*istride;
                vec.push(board[idx as usize]);
            }

            let (merged_vec, points, changed) = Adversarial2048::merge_vec(&vec);
            all_points += points;
            any_changed |= changed;

            for inner in 0..(HEIGHT as isize) {
                let idx = start + outer*ostride + inner*istride;
                new_board[idx as usize] = merged_vec[inner as usize];
            }
        }
        if any_changed {
            (new_board, Some(all_points))
        } else {
            (new_board, None)
        }
    }

    /// Check whether the board is full.
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

    /// Place a tile into some random spot.
    pub fn random_spawn(&mut self) {
        assert!(!self.board_full());

        let possible_spawns = self.allowed_spawn_actions();
        let spawn = choose_random(&possible_spawns);

        self.perform_spawn_action(*spawn);
        self.last_action = Some(Action::SpawnAction(*spawn));
    }

    #[inline]
    pub fn allowed_player_actions(&self) -> Vec<Direction> {
        let directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        directions.iter().map(|t| *t)
            .filter(|&dir| {
                let (_, points) = Adversarial2048::shift_and_merge(self.board, dir);
                match points {
                    Some(_) => true,
                    None => false
                }})
            .collect()
    }

    #[inline]
    pub fn allowed_spawn_actions(& self) -> Vec<SpawnPosition> {
        self.board.iter()
            .enumerate()
            .filter(|&(_, &a)| a == 0)
            .map(|(idx, _)| idx as SpawnPosition)
            .collect()
    }

    #[inline]
    pub fn perform_spawn_action(&mut self, position: SpawnPosition) {
        let idx = position as usize;
        assert!(self.board[idx] == 0);
        self.board[idx] = 2;
    }

    #[inline]
    pub fn perform_player_action(&mut self, dir: Direction) {
        let (new_board, points) = Adversarial2048::shift_and_merge(self.board, dir);
        self.score += points.expect("Illegal move");
        self.moves += 1;
        self.board = new_board;
    }

}

impl Game<Action> for Adversarial2048 {

    /// Return a list with all allowed actions given the current game state.
    fn allowed_actions(&self) -> Vec<Action> {
        match self.last_action {
            Some(Action::PlayerAction(_)) =>
                self.allowed_spawn_actions().iter().map(|&dir| Action::SpawnAction(dir)).collect(),
            None | Some(Action::SpawnAction(_)) =>
                self.allowed_player_actions().iter().map(|&dir| Action::PlayerAction(dir)).collect(),
        }
    }

    /// Change the current game state according to the given action.
    fn make_move(&mut self, action: &Action) {
        // XXX assert we are performing alternating actions
        match *action {
            Action::PlayerAction(direction) => self.perform_player_action(direction),
            Action::SpawnAction(spawn) => self.perform_spawn_action(spawn),
        }
        self.last_action = Some(*action);
    }

    /// Reward for the player when reaching the current game state.
    fn reward(&self) -> f32 {
        self.score / 1000.
    }

    /// Derterminize the game
    fn set_rng_seed(&mut self, _: u32) { }
}

impl fmt::Display for Adversarial2048 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // XXX could be much nicer XXX
        try!(writeln!(f, "Moves={} Score={}:", self.moves, self.score));
        for _ in 0..WIDTH {
            try!(write!(f, "|{: ^5}", "-----"));
        }
        try!(f.write_str("|"));
        for row in 0..HEIGHT {
            try!(f.write_str("\n"));
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
            try!(f.write_str("|"));
        }
        f.write_str("")
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use test::Bencher;

    use mcts::*;
    use adv2048::*;

    #[test]
    fn test_new() {
        let game = Adversarial2048::new();

        assert_eq!(game.reward(), 0.);
    }

    #[test]
    fn test_display() {
        let coords = vec![(0, 1, 2), (2, 2, 4), (3, 1, 2048)];

        // Set given tiles
        let mut game = Adversarial2048::new();
        for (row, col, num) in coords.clone() {
            game.set_tile(row, col, num);
        }

        println!("{}", game);
    }

    #[test]
    fn test_setget_tile() {
        let mut game = Adversarial2048::new();

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
        let mut game = Adversarial2048::empty();

        for _ in 0..WIDTH*HEIGHT {
            assert!(!game.board_full());
            game.random_spawn();
        }
        assert!(game.board_full());
    }


    #[test]
    fn test_playout() {
        let game = Adversarial2048::new();
        let final_game = playout(&game);
        println!("{}", final_game);
    }

    /*
    #[test]
    fn test_mcts() {
        let game = Adversarial2048::new();
        let mut mcts = MCTS::new(&game, 5);

        mcts.search(25, 1.);
        let action = mcts.best_action();
        action.expect("should give some action");
    } */

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
