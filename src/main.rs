//! Handles i/o

mod game;
mod ai;

use std::io;

use game::SIZE;
use game::Game;
use game::Move;

enum Command {
    Exit,
    Move(i32, i32),
    Show(i32, i32),
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
                Command::Show(x, y) => show_move(&game, x, y),
                Command::None => (),
            };
            match result {
                Move::Fail => 0,
                Move::Normal => break,
                Move::Win => return,
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
        print!("{} ", char::from(('A' as i32 + n) as u8));
    }
    for row in 0..SIZE {
        print!("\n{:2} ", row + 1);
        for col in 0..SIZE {
            let char = match game.piece(col, row) {
                0 => '·',
                n if n % 2 == 0 => 'O',
                _ => 'X',
            };
            print!("{} ", char);
        }
    }
    println!();
}

fn show_move(game: &Game, x: i32, y: i32) {
    print!("   ");
    for n in 0..SIZE {
        print!("{} ", char::from(('A' as i32 + n) as u8));
    }
    for row in 0..SIZE {
        let separator = match (x, y) {
            (0, b) if b == row => '[',
            _ => ' ',
        };
        print!("\n{:2}{}", row + 1, separator);
        for col in 0..SIZE {
            let char = match game.piece(col, row) {
                0 => '·',
                n if n % 2 == 0 => 'O',
                _ => 'X',
            };
            let separator = match (x, y) {
                (a, b) if a == col && b == row => ']',
                (a, b) if a == col + 1 && b == row => '[',
                _ => ' ',
            };
            print!("{}{}", char, separator);
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
fn into_move(pos: Option<(i32, i32)>) -> Command {
    match pos {
        Some((x, y)) => Command::Move(x, y),
        None => Command::None,
    }
}

/// Turn an optional tuple into a show command
fn into_show(pos: Option<(i32, i32)>) -> Command {
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
fn parse_pos(str: &str) -> Option<(i32, i32)> {
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
fn parse_letter(character: char) -> Option<i32> {
    let letter = character as i32;
    let a = 'A' as i32;
    if letter >= a && letter < a + SIZE {
        Some(letter - a)
    } else {
        None
    }
}

/// Parse a str as a row number
fn parse_number(str: &str) -> Option<i32> {
    match str.parse::<i32>() {
        Ok(n) if n > 0 && n <= SIZE => Some(n - 1),
        _ => None,
    }
}
