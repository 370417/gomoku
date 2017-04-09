//! Handles i/o

mod game;

use std::io;

use game::SIZE;
use game::Player;
use game::Game;
use game::Move;

enum Command {
    Exit,
    Move(u32, u32),
    Show(u32, u32),
    None,
}

fn main() {
    let mut game = Game::new();
    let stdin = io::stdin();
    loop {
        display_game(&game);
        // let mut move_result = Move::Fail;
        loop {
            let mut result = Move::Fail;
            match read_move(&stdin) {
                Command::Exit => return,
                Command::Move(x, y) => result = game.make_move(x, y),
                Command::Show(x, y) => println!("{} {}", x, y),
                Command::None => (),
            };
            match result {
                Move::Fail => 0,
                Move::Normal => break,
                Move::Win => break,
                Move::Tie => break,
            };
        }
    }
}

/// Read one move from stdin
fn read_move(stdin: &io::Stdin) -> Command {
    let mut line = String::new();
    println!(">>");
    stdin.read_line(&mut line).expect("Failed to read line");
    match line.trim() {
        "exit" | "quit" | "q" => Command::Exit,
        input if is_question(input) => into_show(parse_pos(&drop(input))),
        input => into_move(parse_pos(input)),
    }
}

/// Print the game board
fn display_game(game: &Game) {
    print!("   ");
    for n in 0..SIZE {
        print!("{} ", char::from(('A' as u32 + n) as u8));
    }
    let board = &game.board;
    for (index, row) in board.iter().enumerate() {
        print!("\n{:2} ", index + 1);
        for piece in row {
            let char = match *piece {
                None => '·',
                Some(Player::White) => 'O',
                Some(Player::Black) => 'X',
            };
            print!("{} ", char);
        }
    }
    println!();
}

/// Determine if a str ends in a '?'
fn is_question(str: &str) -> bool {
    match str.chars().rev().next() {
        Some('?') => true,
        _ => false,
    }
}

/// Drop the last char off a string
fn drop(str: &str) -> String {
    let mut string = str.to_string();
    string.pop();
    string
}

/// Turn an optional tuple into a move command
fn into_move(pos: Option<(u32, u32)>) -> Command {
    match pos {
        Some((x, y)) => Command::Move(x, y),
        None => Command::None,
    }
}

/// Turn an optional tuple into a show command
fn into_show(pos: Option<(u32, u32)>) -> Command {
    match pos {
        Some((x, y)) => Command::Show(x, y),
        None => Command::None,
    }
}

/// Parse a str as a position
///
/// # Examples
///
/// ```
/// assert_eq!(parse_pos("A1"), Some(Command::Move(0, 0)));
/// assert_eq!(parse_pos("C10"), Some(Command::Move(2, 9)));
/// assert_eq!(parse_pos("Z99"), None);
/// assert_eq!(parse_pos("A 1"), None);
/// assert_eq!(parse_pos("A0"), None);
/// assert_eq!(parse_pos("a1"), None);
/// ```
fn parse_pos(str: &str) -> Option<(u32, u32)> {
    if str.len() < 2 {
        return None;
    }
    let mut chars = str.chars();
    let head = chars.next().unwrap();
    let tail = chars.as_str();
    let col = parse_letter(head);
    let row = parse_number(tail);
    match (col, row) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

/// Parse a char as a column letter
fn parse_letter(character: char) -> Option<u32> {
    let letter = character as u32;
    let a = 'A' as u32;
    if letter >= a && letter < a + SIZE {
        Some(letter - a)
    } else {
        None
    }
}

/// Parse a str as a row number
fn parse_number(str: &str) -> Option<u32> {
    match str.parse::<u32>() {
        Ok(n) if n > 0 && n <= SIZE => Some(n - 1),
        _ => None,
    }
}
