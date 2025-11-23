
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Vector2;

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn find_char_position(maze: &Maze, target: char, block_size: usize) -> Option<Vector2> {
    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == target {
                // Calcular posición en el centro de la celda
                let x = (col_idx * block_size) as f32 + (block_size as f32 / 2.0);
                let y = (row_idx * block_size) as f32 + (block_size as f32 / 2.0);
                return Some(Vector2::new(x, y));
            }
        }
    }
    None
}