//! Generates a maze and prints to stdout

use maze_generation::cell::Cell;

fn main() {
    println!("Hello, world!");

    let cell = Cell::PATH;
    let cell2 = Cell::WALL;

    println!("{cell:#?}, {cell2:#?}");
}
