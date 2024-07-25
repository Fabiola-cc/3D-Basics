use nalgebra_glm::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub angle: f32, // Ángulo en radianes
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Player {
            pos: Vec2::new(x, y),
            angle,
        }
    }
}
