//render_minimap.rs
use raylib::prelude::*;
use crate::Framebuffer;
use crate::Maze;
use crate::Player;
use crate::ItemState;
use crate::Item;


pub fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    items: &Vec<Item>,
) {
    // Configuración del minimapa
    let minimap_size = 200; // Tamaño del minimapa en píxeles
    let minimap_padding = 20; // Separación del borde
    let minimap_x = minimap_padding; // Esquina inferior izquierda
    let minimap_y = framebuffer.height - minimap_size - minimap_padding;
    
    let maze_height = maze.len();
    let maze_width = if maze_height > 0 { maze[0].len() } else { 0 };
    
    // Escala para ajustar el laberinto al minimapa
    let scale_x = minimap_size as f32 / maze_width as f32;
    let scale_y = minimap_size as f32 / maze_height as f32;
    let scale = scale_x.min(scale_y); // Usar la escala menor para mantener proporción
    
    // Dibujar fondo semitransparente del minimapa
    draw_minimap_background(framebuffer, minimap_x, minimap_y, minimap_size);
    
    // Dibujar el laberinto
    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            let x = minimap_x + (col_idx as f32 * scale) as i32;
            let y = minimap_y + (row_idx as f32 * scale) as i32;
            let cell_size = scale.max(1.0) as i32;
            
            let color = match cell {
                '|' | '+' | '-' => Color::new(100, 100, 100, 255), // Paredes grises
                'g' => Color::new(0, 255, 0, 255), // Goal en verde
                ' ' => Color::new(30, 30, 30, 255), // Piso oscuro
                _ => Color::new(50, 50, 50, 255), // Otros
            };
            
            framebuffer.set_current_color(color);
            
            // Dibujar celda
            for dx in 0..cell_size {
                for dy in 0..cell_size {
                    framebuffer.set_pixel(x + dx, y + dy);
                }
            }
        }
    }
    
    // Dibujar ítems no recogidos
    for item in items.iter() {
        if !item.collected && item.state == ItemState::Idle {
            let item_x = minimap_x + ((item.pos.x / 100.0) * scale) as i32;
            let item_y = minimap_y + ((item.pos.y / 100.0) * scale) as i32;
            
            let item_color = match item.texture_key {
                'b' => Color::new(0, 150, 255, 255), // Azul
                'c' => Color::new(255, 215, 0, 255),  // Dorado
                'h' => Color::new(255, 0, 255, 255),  // Magenta
                _ => Color::new(255, 255, 255, 255),
            };
            
            framebuffer.set_current_color(item_color);
            
            // Dibujar punto del ítem (3x3 píxeles)
            for dx in -1..=1 {
                for dy in -1..=1 {
                    framebuffer.set_pixel(item_x + dx, item_y + dy);
                }
            }
        }
    }
    
    // Dibujar jugador (triángulo apuntando en la dirección que mira)
    let player_x = minimap_x + ((player.pos.x / 100.0) * scale) as i32;
    let player_y = minimap_y + ((player.pos.y / 100.0) * scale) as i32;
    
    draw_player_on_minimap(framebuffer, player_x, player_y, player.a, scale);
    
    // Dibujar borde del minimapa
    draw_minimap_border(framebuffer, minimap_x, minimap_y, minimap_size);
}

fn draw_minimap_background(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    size: i32,
) {
    framebuffer.set_current_color(Color::new(0, 0, 0, 200));
    
    for dx in 0..size {
        for dy in 0..size {
            framebuffer.set_pixel(x + dx, y + dy);
        }
    }
}

fn draw_minimap_border(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    size: i32,
) {
    framebuffer.set_current_color(Color::new(255, 255, 255, 255));
    
    let border_thickness = 2;
    
    // Borde superior e inferior
    for dx in 0..size {
        for t in 0..border_thickness {
            framebuffer.set_pixel(x + dx, y + t); // Superior
            framebuffer.set_pixel(x + dx, y + size - 1 - t); // Inferior
        }
    }
    
    // Borde izquierdo y derecho
    for dy in 0..size {
        for t in 0..border_thickness {
            framebuffer.set_pixel(x + t, y + dy); // Izquierdo
            framebuffer.set_pixel(x + size - 1 - t, y + dy); // Derecho
        }
    }
}

fn draw_player_on_minimap(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    angle: f32,
    scale: f32,
) {
    framebuffer.set_current_color(Color::new(255, 50, 50, 255)); // Rojo brillante
    
    // Tamaño del triángulo del jugador
    let size = (5.0 * (scale / 10.0)).max(3.0) as i32;
    
    // Calcular puntos del triángulo
    let tip_x = x + (size as f32 * angle.cos()) as i32;
    let tip_y = y + (size as f32 * angle.sin()) as i32;
    
    let back_angle_1 = angle + 2.5;
    let back_angle_2 = angle - 2.5;
    
    let back_x1 = x + ((size / 2) as f32 * back_angle_1.cos()) as i32;
    let back_y1 = y + ((size / 2) as f32 * back_angle_1.sin()) as i32;
    
    let back_x2 = x + ((size / 2) as f32 * back_angle_2.cos()) as i32;
    let back_y2 = y + ((size / 2) as f32 * back_angle_2.sin()) as i32;
    
    // Dibujar líneas del triángulo
    draw_line_minimap(framebuffer, x, y, tip_x, tip_y);
    draw_line_minimap(framebuffer, tip_x, tip_y, back_x1, back_y1);
    draw_line_minimap(framebuffer, back_x1, back_y1, back_x2, back_y2);
    draw_line_minimap(framebuffer, back_x2, back_y2, tip_x, tip_y);
    
    // Rellenar el triángulo (simple)
    framebuffer.set_current_color(Color::new(255, 100, 100, 255));
    for dx in -size..=size {
        for dy in -size..=size {
            framebuffer.set_pixel(x + dx, y + dy);
        }
    }
    
    // Redibujar el borde en rojo brillante
    framebuffer.set_current_color(Color::new(255, 50, 50, 255));
    draw_line_minimap(framebuffer, x, y, tip_x, tip_y);
    draw_line_minimap(framebuffer, tip_x, tip_y, back_x1, back_y1);
    draw_line_minimap(framebuffer, back_x1, back_y1, back_x2, back_y2);
    draw_line_minimap(framebuffer, back_x2, back_y2, tip_x, tip_y);
}

fn draw_line_minimap(
    framebuffer: &mut Framebuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) {
    // Algoritmo de Bresenham para dibujar líneas
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    
    let mut x = x0;
    let mut y = y0;
    
    loop {
        framebuffer.set_pixel(x, y);
        
        if x == x1 && y == y1 {
            break;
        }
        
        let e2 = 2 * err;
        
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
