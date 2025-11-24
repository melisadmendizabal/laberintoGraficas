//inventory

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::textures::TextureManager;
use crate::draw_text_centered;

pub struct Inventory {
    pub collected_items: Vec<char>, // texture_keys de items recolectados
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            collected_items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, texture_key: char) {
        if !self.collected_items.contains(&texture_key) {
            self.collected_items.push(texture_key);
            println!("🎒 Item {} agregado al inventario ({}/3)", texture_key, self.collected_items.len());
        }
    }

    pub fn has_all_items(&self) -> bool {
        self.collected_items.len() >= 3
    }

    pub fn count(&self) -> usize {
        self.collected_items.len()
    }
}



// Renderizar inventario en la esquina superior izquierda
pub fn render_inventory(
    framebuffer: &mut Framebuffer,
    inventory: &Inventory,
    texture_manager: &TextureManager,
) {
    let padding = 20_i32;  // ✅ Especificar tipo i32
    let icon_size = 50_i32; // ✅
    let spacing = 10_i32;  

    // Fondo del inventario
    framebuffer.set_current_color(Color::new(0, 0, 0, 180));
    let bg_width = (icon_size + spacing) * 3 + padding * 2;
    let bg_height = icon_size + padding * 2;

    for x in 0..bg_width {
        for y in 0..bg_height {
            framebuffer.set_pixel(padding + x, padding + y);
        }
    }

    // Dibujar borde
    framebuffer.set_current_color(Color::new(255, 255, 255, 255));
    for x in 0..bg_width {
        framebuffer.set_pixel(padding + x, padding);
        framebuffer.set_pixel(padding + x, padding + bg_height - 1);
    }
    for y in 0..bg_height {
        framebuffer.set_pixel(padding, padding + y);
        framebuffer.set_pixel(padding + bg_width - 1, padding + y);
    }

    // Dibujar slots de inventario (3 espacios)
    for slot in 0..3_i32 {
        let x_start = padding * 2 + (slot * (icon_size + spacing));
        let y_start = padding * 2;

        // Dibujar slot vacío
        framebuffer.set_current_color(Color::new(50, 50, 50, 255));
        for x in 0..icon_size {
            for y in 0..icon_size {
                framebuffer.set_pixel(x_start + x, y_start + y);
            }
        }

        // Si hay un item en este slot, dibujarlo
        if (slot as usize) < inventory.collected_items.len() {
            let texture_key = inventory.collected_items[slot as usize];
            let (tex_width, tex_height) = texture_manager.get_dimensions(texture_key);

            for x in 0..icon_size {
                for y in 0..icon_size {
                    let tx = (x as f32 / icon_size as f32 * tex_width as f32) as u32;
                    let ty = (y as f32 / icon_size as f32 * tex_height as f32) as u32;

                    let color = texture_manager.get_pixel_color(texture_key, tx, ty);
                    
                    // Skip transparentes
                    let pixel_u32 = ((color.a as u32) << 24) | ((color.r as u32) << 16) 
                                  | ((color.g as u32) << 8) | (color.b as u32);
                    if texture_manager.is_pixel_transparent(texture_key as u32, pixel_u32) {
                        continue;
                    }

                    framebuffer.set_current_color(color);
                    framebuffer.set_pixel(x_start + x, y_start + y);
                }
            }
        }
    }

    // Mostrar contador
    let count_text = format!("{}/3", inventory.count());
    draw_text_centered(
        framebuffer,
        &count_text,
        padding + bg_height + 10,
        Color::new(255, 255, 255, 255),
        2,
    );
}

