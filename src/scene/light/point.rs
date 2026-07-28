use glam::Vec3;

pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub range: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
            range: (intensity / 0.1).sqrt(),
        }
    }
}
