//caster.rs
use std::hint;

use raylib::color::Color;
use crate::SpecialWall;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;

pub struct Intersect {
    pub distance: f32,
    pub impact:char,
    pub tx: f32,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
     maze: &Maze, 
     player: &Player, 
     a: f32, 
     block_size: usize, 
     draw_line: bool,
     special_wall: Option<&SpecialWall>,
) -> Intersect {
    let mut d = 0.0;
    let maze_height = maze.len();
    let maze_width = if maze_height > 0 { maze[0].len() } else { 0 };

    framebuffer.set_current_color(Color::WHITE);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;
        

        // Verificar si el rayo se sale de los límites del laberinto
        if j >= maze_height || i >= maze_width {
        let hitx = x - i*block_size;
        let hity = y - j * block_size;
        let mut maxhit = hity as f32; 

        if 1 <hitx && hitx < block_size -1 {
            maxhit = hitx as f32;
        }

        let tx = maxhit /block_size as f32;

            return Intersect {
                distance: d,
                impact: ' ',  // Rayo sale del laberinto
                tx:tx
            };
        }

        let mut cell = maze[j][i];

        if cell == '*' {
            if let Some(wall) = special_wall {
                cell = wall.get_texture_key(); // Devuelve '*' o '$'
            }
        }

    
        if cell != ' ' && cell != 'b' && cell != 'c' && cell != 'h'  && cell != 'i' {
            let hitx = x - i * block_size;
            let hity = y - j * block_size;
            let maxhit = if hitx.abs_diff(0) < hitx.abs_diff(block_size) && 
                           hitx.abs_diff(0) < hity.abs_diff(0) && 
                           hitx.abs_diff(0) < hity.abs_diff(block_size) {
                hity as f32
            } else if hitx.abs_diff(block_size) < hity.abs_diff(0) && 
                      hitx.abs_diff(block_size) < hity.abs_diff(block_size) {
                hity as f32
            } else if hity.abs_diff(0) < hity.abs_diff(block_size) {
                hitx as f32
            } else {
                hitx as f32
            };
           
            let tx = maxhit / block_size as f32;

            return Intersect {
                distance: d,
                impact: cell,
                tx: tx
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as i32, y as i32);
        }

        d += 1.0;

        // Evitar que el rayo recorra una distancia demasiado larga
        if d > 2000.0 {
            break;
        }
    }

    // Si no se encuentra nada, retornar un impacto vacío
    Intersect {
        distance: d,
        impact: ' ', // ''
        tx: 0.0
    }
}
 