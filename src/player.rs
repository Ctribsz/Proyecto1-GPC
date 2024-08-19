use nalgebra_glm::Vec2;


pub struct Player {
    pub pos: Vec2,
    pub a: f32, // ángulo de visión
    pub fov: f32,
}

impl Player {
    pub fn new(pos: Vec2, angle: f32, fov: f32) -> Self {
        Self { pos, a: angle, fov }
    }
}