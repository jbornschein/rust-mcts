
use std::fmt;
use std::f32;
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

use utils::{choose_random};


/// A `Game` represets a game state.
///
/// It is important that the game behaves fully deterministic,
/// e.g. it has to produce the same game sequences
pub trait Game<A: GameAction> : Clone {

    /// Return a list with all allowed actions given the current game state.
    fn allowed_actions(&self) -> Vec<A>;

    /// Change the current game state according to the given action.
    fn make_move(&mut self, action: &A);

    /// Reward for the player when reaching the current game state.
    fn reward(&self) -> f32;

    /// Derterminize the game
    fn set_rng_seed(&mut self, seed: u32);
}

/// A `GameAction` represents a move in a game.
pub trait GameAction: Debug+Clone+Copy+Eq+Hash {}


/// Perform a random playout.
///
/// Start with an initial game state and perform random actions from
/// until a game-state is reached that does not have any `allowed_actions`.
pub fn playout<G: Game<A>, A: GameAction>(initial: &G) -> G {
    let mut game = initial.clone();

    let mut potential_moves = game.allowed_actions();
    while potential_moves.len() > 0 {
        let action = choose_random(&potential_moves).clone();
        game.make_move(&action);
        potential_moves = game.allowed_actions();
    }
    game
}

/// Calculate the expected reward based on random playouts.
pub fn expected_reward<G: Game<A>, A: GameAction>(game: &G, n_samples: usize) -> f32 {
    let mut score_sum: f32 = 0.0;

    for _ in 0..n_samples {
        score_sum += playout(game).reward();
    }
    (score_sum as f32) / (n_samples as f32)
}


//////////////////////////////////////////////////////////////////////////

#[derive(Debug,Copy,Clone)]
enum NodeState {
    LeafNode, FullyExpanded, Expandable
}

#[derive(Debug)]
pub struct TreeNode<A: GameAction> {
    action: Option<A>,                  // how did we get here
    children: Vec<TreeNode<A>>,         // next steps we investigated
    state: NodeState,                   // is this a leaf node? fully expanded?
    n: f32, q: f32                      // statistics for this game state
}

impl<A> TreeNode<A> where A: GameAction {

    /// Create and initialize a new TreeNode
    pub fn new(action: Option<A>) -> TreeNode<A> {
        TreeNode::<A> {
            action: action,
            children: Vec::new(),
            state: NodeState::Expandable,
            n: 0., q: 0. }
    }

    /*
    /// XXX
    pub fn merge_nodes(nodes: Vec<TreeNode<A>>, depth: usize) -> TreeNode<A> {

    }
    */

    /// Find the best child accoring to UCT1
    pub fn best_child(&mut self, c: f32) -> Option<&mut TreeNode<A>> {
        let mut best_value :f32 = f32::NEG_INFINITY;
        let mut best_child :Option<&mut TreeNode<A>> = None;

        for child in &mut self.children {
            let value = child.q / child.n + c*(2.*self.n.ln()/child.n).sqrt();
            if value > best_value {
                best_value = value;
                best_child = Some(child);
            }
        }
        best_child
    }

    /// Add a child to the current node with an previously unexplored action.
    ///
    /// XXX Use HashSet? Use iterators? XXX
    pub fn expand<G>(&mut self, game: &G) -> Option<&mut TreeNode<A>>
        where G: Game<A> {
        let allowed_actions = game.allowed_actions();

        if allowed_actions.len() == 0 {
            self.state = NodeState::LeafNode;
            return None;
        }

        let mut child_actions : Vec<A> = Vec::new();
        for child in &self.children {
            match child.action {
                Some(a) => child_actions.push(a),
                None    => panic!("Child node without action"),
            }
        }

        // Find untried actions
        let mut candidate_actions = Vec::new();
        for action in &allowed_actions {
            if !child_actions.contains(action) {
                candidate_actions.push(action);
            }
        }

        if candidate_actions.len() == 1 {
            self.state = NodeState::FullyExpanded;
        }

        // Select random actions
        let action = *choose_random(&candidate_actions).clone();

        self.children.push(TreeNode::new(Some(action)));
        self.children.last_mut()
    }

    /// Recursively perform an MCTS iteration.
    ///
    /// XXX Non recursive implementation would probably be faster. XXX
    pub fn iteration<G: Game<A>>(&mut self, game: &mut G, c: f32) -> f32 {
        let delta = match self.state {
            NodeState::LeafNode => {
                game.reward()
            },
            NodeState::FullyExpanded => {
                // Choose and recurse into child...
                let child = self.best_child(c).unwrap();
                game.make_move(&child.action.unwrap());
                child.iteration(game, c)
            },
            NodeState::Expandable => {
                let child = self.expand(game);
                match child {
                    Some(child) => {           // We expanded our current node...
                        game.make_move(&child.action.unwrap());
                        let delta = playout(game).reward();
                        child.n += 1.;
                        child.q += delta;
                        delta
                    },
                    None => game.reward()      // Could not expand, current node is a leaf node!
                }
            }
        };
        self.n += 1.;
        self.q += delta;
        delta
    }
}



