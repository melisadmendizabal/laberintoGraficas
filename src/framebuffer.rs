use raylib::prelude::*;
use raylib::color::Color;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image, //guarda los pixeles
    background_color: Color, //color de fondo para limpiar
    current_color: Color, //color actual
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) { //limpia su buffer de colores
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color); 
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {//poner un pixel que no se salga de la pantalla
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color){ //settear el color de fondo
        self.background_color= color;
    }

    pub fn set_current_color(&mut self, color: Color) {//setear el color
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str){
        Image::export_image(&self.color_buffer, file_path);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread,){
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer){
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture,0,0,Color::WHITE);
        }
    }
    
}