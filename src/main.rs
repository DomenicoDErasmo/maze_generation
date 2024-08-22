//! Generates a maze and prints to stdout

use maze_generation::maze::Maze;

fn main() {
    let maze = Maze::from_backtracking(20, 20);
    println!("{maze:#?}");
}
