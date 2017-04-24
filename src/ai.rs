use game::{SIZE, Game, Move};

use rand;
use rand::Rng;

struct GameTree {
    move_: (i32, i32),
    wins: i32,
    games: i32,
    children: Vec<GameTree>,
}

pub fn make_move(game: &Game) -> (i32, i32) {
    let mut root = GameTree {
        move_: (-1, -1),
        wins: 0,
        games: 0,
        children: Vec::new(),
    };
    for _ in 0..100 {
        root.backpropagate(&mut game.clone());
    }
    let mut best_move = (0, 0);
    let mut most_games = 0;
    for child in root.children.iter() {
        if child.games > most_games {
            best_move = child.move_;
            most_games = child.games;
        }
    }
    best_move
}

impl GameTree {

    fn backpropagate(&mut self, game: &mut Game) -> (i32, i32) {
        let (wins, games) = if self.children.len() > 0 {
            let best_index = self.select();
            let best_child = &mut self.children[best_index];
            let (x, y) = best_child.move_;
            game.make_move(x, y);
            best_child.backpropagate(game)
        } else {
            let (wins, games, children) = self.expand(game);
            self.children = children;
            (wins, games)
        };
        self.wins += wins;
        self.games += games;
        (wins, games)
    }

    /// Select the best child state according to UCT.
    /// Panic if there are no children.
    fn select(&self) -> usize {
        let mut best_index = 0;
        let mut best_uct = 0_f64;
        for (index, child) in self.children.iter().enumerate() {
            let uct = child.uct(self.games);
            if uct > best_uct {
                best_index = index;
                best_uct = uct;
            }
        }
        best_index
    }

    fn expand(&self, game: &Game) -> (i32, i32, Vec<GameTree>) {
        let mut rng = rand::thread_rng();
        let mut total_wins = 0;
        let mut total_games = 0;
        let mut children = Vec::new();
        for y in 0..SIZE {
            for x in 0..SIZE {
                let wins = simulate_move(x, y, &mut game.clone(), &mut rng);
                if wins >= 0 {
                    total_wins += wins;
                    total_games += 2;
                    children.push(GameTree {
                        move_: (x, y),
                        wins: wins,
                        games: 2,
                        children: Vec::new(),
                    });
                }
            }
        }
        (total_wins, total_games, children)
    }

    fn uct(&self, games: i32) -> f64{
        self.wins as f64 / self.games as f64 + (2_f64 * (games as f64).ln() / self.games as f64).sqrt()
    }

}

/// Simulate the outcome of a random game after a specific move is made.
/// Return -1 if the move can't be made.
fn simulate_move<R: Rng>(x: i32, y: i32, game: &mut Game, rng: &mut R) -> i32 {
    match game.make_move(x, y) {
        Move::Win => 2,
        Move::Tie => 1,
        Move::Normal => 2 - simulate(game, rng),
        Move::Fail => -1,
    }
}

/// Play random moves until the end of the game.
/// Return 2 if win, 1 if tie, 0 if loss.
fn simulate<R: Rng>(game: &mut Game, rng: &mut R) -> i32 {
    let (x, y) = get_random_move(game, rng);
    match game.make_move(x, y) {
        Move::Win => 2,
        Move::Tie => 1,
        Move::Normal => 2 - simulate(game, rng),
        Move::Fail => panic!(),
    }
}

fn get_random_move<R: Rng>(game: &Game, rng: &mut R) -> (i32, i32) {
    let mut moves = Vec::new();
    for y in 0..SIZE {
        for x in 0..SIZE {
            if game.can_move(x, y) {
                moves.push((x, y));
            }
        }
    }
    let index = rng.gen_range(0, moves.len());
    moves[index]
}
