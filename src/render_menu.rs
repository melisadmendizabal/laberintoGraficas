use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::menu::Menu;
use crate::{draw_text_centered, draw_text_large};

pub fn render_menu(framebuffer: &mut Framebuffer, menu: &Menu) {
    framebuffer.clear();
    
    // Fondo del menú
    framebuffer.set_current_color(Color::new(20, 20, 40, 255));
    for x in 0..framebuffer.width {
        for y in 0..framebuffer.height {
            framebuffer.set_pixel(x, y);
        }
    }

    // Título
    let title = "La Torpeza del Aprendiz";
    draw_text_large(framebuffer, title, framebuffer.height / 6, Color::new(255, 255, 255, 255));

    // Subtítulo
    let subtitle = "Selecciona un Mapa";
    draw_text_centered(framebuffer, subtitle, framebuffer.height / 3, Color::new(200, 200, 200, 255), 2);

    // Opciones
    let start_y = framebuffer.height / 2;
    let spacing = 80;

    for (idx, option) in menu.options.iter().enumerate() {
        let y = start_y + (idx as i32 * spacing);
        let color = if idx == menu.selected_option {
            Color::new(255, 215, 0, 255) // Dorado para seleccionado
        } else {
            Color::new(150, 150, 150, 255) // Gris para no seleccionado
        };

        // Indicador de selección
        if idx == menu.selected_option {
            let indicator = ">";
            draw_text_centered(framebuffer, indicator, y - 10, color, 3);
        }

        draw_text_centered(framebuffer, &option.display_name, y + 20, color, 2);
    }

    // Instrucciones
    let instructions = "Flechas: Navegar  |  ENTER: Seleccionar";
    draw_text_centered(
        framebuffer,
        instructions,
        framebuffer.height - 100,
        Color::new(100, 100, 100, 255),
        1,
    );
}
