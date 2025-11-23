use raylib::math::Vector2;

#[derive(Clone, Copy, PartialEq)]
pub enum ItemState {
    Idle,
    Fading,
}

pub struct Item {
    pub pos: Vector2,
    pub texture_key: char,
    pub collected: bool,
    pub bob_timer: f32,
    pub rotation_timer: f32,
    pub alpha: f32,
    pub state: ItemState,
    pub name: String, // Para mostrar en texto
}

impl Item {
    pub fn new(x: f32, y: f32, texture_key: char) -> Self {
        let name = match texture_key {
            'b' => "Llave Azul",
            'c' => "Moneda",
            'h' => "Poción",
            _ => "Item",
        };

        Self {
            pos: Vector2::new(x, y),
            texture_key,
            collected: false,
            bob_timer: 0.0,
            rotation_timer: 0.0,
            alpha: 1.0,
            state: ItemState::Idle,
            name: name.to_string(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        match self.state {
            ItemState::Idle => {
                // Animación de flotación (bobbing)
                self.bob_timer += delta_time * 3.0;
                
                // Animación de rotación
                self.rotation_timer += delta_time * 2.0;
            }
            ItemState::Fading => {
                // Animación de desvanecimiento
                self.alpha -= delta_time * 3.0; // Velocidad del fade
                if self.alpha <= 0.0 {
                    self.alpha = 0.0;
                    self.collected = true;
                }
            }
        }
    }

    pub fn get_bob_offset(&self) -> f32 {
        // Flotación de -20 a 20 píxeles
        self.bob_timer.sin() * 20.0
    }

    pub fn start_collecting(&mut self) {
        self.state = ItemState::Fading;
    }

    pub fn is_near_player(&self, player_pos: Vector2, pickup_range: f32) -> bool {
        let dx = self.pos.x - player_pos.x;
        let dy = self.pos.y - player_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < pickup_range
    }
}

