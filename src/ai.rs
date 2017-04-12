use game::SIZE;
use game::Game;
use game::Move;

// memoize heuristic?
const EXPLORATION: f64 = 1.414213562;

pub struct GameTree {
    wins: i32,
    games: i32,
    children: [Option<Box<GameTree>>; (SIZE * SIZE) as usize],
}

impl GameTree {
    pub fn select_child(&self) {
        self.children.iter().fold((0_f64, &self.children[0]), |max, child| {
            match *child {
                Some(ref x) => {
                    let heuristic = calc_heuristic(self.games, x);
                    if heuristic > max.0 {
                        (heuristic, child)
                    } else {
                        max
                    }
                },
                None => max,
            }
        });
    }
}

fn calc_heuristic(parent_games: i32, tree: &Box<GameTree>) -> f64 {
    tree.wins as f64 / tree.games as f64 + EXPLORATION * ((parent_games as f64).ln() / tree.games as f64).sqrt()
}