impl<A: GameAction> fmt::Display for TreeNode<A> {

    /// Output a nicely indented tree
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Nested definition for recursive formatting
        fn fmt_subtree<M: GameAction>(f: &mut fmt::Formatter, node: &TreeNode<M>, indent_level :i32) -> fmt::Result {
            for _ in (0..indent_level) {
                try!(f.write_str("    "));
            }
            match node.action {
                Some(a)  => try!(writeln!(f, "{:?} q={} n={}", a, node.q, node.n)),
                None     => try!(writeln!(f, "Root q={} n={}", node.q, node.n))
            }
            for child in &node.children {
                try!(fmt_subtree(f, child, indent_level+1));
            }
            write!(f, "")
        }

        fmt_subtree(f, self, 0)
    }
}


//////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
/// Represents an ensamble of MCTS trees.
///
/// For many applications we need to work with ensambles because we use
/// determinization.
pub struct MCTS<G: Game<A>, A: GameAction> {
    roots: Vec<TreeNode<A>>,
    games: Vec<G>
}

impl<G: Game<A>, A: GameAction> MCTS<G, A> {

    /// Create a new MCTS solver.
    pub fn new(game: &G, ensamble_size: usize) -> MCTS<G, A> {
        let mut roots = Vec::new();
        let mut games = Vec::new();
        for i in 0..ensamble_size {
            let mut game = game.clone();
            game.set_rng_seed(i as u32);
            games.push(game);
            roots.push(TreeNode::new(None));
        }
        MCTS {roots: roots, games: games}
    }

    /// Set a new game state for this solver.
    pub fn advance_game(&mut self, game: &G) {
        let ensamble_size = self.games.len();

        let mut roots = Vec::new();
        let mut games = Vec::new();
        for i in 0..ensamble_size {
            let mut game = game.clone();
            game.set_rng_seed(i as u32);
            games.push(game);
            roots.push(TreeNode::new(None));
        }
        self.games = games;
        self.roots = roots;
    }

    pub fn search(&mut self, n_samples: usize, c: f32) {
        let ensamble_size = self.games.len();

        // Iterate over ensamble and perform MCTS iterations
        for e in 0..ensamble_size {
            let game = &self.games[e];
            let root = &mut self.roots[e];

            // Perform MCTS iterations
            for _ in 0..n_samples {
                let mut this_game = game.clone();
                root.iteration(&mut this_game, c);
            }
        }
    }

    /// Return the best action found so far by averaging over the ensamble.
    pub fn best_action(&self) -> Option<A> {
        let ensamble_size = self.games.len();

        // Merge ensamble results
        let mut n_values = HashMap::<A, f32>::new();
        let mut q_values = HashMap::<A, f32>::new();

        for e in 0..ensamble_size {
            let root = &self.roots[e];

            for child in &root.children {
                let action = child.action.unwrap();

                let n = n_values.entry(action).or_insert(0.);
                let q = q_values.entry(action).or_insert(0.);

                *n += child.n;
                *q += child.q;
            }
        }

        // Find best action
        let mut best_action: Option<A> = None;
        let mut best_value: f32 = f32::NEG_INFINITY;
        for (action, n) in &n_values {
            let q = q_values.get(action).unwrap();
            let value = q / n;
            if value > best_value {
                best_action = Some(*action);
                best_value = value;
            }
        }

        best_action
    }
}


impl<G: Game<A>, A: GameAction> fmt::Display for MCTS<G, A> {

    /// Output a nicely indented tree
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Ensable of {} trees:", self.roots.len()));
        //for root in &self.roots {
        //    try!(root.fmt(f));
        //}
        write!(f, "")
    }
}


/////////////////////////////////////////////////////////////////////////////
// Unittests

#[cfg(test)]
mod tests {
    use test::Bencher;

    use mcts::*;
    use minigame::MiniGame;
    // use tictactoe::{TicTacToe, Action, Player, GameStatus};

    #[test]
    fn test_playout() {
        let game = MiniGame::new();
        let game = playout(&game);
        println!("Final: {:?}", game);
    }

    #[test]
    fn test_expand() {
        let game = MiniGame::new();
        let mut node = TreeNode::new(None);

        node.expand(&game);
        node.expand(&game);
        {
            let v = node.expand(&game).unwrap();
            v.expand(&game);
        }

        println!("After some expands:\n{}", node);
    }

    /*
    #[test]
    fn test_mcts() {
        let game = MiniGame::new();
        let mut mcts = MCTS::new(&game, 1);
        //println!("MCTS on new game: {:?}", mcts);



        for i in 0..5 {
            mcts.root.iteration(&mut game.clone(), 1.0);
            println!("After {} iteration(s):\n{}", i, mcts);
        }
    }*/

    #[test]
    fn test_search() {
        let game = MiniGame::new();
        let mut mcts = MCTS::new(&game, 2);

        mcts.search(50, 1.);

        println!("Search result: {:?}", mcts.best_action());
    }


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
