use std::io;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();
    read_move(&stdin);
}

fn read_move(stdin: &std::io::Stdin) {
    for line in stdin.lock().lines() {
        let string = line.unwrap();
        let word = string.trim();

        match word {
            "exit" => return,
            _ => println!("echo {}", word),
        };
    }
}
