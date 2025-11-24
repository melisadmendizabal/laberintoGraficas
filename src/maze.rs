//maze.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Vector2;
use crate::item::Item;
use crate::menu::ItemConfig;

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

// Encontrar la posición de un carácter en el laberinto
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

// Spawn automático con configuración específica
pub fn spawn_items_from_config(
    maze: &Maze,
    block_size: usize,
    item_configs: &[crate::menu::ItemConfig],
) -> Vec<Item> {
    let mut items = Vec::new();

    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            // Buscar si este caracter tiene configuración
            if let Some(config) = item_configs.iter().find(|c| c.char_key == cell) {
                let x = (col_idx * block_size) as f32 + (block_size as f32 / 2.0);
                let y = (row_idx * block_size) as f32 + (block_size as f32 / 2.0);

                // Crear item con la textura específica del mapa
                let mut item = Item::new(x, y, config.texture_key);
                item.name = config.item_name.clone();

                items.push(item);
                println!(
                    "✨ Item '{}' ({}) spawneado en ({}, {})",
                    config.item_name, config.texture_key, x, y
                );
            }
        }
    }

    items
}

