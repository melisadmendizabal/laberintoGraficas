// special_wall.rs
use raylib::math::Vector2;

pub struct SpecialWall {
    pub pos: Vector2,
    pub is_unlocked: bool,
    pub unlock_timer: f32,
}

impl SpecialWall {
    pub fn new(x: f32, y: f32) -> Self {
        SpecialWall {
            pos: Vector2::new(x, y),
            is_unlocked: false,
            unlock_timer: 0.0,
        }
    }

    pub fn check_proximity(&self, player_pos: Vector2, range: f32) -> bool {
        let dx = self.pos.x - player_pos.x;
        let dy = self.pos.y - player_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < range
    }

    pub fn unlock(&mut self) {
        self.is_unlocked = true;
        self.unlock_timer = 3.0;
        println!("✨ ¡Pared especial desbloqueada!");
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        if self.is_unlocked && self.unlock_timer > 0.0 {
            self.unlock_timer -= delta_time;
            if self.unlock_timer <= 0.0 {
                self.unlock_timer = 0.0;
                return true;  // El tiempo terminó
            }
        }
        false
    }

    /// Esta es la clave: devuelve qué textura usar
    pub fn get_texture_key(&self) -> char {
        if self.is_unlocked {
            '$' // Textura del mapa completo
        } else {
            '*' // Textura del mapa roto
        }
    }
}