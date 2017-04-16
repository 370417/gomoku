//! Handles game state and logic

pub const SIZE: i32 = 15;
pub const AREA: usize = 15 * 15;

pub enum Move {
    Win,
    Tie,
    Normal,
    Fail, 
}

pub struct Game {
    pub turn: i32,
    board: [i32; AREA],
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: 1,
            board: [0; AREA],
        }
    }

    pub fn piece(&self, x: i32, y: i32) -> i32 {
        self.board[(x + y * SIZE) as usize]
    }

    pub fn can_move(&self, x:i32, y: i32) -> bool {
        let center = SIZE / 2;
        if self.turn == 3 && x >= center - 3 && x <= center + 3 && y >= center - 3 && y <= center + 3 {
            return false;
        }
        self.piece(x, y) == 0
    }

    /// Make the move (x, y)
    pub fn make_move(&mut self, x: i32, y: i32) -> Move {
        if !self.can_move(x, y) {
            return Move::Fail;
        }
        self.board[(x + y * SIZE) as usize] = self.turn;
        if self.check_victory(x, y) {
            return Move::Win;
        }
        if self.check_tie() {
            return Move::Tie;
        }
        self.turn += 1;
        Move::Normal
    }

    fn check_tie(&self) -> bool {
        self.turn == SIZE * SIZE
    }

    fn check_victory(&self, x: i32, y: i32) -> bool {
        self.count_ray(x, y, 0, 1) + self.count_ray(x, y, 0, -1) > 5
            || self.count_ray(x, y, 1, 0) + self.count_ray(x, y, -1, 0) > 5
            || self.count_ray(x, y, 1, 1) + self.count_ray(x, y, -1, -1) > 5
            || self.count_ray(x, y, -1, 1) + self.count_ray(x, y, 1, -1) > 5
    }

    fn count_ray(&self, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
        if !in_bounds(x, y) {
            return 0;
        }
        match self.board[(x + y * SIZE) as usize] {
            0 => 0,
            n if same_parity(n, self.turn) => 1 + self.count_ray(x + dx, y + dy, dx, dy),
            _ => 0,
        }
    }
}

impl Clone for Game {
    fn clone(&self) -> Game {
        Game {
            turn: self.turn,
            board: self.board,
        }
    }
}

fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < SIZE && y < SIZE
}

fn same_parity(a: i32, b: i32) -> bool {
    (a + b) % 2 == 0
}
