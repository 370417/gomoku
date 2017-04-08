use std::io;

mod cli;

const SIZE: u32 = 19;

fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();
    cli::read_move(&stdin);
}
