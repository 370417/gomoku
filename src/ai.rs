use game::{SIZE, Game, Move};

use rand;
use rand::Rng;

struct GameTree {
    move_: (i32, i32),
    wins: i32,
    games: i32,
    unvisited: Vec<(i32, i32)>,
    children: Vec<GameTree>,
}

pub fn make_move(game: &Game) -> (i32, i32) {
    let mut root = GameTree {
        move_: (-1, -1),
        wins: 0,
        games: 0,
        unvisited: get_possible_moves(game),
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

    fn backpropagate(&mut self, game: &mut Game) -> i32 {
        match (self.unvisited.len(), self.children.len()) {
            (a, b) if a == 0 && b == 0 => 2,
            (a, _) if a > 0 => self.expand(game),
            _ => {
                let best_index = self.select();
                let best_child = &mut self.children[best_index];
                let (x, y) = best_child.move_;
                game.make_move(x, y);
                let playout = best_child.backpropagate(game);
                self.wins += playout;
                playout
            },
        }
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

    fn expand(&mut self, game: &mut Game) -> i32 {
        let mut rng = rand::thread_rng();
        let (x, y) = remove_random(&mut self.unvisited, &mut rng);
        let result = match game.make_move(x, y) {
            Move::Win => Some(2),
            Move::Tie => Some(1),
            Move::Normal => None,
            Move::Fail => panic!(),
        };
        let possible_moves = get_possible_moves(game);
        let (wins, unvisited) = match result {
            Some(x) => (x, Vec::new()),
            None => (2 - simulate(game, &mut rng), possible_moves),
        };
        self.children.push(GameTree {
            move_: (x, y),
            wins: wins,
            games: 2,
            unvisited: unvisited,
            children: Vec::new(),
        });
        wins
    }

    fn uct(&self, games: i32) -> f64 {
        self.wins as f64 / self.games as f64 + (2_f64 * (games as f64).ln() / self.games as f64).sqrt()
    }

}

/// Play random moves until the end of the game.
/// Return 2 if win, 1 if tie, 0 if loss.
fn simulate<R: Rng>(game: &mut Game, rng: &mut R) -> i32 {
    let (x, y) = *choose_random(&get_possible_moves(game), rng);
    match game.make_move(x, y) {
        Move::Win => 2,
        Move::Tie => 1,
        Move::Normal => 2 - simulate(game, rng),
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

fn choose_random<'a, T, R: Rng>(things: &'a Vec<T>, rng: &mut R) -> &'a T {
    let index = rng.gen_range(0, things.len());
    &things[index]
}

fn remove_random<T, R: Rng>(things: &mut Vec<T>, rng: &mut R) -> T {
    let index = rng.gen_range(0, things.len());
    things.remove(index)
}
