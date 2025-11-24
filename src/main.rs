#![allow(unused_imports)]
#![allow(dead_code)]

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod item;
mod menu;
mod render_items;  
mod inventory;      
mod render_menu;    
mod render_minimap;
mod special_wall;
use special_wall::SpecialWall;

use item::Item;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use framebuffer::Framebuffer;
use maze::{Maze,load_maze};
use player::Player;
use std::f32::consts::PI;
use textures::TextureManager;
use crate::item::ItemState;
use crate::Vector2;
use crate::maze::find_char_position;
use menu::{Menu, GameState};
use crate::maze::spawn_items_from_config;


use inventory::{Inventory, render_inventory};
use render_items::{render_items, draw_item};
use render_menu::render_menu;
use render_minimap::render_minimap;
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
    player: &Player,
    special_wall: Option<&SpecialWall>,
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
        cast_ray(framebuffer, maze, player, a,block_size,true,special_wall );
    }
}

pub fn render_world(framebuffer:&mut Framebuffer, player: &Player,maze:&Maze, 
    texture_cache: &TextureManager,
    special_wall: Option<&SpecialWall>,
){
    // let maze = load_maze("./maze.txt");
    let block_size = 100;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32/2.0;
    let hh = framebuffer.height as f32/2.0;

    framebuffer.set_current_color(Color::WHITE);
    
    for i in 0..num_rays{
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov/2.0) + (player.fov*current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false, special_wall);
        let d = intersect.distance;
        let c = intersect.impact;

        let angle_diff = a-player.a;
        let mut distance_to_wall = intersect.distance*angle_diff.cos();
        if distance_to_wall < 0.1 {
            distance_to_wall = 0.2;
            // continue; 
        }
        //altura de la pared
        let stake_height = (hh/distance_to_wall) * 150.0;
        

        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

        // DIBUJAR TECHO (desde arriba hasta donde empieza la pared)
        for y in 0..stake_top {
            let gradient = 1.0 -(y as f32 / stake_bottom as f32) ;
            let r = (24.0 + gradient * 50.0) as u8;
            let g = (98.0 + gradient * 20.0) as u8;
            let b = (98.0 + gradient * 20.0) as u8;
            framebuffer.set_current_color(Color::new(r, g, b, 255));
            framebuffer.set_pixel(i, y as i32);
        }

        //dibujar pared
        let (texture_width, texture_height) = texture_cache.get_dimensions(c);
        let texture_width = texture_width as f32;
        let texture_height = texture_height as f32;
        

        for y in stake_top..stake_bottom {
            let tx = (intersect.tx * texture_width) as u32;
            let ty = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) 
                     * texture_height) as u32;

            let tx = tx.min(texture_width as u32 - 1);
            let ty = ty.min(texture_height as u32 - 1);

            let mut color = texture_cache.get_pixel_color(c, tx, ty);
            

            let max_visibility = 800.0;  // Distancia máxima de visibilidad
            let fog_intensity = (distance_to_wall / max_visibility).min(1.0);
            let fog_color_r = 25.0;  // Color de la niebla (gris muy oscuro)
            let fog_color_g = 15.0;
            let fog_color_b = 25.0;
            
            color.r = (color.r as f32 * (1.0 - fog_intensity) + fog_color_r * fog_intensity) as u8;
            color.g = (color.g as f32 * (1.0 - fog_intensity) + fog_color_g * fog_intensity) as u8;
            color.b = (color.b as f32 * (1.0 - fog_intensity) + fog_color_b * fog_intensity) as u8;
            
            framebuffer.set_current_color(color);
            framebuffer.set_pixel_with_depth(i, y as i32, distance_to_wall);
        }

        // DIBUJAR PISO (desde donde termina la pared hasta abajo)
        let floor_start = stake_bottom;
        let floor_end = framebuffer.height as usize;
        for y in floor_start..floor_end {
            let gradient = (floor_end - y ) as f32 / (floor_end - floor_start) as f32;
            let r = (176.0 - gradient * 30.0) as u8;
            let g = (151.0 - gradient * 30.0) as u8;
            let b = (95.0 - gradient * 50.0) as u8;
            framebuffer.set_current_color(Color::new(r, g, b, 255));
            framebuffer.set_pixel(i, y as i32);
        }

    }
}




