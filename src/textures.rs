use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    images: HashMap<char, Image>,       // Store images for pixel access
    textures: HashMap<char, Texture2D>, // Store GPU textures for rendering
    dimensions: HashMap<char, (u32, u32)>,
}
 
impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();
        let mut dimensions = HashMap::new();

        // Map characters to texture file paths
        let texture_files = vec![
            // ('+', "assets/wall4.png"),
            // ('-', "assets/wall2.png"),
            ('+', "assets/fondoV.png"),
            ('|', "assets/fondo1.png"),
            ('-', "assets/fondo1.png"),

            ('b', "assets/furnace_front_off.png"),
            ('c', "assets/redstone_lamp_on.png"),
            ('h', "assets/furnace_front_off.png"),
            // ('g', "assets/wall5.png"),
            // ('#', "assets/wall3.png"), // default/fallback
        ];

          for (ch, path) in texture_files {
            match Image::load_image(path) {
                Ok(image) => {
                    let width = image.width as u32;
                    let height = image.height as u32;
                    println!("Cargada imagen para '{}': {} ({}x{})", ch, path, width, height);

                    dimensions.insert(ch, (width, height));
                    let texture = rl.load_texture(thread, path).expect(&format!("Failed to load texture {}", path));
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

    // ✨ NUEVA: Obtener alto
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