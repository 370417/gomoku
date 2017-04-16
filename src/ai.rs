use rand;
use rand::Rng;

use game::{SIZE, AREA, Game, Move};

// memoize heuristic?
const EXPLORATION: f64 = 1.414213562;

pub struct GameTree {
    wins: i32,
    games: i32,
    children: [Node; AREA],
}

type Node = Option<Box<GameTree>>;

pub fn make_move(game: &Game) -> (i32, i32) {
    let mut root = GameTree::new();
    for _ in 0..20 {
        root.select(&mut game.clone());
    }
    let mut games = 0;
    let mut best_x = 0;
    let mut best_y = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            match *root.child(x, y) {
                Some(ref child) if child.games > games => {
                    games = child.games;
                    best_x = x;
                    best_y = y;
                },
                Some(_) => (),
                None => (),
            }
        }
    }
    (best_x, best_y)
}

impl GameTree {
    fn new() -> GameTree {
        GameTree {
            wins: 0,
            games: 0,
            children: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
        }
    }

    fn child(&self, x: i32, y: i32) -> &Node {
        &self.children[(x + y * SIZE) as usize]
    }

    fn mut_child(&mut self, x: i32, y: i32) -> &mut Node {
        &mut self.children[(x + y * SIZE) as usize]
    }

    pub fn select(&mut self, game: &mut Game) -> (i32, i32) {
        let playout = match self.get_best_move() {
            Some((x, y)) => match game.make_move(x, y) {
                Move::Win => (2, 2),
                Move::Tie => (1, 2),
                Move::Normal => match *self.mut_child(x, y) {
                    Some(ref mut child) => child.select(game),
                    None => panic!(),
                },
                Move::Fail => panic!(),
            },
            None => self.expand(game),
        };
        self.wins += 2 - playout.0;
        self.games += playout.1;
        playout
    }

    pub fn expand(&mut self, game: &mut Game) -> (i32, i32) {
        let moves = get_possible_moves(game);
        let mut wins = 0;
        let mut games = 0;
        let mut rng = rand::thread_rng();
        for &(x, y) in moves.iter() {
            let mut game_clone = game.clone();
            let playout = match game_clone.make_move(x, y) {
                Move::Normal => (simulate(&mut game_clone, &mut rng), 2),
                Move::Win => (2, 2),
                Move::Tie => (1, 2),
                Move::Fail => panic!(),
            };
            let mut child = GameTree::new();
            child.wins = playout.0;
            child.games = playout.1;
            self.children[(x + y * SIZE) as usize] = Some(Box::new(child));
            wins += playout.0;
            games += playout.1;
        }
        (wins, games)
    }

    fn get_best_move(&self) -> Option<(i32, i32)> {
        let mut max_heuristic = 0_f64;
        let mut best_child = self.child(0, 0);
        let mut best_move = (0, 0);
        for y in 0..SIZE {
            for x in 0..SIZE {
                match *self.child(x, y) {
                    Some(ref child) => {
                        let heuristic = calc_heuristic(self.games, child);
                        if heuristic > max_heuristic {
                            max_heuristic = heuristic;
                            best_child = self.child(x, y);
                            best_move = (x, y);
                        }
                    },
                    None => (),
                };
            }
        }
        match *best_child {
            Some(..) => Some(best_move),
            None => None,
        }
    }
}

fn simulate(game: &mut Game, rng: &mut rand::ThreadRng) -> i32 {
    let moves = get_possible_moves(game);
    let index = rng.gen_range(0, moves.len());
    let (x, y) = moves[index];
    match game.make_move(x, y) {
        Move::Normal => 2 - simulate(game, rng),
        Move::Win => 2,
        Move::Tie => 1,
        Move::Fail => panic!(),
    }
}

fn get_possible_moves(game: &Game) -> Vec<(i32, i32)> {
    let mut moves = Vec::new();
    for y in 0..SIZE {
        for x in 0..SIZE {
            if game.can_move(x, y) {
                moves.push((x, y));
            }
        }
    }
    moves
}

fn calc_heuristic(parent_games: i32, tree: &Box<GameTree>) -> f64 {
    tree.wins as f64 / tree.games as f64 + EXPLORATION * ((parent_games as f64).ln() / tree.games as f64).sqrt()
}
