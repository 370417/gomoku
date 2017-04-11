//! Handles game state and logic

pub const SIZE: i32 = 19;

pub enum Move {
    Win,
    Tie,
    Normal,
    Fail, 
}

pub struct Game {
    pub turn: i32,
    pub board: [[Option<i32>; SIZE as usize]; SIZE as usize],
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: 1,
            board: [[None; SIZE as usize]; SIZE as usize],
        }
    }

    /// Make the move (x, y)
    pub fn make_move(&mut self, x: i32, y: i32) -> Move {
        if self.board[y as usize][x as usize].is_some() {
            return Move::Fail;
        }
        self.board[y as usize][x as usize] = Some(self.turn);
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
        if x < 0 || y < 0 || x >= SIZE || y >= SIZE {
            return 0;
        }
        match self.board[y as usize][x as usize] {
            Some(n) if same_parity(n, self.turn) => 1 + self.count_ray(x + dx, y + dy, dx, dy),
            _ => 0,
        }
    }
}

fn same_parity(a: i32, b: i32) -> bool {
    (a + b) % 2 == 0
}
