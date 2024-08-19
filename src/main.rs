//! Generates a maze and prints to stdout

use maze_generation::tile::Tile;

fn main() {
    println!("Hello, world!");

    let cell = Tile::PATH;
    let cell2 = Tile::WALL;

    println!("{cell:#?}, {cell2:#?}");
}
