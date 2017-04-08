use std::io;

mod cli;
use cli::Command;

const SIZE: u32 = 19;

fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();
    loop {
        let input = cli::read_move(&stdin, "X");
        match input {
            Command::Exit => break,
            Command::Move(x, y) => println!("{:?}", (x, y)),
            Command::None => println!("None"),
        };
    }
}
