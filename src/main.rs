#![allow(dead_code)]
#![allow(unused)]

use board::Board;

mod board;
mod moves;

fn main() {
    let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0").unwrap();
    print!("{b}");
    println!("Move Generation:");
    let moves = b.gen_moves();
    for m in moves {
        println!("{m}");
    }
}
