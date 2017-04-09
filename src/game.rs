pub const SIZE: u32 = 19;

#[derive(Copy, Clone)]
pub enum Player {
    Black,
    White,
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

    pub fn can_move(&self, x: u32, y: u32) -> bool {
        self.board[y as usize][x as usize].is_none()
    }

    pub fn make_move(&mut self, x: u32, y: u32) {
        self.board[y as usize][x as usize] = Some(self.player);
        self.player = match self.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };
    }
}
