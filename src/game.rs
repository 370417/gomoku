//! Handles game state and logic

pub const SIZE: u32 = 19;

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
    pub fn make_move(&mut self, x: u32, y: u32) -> Move {
        let mut piece = &mut self.board[y as usize][x as usize];
        if piece.is_some() {
            return Move::Fail;
        }
        *piece = Some(self.player);
        println!("hi");
        self.player = match self.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };
        Move::Normal
    }
}
