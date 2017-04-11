//! Handles game state and logic

pub const SIZE: i32 = 19;

#[derive(Copy, Clone)]
pub enum Player {
    Black,
    White,
}

pub enum Move {
    Win,
    Tie,
    Normal,
    Fail, 
}

pub struct Game {
    pub player: Player,
    pub board: [[Option<Player>; SIZE as usize]; SIZE as usize],
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::Black,
            board: [[None; SIZE as usize]; SIZE as usize],
        }
    }

    /// Make the move (x, y)
    pub fn make_move(&mut self, x: i32, y: i32) -> Move {
        if self.board[y as usize][x as usize].is_some() {
            return Move::Fail;
        }
        self.board[y as usize][x as usize] = Some(self.player);
        if self.check_victory(x, y) {
            return Move::Win;
        }
        self.player = match self.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };
        Move::Normal
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
        match (self.player, self.board[x as usize][y as usize]) {
            (Player::White, Some(Player::White)) | (Player::Black, Some(Player::Black)) => 1 + self.count_ray(x + dx, y + dy, dx, dy),
            _ => 0,
        }
    }
}
