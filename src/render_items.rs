//render_item.rs
use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::item::{Item, ItemState};
use crate::textures::TextureManager;
use crate::draw_text_centered; 
use crate::Inventory;


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
    
    let half_size_x = size_x / 5.0;
    let half_size_y = size_y / 5.0;


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
    let (texture_width, texture_height) = texture_manager.get_dimensions(texture_key);
    let texture_width = texture_width as f32;
    let texture_height = texture_height as f32;

    for x in x_start..x_end {
        for y in top as i32..bottom as i32 {
            let tx = ((x - x_start) as f32 / xsize as f32) * texture_width;
            let ty = ((y as f32 - top) / (bottom - top)) * texture_height;

            let mut color = texture_manager.get_pixel_color(texture_key, tx as u32, ty as u32);

            let pixel_u32 =
                ((color.a as u32) << 24) |
                ((color.r as u32) << 16) |
                ((color.g as u32) << 8)  |
                (color.b as u32);

            if texture_manager.is_pixel_transparent(texture_key as u32, pixel_u32) {
                continue;
            }

            color.a = (color.a as f32 * item.alpha) as u8;

            framebuffer.set_current_color(color);
            framebuffer.set_pixel_with_depth(x, y, distance);
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
    inventory: &mut Inventory,
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
            inventory.add_item(item.texture_key);
            item.start_collecting();
        }
    }
}