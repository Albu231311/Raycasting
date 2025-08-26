// maze.rs

use std::fs::File;
use std::io::{BufRead, BufReader};
use raylib::prelude::Vector2;

pub type Maze = Vec<Vec<char>>;

#[derive(Debug, Clone)]
pub struct SpritePosition {
    pub x: f32,
    pub y: f32,
}

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}


pub fn extract_sprite_positions(maze: &Maze, block_size: usize) -> Vec<SpritePosition> {
    let mut sprite_positions = Vec::new();
    
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == '.' {
                // Calcular la posición central del bloque
                let x = (col_index * block_size) as f32 + (block_size as f32 / 2.0);
                let y = (row_index * block_size) as f32 + (block_size as f32 / 2.0);
                
                sprite_positions.push(SpritePosition { x, y });
            }
        }
    }
    
    sprite_positions
}


pub fn clean_maze(maze: &mut Maze) {
    for row in maze.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == '.' {
                *cell = ' '; 
            }
        }
    }
}