//Detectar si el jugador está cerca del mapa
pub fn check_map_proximity(player: &Player, maze: &Maze, block_size: usize) -> Option<bool> {
    let check_range = 100.0;

    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == '*' || cell == '$' {
                let map_x = (col_idx * block_size) as f32 + (block_size as f32 / 2.0);
                let map_y = (row_idx * block_size) as f32 + (block_size as f32 / 2.0);

                let dx = player.pos.x - map_x;
                let dy = player.pos.y - map_y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < check_range {
                    return Some(cell == '*');
                }
            }
        }
    }
    None
}

//Cambiar el mapa de roto a completo
pub fn fix_map_in_maze(maze: &mut Maze) {
    for row in maze.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == '*' {
                *cell = '$'; // Cambiar a mapa completo
                println!("✨ ¡Mapa restaurado!");
                return;
            }
        }
    }
}

//  Pantalla de nivel completado
pub fn render_victory_screen(framebuffer: &mut Framebuffer) {
    // Fondo oscuro
    framebuffer.set_current_color(Color::new(0, 0, 0, 200));
    for x in 0..framebuffer.width {
        for y in 0..framebuffer.height {
            framebuffer.set_pixel(x, y);
        }
    }

    // Título de victoria
    let title = "NIVEL COMPLETADO";
    draw_text_large(framebuffer, title, framebuffer.height / 3, Color::new(255, 215, 0, 255));

    // Instrucciones
    let instruction = "Presiona ENTER para volver al menu";
    draw_text_centered(
        framebuffer,
        instruction,
        framebuffer.height / 2 + 100,
        Color::new(200, 200, 200, 255),
        2,
    );
}


