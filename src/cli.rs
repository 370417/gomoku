use std::io;

use SIZE;

pub enum Command {
    Exit,
    Move(u32, u32),
    None,
}

/// Read one move from stdin
pub fn read_move(stdin: &io::Stdin, player: &str) -> Command {
    let mut line = String::new();

    println!("{}>", player);
    stdin.read_line(&mut line).expect("Failed to read line");
    let input = line.trim();
    match input {
        "exit" | "quit" | "q" => Command::Exit,
        _ => parse_pos(input),
    }
}

/// ```
/// assert_eq!(parse_pos("A1"), Some((0, 0)));
/// assert_eq!(parse_pos("C10"), Some((2, 9)));
/// assert_eq!(parse_pos("Z99"), None);
/// assert_eq!(parse_pos("A 1"), None);
/// assert_eq!(parse_pos("A0"), None);
/// assert_eq!(parse_pos("a1"), None);
/// ```
fn parse_pos(str: &str) -> Command {
    if str.len() < 2 {
        return Command::None;
    }
    let mut chars = str.chars();
    let head = chars.next().unwrap();
    let tail = chars.as_str();
    let col = parse_letter(head);
    let row = parse_number(tail);
    match (col, row) {
        (Some(x), Some(y)) => Command::Move(x, y),
        _ => Command::None,
    }
}

fn parse_letter(character: char) -> Option<u32> {
    let letter = character as u32;
    let a = 'A' as u32;
    if letter >= a && letter < a + SIZE {
        Some(letter - a)
    } else {
        None
    }
}

fn parse_number(str: &str) -> Option<u32> {
    match str.parse::<u32>() {
        Ok(n) if n > 0 && n <= SIZE => Some(n - 1),
        _ => None,
    }
}
