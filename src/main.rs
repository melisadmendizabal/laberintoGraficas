#![allow(unused_imports)]
#![allow(dead_code)]

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod enemy;
mod item;

use item::Item;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use framebuffer::Framebuffer;
use maze::{Maze,load_maze};
use player::Player;
use std::f32::consts::PI;
use textures::TextureManager;
use enemy::Enemy;
use crate::item::ItemState;

use crate::{caster::cast_ray, player::process_events};


fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color::RED);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as i32, y as i32);
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    //draw player
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.set_pixel(player.pos.x as i32, player.pos.y as i32);

    //dibujar n rayos
    let num_rays = 5; //framebuffer.width;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a -(player.fov/2.0)+(player.fov*current_ray);
        cast_ray(framebuffer, maze, player, a,block_size,true);
    }
}

pub fn render_world(framebuffer:&mut Framebuffer, player: &Player,maze:&Maze, texture_cache: &TextureManager){
    // let maze = load_maze("./maze.txt");
    let block_size = 100;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32/2.0;
    let hh = framebuffer.height as f32/2.0;

    framebuffer.set_current_color(Color::WHITE);
    
    for i in 0..num_rays{
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov/2.0) + (player.fov*current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);
        let d = intersect.distance;
        let c = intersect.impact;

        let angle_diff = a-player.a;
        let mut distance_to_wall = intersect.distance*angle_diff.cos();
        if distance_to_wall < 0.1 {
            distance_to_wall = 0.2;
            // continue; 
        }
        let stake_height = (hh/distance_to_wall) *70.0;
        

        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;
        let texture_size = 16.0;
        

        for y in stake_top..stake_bottom{
            

            let tx = intersect.tx as f32 * (texture_size / 6.0);
            let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * texture_size;

            let color = texture_cache.get_pixel_color(c, tx as u32, ty as u32);
            framebuffer.set_current_color(color);
            // match c {
            //     '|' => framebuffer.set_current_color(color),
            //     _ => framebuffer.set_current_color(Color::new(255, 255, 255, 255)), 
            // }
            framebuffer.set_pixel(i, y as i32);

        }

    }
}

const TRANSPARENT_COLOR: Color = Color::new(152, 0, 136, 255);
fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy, //cambiar
    texture_manager: &TextureManager
) {
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
    let mut angle_diff = sprite_a - player.a;
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    if angle_diff.abs() > player.fov / 2.5 {
        return;
    }

    let sprite_d: f32 = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    // near plane           far plane
    if sprite_d < 50.0 || sprite_d > 1000.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;
    let sprite_size_usize: usize = sprite_size as usize;
    let end_x = (start_x + sprite_size_usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size_usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = ((x - start_x) * 6 / sprite_size_usize) as u32; //128
            let ty = ((y - start_y) * 6 / sprite_size_usize) as u32; //128

            let color = texture_manager.get_pixel_color(enemy.texture_key, tx, ty);
            
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as i32, y as i32);
            }
        }
    }
}




fn render_enemies(framebuffer:&mut Framebuffer, player: &Player, texture_cache: &TextureManager){
    let enemies = vec![
        Enemy::new(250.0, 250.0, 'e')
    ];
    for enemy in enemies{
        draw_sprite(framebuffer, &player, &enemy, texture_cache);
    }
}


