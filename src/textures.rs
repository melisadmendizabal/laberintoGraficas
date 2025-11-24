//texture.rs

use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;
use crate::menu::ItemConfig;

pub struct TextureManager {
    images: HashMap<char, Image>,
    textures: HashMap<char, Texture2D>,
    dimensions: HashMap<char, (u32, u32)>,
}
 
impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();
        let mut dimensions = HashMap::new();

        let texture_files = vec![
            ('+', "assets/fondoV.png"),
            ('|', "assets/fondo1.png"),
            ('-', "assets/fondo1.png"),
        ];

        for (ch, path) in texture_files {
            match Image::load_image(path) {
                Ok(image) => {
                    let width = image.width as u32;
                    let height = image.height as u32;
                    println!("Cargada imagen para '{}': {} ({}x{})", ch, path, width, height);

                    dimensions.insert(ch, (width, height));
                    let texture = rl.load_texture(thread, path)
                        .expect(&format!("Failed to load texture {}", path));
                    images.insert(ch, image);
                    textures.insert(ch, texture);
                }
                Err(e) => {
                    eprintln!("Error al cargar imagen para '{}': {}. Error: {}", ch, path, e);
                }
            }
        }

        TextureManager { images, textures, dimensions }
    }

    pub fn load_map_textures(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        broken_path: &str,
        fixed_path: &str,
    ) {
        // Cargar textura rota ('*')
        match Image::load_image(broken_path) {
            Ok(image) => {
                let width = image.width as u32;
                let height = image.height as u32;
                println!("✨ Cargada textura mapa ROTO '*': {} ({}x{})", broken_path, width, height);

                self.dimensions.insert('*', (width, height));
                if let Ok(texture) = rl.load_texture(thread, broken_path) {
                    self.images.insert('*', image);
                    self.textures.insert('*', texture);
                }
            }
            Err(e) => {
                eprintln!("❌ Error cargando mapa roto: {}", e);
            }
        }

        // Cargar textura completa ('$')
        match Image::load_image(fixed_path) {
            Ok(image) => {
                let width = image.width as u32;
                let height = image.height as u32;
                println!("✨ Cargada textura mapa COMPLETO '$': {} ({}x{})", fixed_path, width, height);

                self.dimensions.insert('$', (width, height));
                if let Ok(texture) = rl.load_texture(thread, fixed_path) {
                    self.images.insert('$', image);
                    self.textures.insert('$', texture);
                }
            }
            Err(e) => {
                eprintln!("❌ Error cargando mapa completo: {}", e);
            }
        }
    }

    pub fn load_item_textures(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        item_configs: &[ItemConfig],
    ) {
        for config in item_configs {
            let ch = config.texture_key;
            let path = &config.texture_path;

            if self.images.contains_key(&ch) {
                println!("⚠️ Textura '{}' ya cargada, saltando...", ch);
                continue;
            }

            match Image::load_image(path) {
                Ok(image) => {
                    let width = image.width as u32;
                    let height = image.height as u32;
                    println!("✨ Cargada textura item '{}': {} ({}x{})", ch, path, width, height);

                    self.dimensions.insert(ch, (width, height));
                    
                    match rl.load_texture(thread, path) {
                        Ok(texture) => {
                            self.images.insert(ch, image);
                            self.textures.insert(ch, texture);
                        }
                        Err(e) => {
                            eprintln!("❌ Error cargando textura GPU '{}': {}", ch, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error cargando imagen item '{}' ({}): {}", ch, path, e);
                }
            }
        }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            get_pixel_color(image, x, y)
        } else {
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }

    pub fn get_dimensions(&self, ch: char) -> (u32, u32) {
        self.dimensions.get(&ch).copied().unwrap_or((128, 128))
    }

    pub fn get_width(&self, ch: char) -> u32 {
        self.get_dimensions(ch).0
    }

    pub fn get_height(&self, ch: char) -> u32 {
        self.get_dimensions(ch).1
    }

    pub fn is_pixel_transparent(&self, _texture_key: u32, color: u32) -> bool {
        let alpha = (color >> 24) & 0xFF;
        alpha < 10
    }
}

fn get_pixel_color(image: &Image, x: i32, y: i32) -> Color {
    let width = image.width as usize;
    let height = image.height as usize;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return Color::WHITE;
    }

    let x = x as usize;
    let y = y as usize;

    let data_len = width * height * 4;

    unsafe {
        let data = slice::from_raw_parts(image.data as *const u8, data_len);
        let idx = (y * width + x) * 4;

        if idx + 3 >= data_len {
            return Color::WHITE;
        }

        Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }
}