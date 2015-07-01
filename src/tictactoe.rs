//!
//! Implementation of the TicTacToe board game
//!

use std::fmt;
use mcts::{Action, Game};


/// Represent a player.
///
/// ToDo: Should this rather be Option<Cross/Cicle> and
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    None, Cross, Circle
}

#[allow(dead_code)]
impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::Cross  => Player::Circle,
            Player::Circle => Player::Cross,
            Player::None   => Player::None,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::None   => write!(f, "{}", ' '),
            Player::Cross  => write!(f, "{}", 'X'),
            Player::Circle => write!(f, "{}", 'O'),
        }
    }
}

/// Represent a move in the TicTacToe game
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub x: u8,
    pub y: u8,
}

impl Action for Move {}

/// Represent the game status (who has won?)
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Ongoing,
    Won(Player)
}

/// TicTacToe board game.
#[derive(Debug, Clone)]
pub struct TicTacToe {
    board: [Player; 9],
    next_player: Player,
}

#[allow(dead_code)]
impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe { board: [Player::None; 9], next_player: Player::Cross }
    }

    /// Direct access to the board
    fn get_field(&self, y: u8, x: u8) -> Player {
        //assert!(y < 3);
        //assert!(x < 3);

        let idx = 3*y + x;
        self.board[idx as usize]
    }

    /// Direct access to the board
    fn set_field(&mut self, y: u8, x: u8, what: Player) {
        assert!(y < 3);
        assert!(x < 3);

        let idx = 3*y + x;
        self.board[idx as usize] = what;
    }

    ///
    pub fn game_status(&self) -> GameStatus {
        // Check rows
        for row in 0..3 {
            let this_row_same = three_same(
                self.get_field(row, 0),
                self.get_field(row, 1),
                self.get_field(row, 2));

            match this_row_same {
                Player::Cross | Player::Circle => return GameStatus::Won(this_row_same),
                Player::None => (),
            }
        }

        // Check columns
        for col in 0..3 {
            let this_row_same = three_same(
                self.get_field(0, col),
                self.get_field(1, col),
                self.get_field(2, col));

            match this_row_same {
                Player::Cross | Player::Circle => return GameStatus::Won(this_row_same),
                Player::None => (),
            }
        }

        for row in 0..3 {
            for col in 0..3 {
                match self.get_field(row, col) {
                    Player::None => return GameStatus::Ongoing,
                    _ => (),
                }
            }
        }
        GameStatus::Won(Player::None)
    }

}

impl Game<Move> for TicTacToe {
    /// Return a vector with all allowed moves
    fn allowed_actions(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for y in 0..3 {
            for x in 0..3 {
                match self.get_field(y, x) {
                    Player::None => moves.push(Move{x:x, y:y}),
                    _ => continue,
                }
            }
        }
        moves
    }

    /// Calculate rewar for current game situation
    fn reward(&self) -> f32 {
        match self.game_status() {
            GameStatus::Won(Player::Cross)  =>  1.,
            GameStatus::Won(Player::Circle) => -1.,
            _  =>  0.,
        }
    }

    /// Make a move
    fn make_move(&mut self, a_move: &Move) {
        let what = self.next_player;
        self.set_field(a_move.y, a_move.x, what);
        self.next_player = self.next_player.other();
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}|{}|{}\n-+-+-\n{}|{}|{}\n-+-+-\n{}|{}|{}",
                        self.board[0], self.board[1], self.board[2],
                        self.board[3], self.board[4], self.board[5],
                        self.board[6], self.board[7], self.board[8])
    }
}


/// Simply check if three Players are the same.
///
/// Return Player::Cross or Player::Circle if all three are
/// the same; Player::None otherwise.
fn three_same(f1: Player, f2: Player, f3: Player) -> Player {
    if (f1 == f2) && (f2 == f3) {
        return f1;
    }
    Player::None
}


/////////////////////////////////////////////////////////////////////////////
// Unittests

/*
#[test]
fn player_printing() {
    let p = Player::None;
    println!("Debug print: {:?}", p);
    println!("Display print: {}", p);
}

#[test]
fn tictactoe_printing() {
    let mut t1 = TicTacToe::new();

    println!("{}", t1);
    println!("{:?}", t1);

    t1.set_field(1, 0, Player::Cross);
    t1.set_field(1, 1, Player::Circle);
    t1.set_field(1, 2, Player::Cross);

    println!("{}", t1);
    println!("{:?}", t1);
}

#[test]
fn game_status() {
    let moves = vec![
        Move{x: 1, y: 0},
        Move{x: 2, y: 0},
        Move{x: 1, y: 1},
        Move{x: 2, y: 1},
        Move{x: 1, y: 2},
    ];

    let mut ttt = TicTacToe::new();
    for m in moves {
        println!("{:?}", ttt.game_status());
        ttt.make_move(m);
    }
    println!("{:?}", ttt.game_status());
}


#[test]
fn tictactoe() {
    let mut t1 = TicTacToe::new();

    assert_eq!(t1.game_status(), GameStatus::Ongoing);

    t1.make_move(Move{x: 1, y: 1});
    t1.make_move(Move{x: 1, y: 2});

    assert_eq!(t1.game_status(), GameStatus::Ongoing);

    let moves = t1.allowed_actions();
    println!("{:?}", moves);
}
*/
