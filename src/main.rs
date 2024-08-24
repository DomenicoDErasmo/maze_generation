//! Generates a maze and prints to stdout

use maze_generation::maze::Maze;

fn main() {
    let Some(maze) = Maze::from_backtracking(20, 20) else {
        eprintln!("Failed to generate maze.");
        return;
    };
    println!("{maze:#?}");
}
