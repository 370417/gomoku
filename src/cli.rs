use std::io;
use std::io::BufRead;

use SIZE;

pub fn read_move(stdin: &io::Stdin) {
    for line in stdin.lock().lines() {
        let string = line.unwrap();
        let word = string.trim();

        match word {
            "exit" | "quit" | "q" => return,
            _ if is_pos(word) => println!("pos {}", word),
            _ => println!("echo {}", word),
        };
    }
}

/// Determine if str is a valid position
///
/// ```
/// assert!(is_pos("A1"));
/// assert!(!is_pos("Z99"));
/// assert!(!is_pos("A 1"));
/// assert!(!is_pos("A0"));
/// assert!(!is_pos("a1"));
/// ```
fn is_pos(str: &str) -> bool {
    if str.len() < 2 {
        return false;
    }
    let (head, tail) = divide_pos(str);
    is_letter(head) && is_number(tail)
}

/// Divide a str into a head char and tail str
fn divide_pos(str: &str) -> (char, &str) {
    let mut chars = str.chars();
    let head = chars.next().unwrap();
    let tail = chars.as_str();
    (head, tail)
}

/// Determine if char is a valid column letter
fn is_letter(character: char) -> bool {
    let letter = character as u32;
    let a = 'A' as u32;
    letter >= a && letter < a + SIZE
}

/// Determine if str is a valid row number
fn is_number(str: &str) -> bool {
    match str.parse::<u32>() {
        Ok(n) if n > 0 && n <= SIZE => true,
        _ => false,
    }
}
