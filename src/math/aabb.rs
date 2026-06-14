use glam::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}