// Dibujar texto grande
fn draw_text_large(framebuffer: &mut Framebuffer, text: &str, y: i32, color: Color) {
    let scale = 4;
    let char_width = 6 * scale;
    let text_width = text.len() * char_width;
    let x_start = (framebuffer.width as i32 - text_width as i32) / 2;

    framebuffer.set_current_color(color);

    for (i, ch) in text.chars().enumerate() {
        let x_offset = x_start + (i * char_width) as i32;
        draw_char_simple(framebuffer, ch, x_offset, y, scale);
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
        'A' => &[0x7C, 0x12, 0x11, 0x12, 0x7C],
        'B' => &[0x7F, 0x49, 0x49, 0x49, 0x36],
        'C' => &[0x3E, 0x41, 0x41, 0x41, 0x22],
        'D' => &[0x7F, 0x41, 0x41, 0x41, 0x3E],
        'E' => &[0x7F, 0x49, 0x49, 0x49, 0x41],
        'F' => &[0x7F, 0x09, 0x09, 0x09, 0x01],
        'G' => &[0x3E, 0x41, 0x49, 0x49, 0x7A],
        'H' => &[0x7F, 0x08, 0x08, 0x08, 0x7F],
        'I' => &[0x00, 0x41, 0x7F, 0x41, 0x00],
        'J' => &[0x20, 0x40, 0x41, 0x3F, 0x01],
        'K' => &[0x7F, 0x08, 0x14, 0x22, 0x41],
        'L' => &[0x7F, 0x40, 0x40, 0x40, 0x40],
        'M' => &[0x7F, 0x02, 0x0C, 0x02, 0x7F],
        'N' => &[0x7F, 0x04, 0x08, 0x10, 0x7F],
        'O' => &[0x3E, 0x41, 0x41, 0x41, 0x3E],
        'P' => &[0x7F, 0x09, 0x09, 0x09, 0x06],
        'Q' => &[0x3E, 0x41, 0x51, 0x21, 0x5E],
        'R' => &[0x7F, 0x09, 0x19, 0x29, 0x46],
        'S' => &[0x46, 0x49, 0x49, 0x49, 0x31],
        'T' => &[0x01, 0x01, 0x7F, 0x01, 0x01],
        'U' => &[0x3F, 0x40, 0x40, 0x40, 0x3F],
        'V' => &[0x1F, 0x20, 0x40, 0x20, 0x1F],
        'W' => &[0x3F, 0x40, 0x38, 0x40, 0x3F],
        'X' => &[0x63, 0x14, 0x08, 0x14, 0x63],
        'Y' => &[0x07, 0x08, 0x70, 0x08, 0x07],
        'Z' => &[0x61, 0x51, 0x49, 0x45, 0x43],
        
        // Letras minúsculas
        'a' => &[0x20, 0x54, 0x54, 0x54, 0x78],
        'b' => &[0x7F, 0x48, 0x44, 0x44, 0x38],
        'c' => &[0x38, 0x44, 0x44, 0x44, 0x20],
        'd' => &[0x38, 0x44, 0x44, 0x48, 0x7F],
        'e' => &[0x38, 0x54, 0x54, 0x54, 0x18],
        'f' => &[0x08, 0x7E, 0x09, 0x01, 0x02],
        'g' => &[0x18, 0xA4, 0xA4, 0xA4, 0x7C],
        'h' => &[0x7F, 0x08, 0x04, 0x04, 0x78],
        'i' => &[0x00, 0x44, 0x7D, 0x40, 0x00],
        'j' => &[0x40, 0x80, 0x84, 0x7D, 0x00],
        'k' => &[0x7F, 0x10, 0x28, 0x44, 0x00],
        'l' => &[0x00, 0x41, 0x7F, 0x40, 0x00],
        'm' => &[0x7C, 0x04, 0x18, 0x04, 0x78],
        'n' => &[0x7C, 0x08, 0x04, 0x04, 0x78],
        'o' => &[0x38, 0x44, 0x44, 0x44, 0x38],
        'p' => &[0xFC, 0x24, 0x24, 0x24, 0x18],
        'q' => &[0x18, 0x24, 0x24, 0x18, 0xFC],
        'r' => &[0x7C, 0x08, 0x04, 0x04, 0x08],
        's' => &[0x48, 0x54, 0x54, 0x54, 0x20],
        't' => &[0x04, 0x3F, 0x44, 0x40, 0x20],
        'u' => &[0x3C, 0x40, 0x40, 0x20, 0x7C],
        'v' => &[0x1C, 0x20, 0x40, 0x20, 0x1C],
        'w' => &[0x3C, 0x40, 0x30, 0x40, 0x3C],
        'x' => &[0x44, 0x28, 0x10, 0x28, 0x44],
        'y' => &[0x1C, 0xA0, 0xA0, 0xA0, 0x7C],
        'z' => &[0x44, 0x64, 0x54, 0x4C, 0x44],
        ' ' => &[0x00, 0x00, 0x00, 0x00, 0x00],
        ':' => &[0x00, 0x36, 0x36, 0x00, 0x00],
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




fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Laberinto 3D")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as i32, window_height as i32, Color::BLACK);
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    // ESTADO DEL JUEGO
    let mut game_state = GameState::Menu;
    let mut menu = Menu::new();
    
    
    // Variables del juego (se inicializarán cuando se seleccione un mapa)
    let mut maze: Option<Maze> = None;
    let mut player: Option<Player> = None;
    let mut items: Vec<Item> = Vec::new();
    let mut inventory = Inventory::new();
    let mut special_wall: Option<SpecialWall> = None;
    
    let mut map_fixed = false;
    let mut texture_cache = TextureManager::new(&mut window, &raylib_thread);

    let mut last_time = std::time::Instant::now();

    while !window.window_should_close() {
        let current_time = std::time::Instant::now();
        let delta_time = (current_time - last_time).as_secs_f32();
        last_time = current_time;

        match game_state {
            GameState::Menu => {
                //RENDERIZAR MENÚ
                render_menu(&mut framebuffer, &menu);

                //MANEJAR INPUT DEL MENÚ
                if let Some(selected_index) = menu.handle_input(&window) {
                    let selected_map = &menu.options[selected_index];
                    println!("🎮 Cargando mapa: {}", selected_map.file_path);

                    // Cargar el mapa seleccionado
                    let loaded_maze = load_maze(&selected_map.file_path);


                    // CARGAR TEXTURAS DINÁMICAMENTE PARA ESTE MAPA
                    texture_cache.load_map_textures(
                        &mut window,
                        &raylib_thread,
                        &selected_map.broken_map_texture,
                        &selected_map.fixed_map_texture,
                       
                    );

                    //Cargar texturas de items (solo item_configs)
                    texture_cache.load_item_textures(
                        &mut window,
                        &raylib_thread,
                        &selected_map.item_configs
                    );

                    // BUSCAR LA PARED ESPECIAL
                    if let Some(wall_pos) = find_char_position(&loaded_maze, '*', block_size) {
                        special_wall = Some(SpecialWall::new(wall_pos.x, wall_pos.y));
                        println!("🧱 Pared especial encontrada en: ({}, {})", wall_pos.x, wall_pos.y);
                    }

                    //  SPAWN AUTOMÁTICO DE ITEMS CON CONFIGURACIÓN
                    items = spawn_items_from_config(&loaded_maze, block_size, &selected_map.item_configs);

                    // Encontrar posición inicial del jugador
                    let spawn_position = find_char_position(&loaded_maze, 'i', block_size)
                        .unwrap_or(Vector2::new(180.0, 180.0));

                    println!("🎮 Spawneando jugador en: ({}, {})", spawn_position.x, spawn_position.y);
                    println!("✨ Total de items spawneados: {}", items.len());

                    // Inicializar jugador
                    player = Some(Player {
                        pos: spawn_position,
                        a: PI / 3.0,
                        fov: PI / 2.0,
                    });

                    maze = Some(loaded_maze);
                    inventory = Inventory::new(); //Reiniciar inventario
                    
                    game_state = GameState::Playing;
                }
            }

            GameState::Playing => {
                if let (Some(maze_data), Some(player_data)) = (&mut maze, &mut player) {
                    framebuffer.clear();
                    process_events(&window, player_data, maze_data);

                    // Cambiar a modo 2D/3D
                    let mut mode = "3D";
                    if window.is_key_down(KeyboardKey::KEY_M) {
                        mode = if mode == "2D" { "3D" } else { "2D" };
                    }

                    // Volver al menú con ESC
                    if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                        game_state = GameState::Menu;
                        continue;
                    }

                    // ✅ NUEVA LÓGICA CON SpecialWall
                    if let Some(wall) = &mut special_wall {
                        
                        if wall.update(delta_time) {
                            game_state = GameState::Victory;
                            continue;
                        }

                        if wall.check_proximity(player_data.pos, 100.0) {
                        let fb_height = framebuffer.height;
                        
                        if !wall.is_unlocked {
                            if inventory.has_all_items() {
                                // ✅ Tiene todas las piezas
                                let text = "Presiona F para restaurar el mapa";
                                draw_text_centered(
                                    &mut framebuffer,
                                    text,
                                    fb_height - 150,
                                    Color::new(255, 215, 0, 255),
                                    2,
                                );

                                if window.is_key_pressed(KeyboardKey::KEY_F) {
                                    wall.unlock();
                                    
                                }
                            } else {
                                // ❌ Faltan piezas
                                let collected = inventory.count();
                                let text = format!("Aun faltan piezas ({}/3)", collected);
                                draw_text_centered(
                                    &mut framebuffer,
                                    &text,
                                    fb_height - 150,
                                    Color::new(255, 100, 100, 255),
                                    2,
                                );
                            }
                        } else {
                            // MOSTRAR MENSAJE MIENTRAS CORRE EL TEMPORIZADOR
                            let text = format!("¡Mapa restaurado! ({:.1}s)", wall.unlock_timer);
                            draw_text_centered(
                                &mut framebuffer,
                                &text,
                                fb_height - 150,
                                Color::new(0, 255, 0, 255),  // Verde brillante
                                3,
                            );
                        }

                    }
                }
                 
                

                   

                    if mode == "2D" {
                        render_maze(&mut framebuffer, maze_data, block_size, player_data,special_wall.as_ref());
                    } else {
                        render_world(&mut framebuffer, player_data, maze_data, &texture_cache, special_wall.as_ref());

                        render_items(
                            &mut framebuffer,
                            player_data,
                            &mut items,
                            &texture_cache,
                            delta_time,
                            &window,
                            &mut inventory,
                        );
        
                        render_minimap(&mut framebuffer, maze_data, player_data, &items);
                        render_inventory(&mut framebuffer, &inventory, &texture_cache);
                    }
                }
            }

            GameState::Victory => {
                render_victory_screen(&mut framebuffer);

                // Volver al menú con ENTER
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    game_state = GameState::Menu;
                }
            }
            GameState::Paused => {
                // Por si quieres implementar pausa más adelante
            }
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
    }
}