pub fn draw_item(
    framebuffer: &mut Framebuffer,
    player: &Player,
    item: &Item,
    texture_manager: &TextureManager,
) {
    if item.collected {
        return;
    }

    let dx = item.pos.x - player.pos.x;
    let dy = item.pos.y - player.pos.y;

    // Distancia al jugador
    let distance = (dx * dx + dy * dy).sqrt();

    // Ángulo hacia el ítem
    let angle_to_item = dy.atan2(dx);
    let mut angle_diff = angle_to_item - player.a;

    // Normalización del ángulo
    while angle_diff > std::f32::consts::PI {
        angle_diff -= 2.0 * std::f32::consts::PI;
    }
    while angle_diff < -std::f32::consts::PI {
        angle_diff += 2.0 * std::f32::consts::PI;
    }

    // Ítem fuera del campo de visión
    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    // Proyección en pantalla
    let screen_x =
        (0.5 * framebuffer.width as f32) * (1.0 + angle_diff / (player.fov / 2.0));

    let base_size = (framebuffer.height as f32 / distance) * 80.0;

    //animación de rotación, se hace más estrecho según el angulo
    let rotation_factor = (item.rotation_timer.cos() * 0.5 + 0.5).max(0.3); // Entre 0.3 y 1.0
    let size_x = base_size * rotation_factor;
    let size_y = base_size;

    // APLICAR BOBBING
    let bob_offset = item.get_bob_offset();
    
    let half_size_x = size_x / 2.0;
    let half_size_y = size_y / 2.0;


    let top = (framebuffer.height as f32 / 2.0 - half_size_y + bob_offset).max(0.0);
    let bottom = (framebuffer.height as f32 / 2.0 + half_size_y + bob_offset)
        .min(framebuffer.height as f32);

    let x_start = (screen_x - half_size_x).max(0.0) as i32;
    let x_end = (screen_x + half_size_x).min(framebuffer.width as f32) as i32;

    let xsize = x_end - x_start;
    if xsize <= 0 {
        return;
    }

    let texture_key = item.texture_key;
    let texture_size = 16.0;

    for x in x_start..x_end {
        for y in top as i32..bottom as i32 {
            let tx = ((x - x_start) as f32 / xsize as f32) * texture_size;
            let ty = ((y as f32 - top) / (bottom - top)) * texture_size;

            let mut color = texture_manager.get_pixel_color(texture_key, tx as u32, ty as u32);

            let pixel_u32 =
                ((color.a as u32) << 24) |
                ((color.r as u32) << 16) |
                ((color.g as u32) << 8)  |
                (color.b as u32);


            if texture_manager.is_pixel_transparent(texture_key as u32, pixel_u32) {
                continue;
            }

            //  APLICAR ALPHA (transparencia para fade out)
            color.a = (color.a as f32 * item.alpha) as u8;

            framebuffer.set_current_color(color);
            framebuffer.set_pixel(x, y);
        }
    }
}

// Dibujar texto en pantalla
pub fn draw_text_centered(
    framebuffer: &mut Framebuffer,
    text: &str,
    y: i32,
    color: Color,
    scale: usize,
) {
    // Fuente simple de 5x7 píxeles por carácter
    let char_width = 6 * scale;
    let text_width = text.len() * char_width;
    let x_start = (framebuffer.width as i32 - text_width as i32) / 2;

    framebuffer.set_current_color(color);

    for (i, ch) in text.chars().enumerate() {
        let x_offset = x_start + (i * char_width) as i32;
        draw_char_simple(framebuffer, ch, x_offset, y, scale);
    }
}

//  Dibujar un carácter simple
fn draw_char_simple(framebuffer: &mut Framebuffer, ch: char, x: i32, y: i32, scale: usize) {
    // Fuente bitmap simple (5x7 píxeles)
    let patterns: &[u8] = match ch {
        'P' => &[0x7C, 0x12, 0x12, 0x12, 0x0C],
        'r' => &[0x00, 0x7C, 0x08, 0x04, 0x04],
        'e' => &[0x38, 0x54, 0x54, 0x54, 0x18],
        's' => &[0x48, 0x54, 0x54, 0x54, 0x24],
        'i' => &[0x00, 0x44, 0x7D, 0x40, 0x00],
        'o' => &[0x38, 0x44, 0x44, 0x44, 0x38],
        'n' => &[0x7C, 0x08, 0x04, 0x04, 0x78],
        'a' => &[0x20, 0x54, 0x54, 0x54, 0x78],
        '"' => &[0x00, 0x07, 0x00, 0x07, 0x00],
        'D' => &[0x7C, 0x44, 0x44, 0x44, 0x38],
        'p' => &[0xFC, 0x24, 0x24, 0x24, 0x18],
        'c' => &[0x38, 0x44, 0x44, 0x44, 0x28],
        'g' => &[0x18, 0xA4, 0xA4, 0xA4, 0x7C],
        'l' => &[0x00, 0x44, 0x7C, 0x40, 0x00],
        ' ' => &[0x00, 0x00, 0x00, 0x00, 0x00],
        _ => &[0x7C, 0x7C, 0x7C, 0x7C, 0x7C], // Cuadrado para caracteres no soportados
    };

    for (col, &pattern) in patterns.iter().enumerate() {
        for row in 0..7 {
            if (pattern >> row) & 1 == 1 {
                for sx in 0..scale {
                    for sy in 0..scale {
                        framebuffer.set_pixel(
                            x + (col * scale) as i32 + sx as i32,
                            y + (row * scale) as i32 + sy as i32,
                        );
                    }
                }
            }
        }
    }
}

