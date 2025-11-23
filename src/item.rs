use raylib::math::Vector2;

pub struct Item {
pub pos: Vector2,
pub texture_key: char,
pub collected: bool,
}

impl Item {
    pub fn new(x: f32, y: f32, texture_key: char) -> Self {
        Self {
        pos: Vector2::new(x, y),
        texture_key,
        collected: false,
        }
    }
}
