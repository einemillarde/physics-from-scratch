use glam::Vec3;

pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }
}