pub fn render_items(
    framebuffer: &mut Framebuffer,
    player: &mut Player,
    items: &mut Vec<Item>,
    texture_manager: &TextureManager,
    delta_time: f32,
    window: &RaylibHandle,
) {
    let pickup_range = 50.0; // Distancia para mostrar el texto
    let mut nearest_item: Option<usize> = None;
    let mut nearest_distance = f32::MAX;

    for (idx, item) in items.iter_mut().enumerate() {
        // ACTUALIZAR ANIMACIÓN
        item.update(delta_time);
        
        if item.state == ItemState::Idle {
            let dx = item.pos.x - player.pos.x;
            let dy = item.pos.y - player.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Encontrar el item más cercano
            if distance < pickup_range && distance < nearest_distance {
                nearest_distance = distance;
                nearest_item = Some(idx);
            }
        }

        draw_item(framebuffer, player, item, texture_manager);
    }

    // MOSTRAR TEXTO Y MANEJAR RECOLECCIÓN
    if let Some(idx) = nearest_item {
        let item = &mut items[idx];
        
        // Dibujar texto "Presiona D para recoger [Item]"
        let text = format!("Presiona \"D\" para recoger {}", item.name);
        draw_text_centered(
            framebuffer,
            &text,
            framebuffer.height - 100,
            Color::new(255, 255, 255, 255),
            2,
        );

        // DETECTAR TECLA D
        if window.is_key_pressed(KeyboardKey::KEY_D) {
            println!("¡Recogiendo {}!", item.name);
            item.start_collecting();
        }
    }
}

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




fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as i32, window_height as i32,Color::BLACK);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    // Load the maze once before the loop
    let maze = load_maze("./maze.txt");
    let mut player = Player{pos:(Vector2::new(180.0,180.0)), a: PI/3.0, fov: PI/2.0 };
    let texture_cache = TextureManager::new(&mut window, &raylib_thread);

    let mut items = vec![
        Item::new(400.0, 350.0, 'b'), // llave azul
        Item::new(700.0, 300.0, 'c'), // moneda
        Item::new(1000.0, 800.0, 'h'), // poción
    ];

    let mut last_time = std::time::Instant::now();

    while !window.window_should_close() {
        let current_time = std::time::Instant::now();
        let delta_time = (current_time - last_time).as_secs_f32();
        last_time = current_time;

        framebuffer.clear();
        process_events(&window, &mut player, &maze);
        // 1. clear framebuffer
        let mut mode = "3D";

        if window.is_key_down(KeyboardKey::KEY_M) {
            mode = if mode =="2D" {"3D"} else {"2D"};
        }
        framebuffer.clear();

        if mode == "2D"{
            render_maze(&mut framebuffer, &maze, block_size,&player);
        }
        else {
            render_world(&mut framebuffer,&player,&maze,&texture_cache);

            render_items(
                &mut framebuffer,
                &mut player,
                &mut items,
                &texture_cache,
                delta_time,
                &window,
            );
            render_minimap(&mut framebuffer, &maze, &player, &items);
            //render_enemies(&mut framebuffer,&player,&texture_cache);
        }

        
            framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
