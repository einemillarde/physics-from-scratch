use glam::Vec3;
use crate::physics_calc::math::{
    Aabb,
    Transform,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn compute_aabb(self, transform: Transform) -> Aabb {
        Aabb {
            min: transform.position - Vec3::splat(self.radius),
            max: transform.position + Vec3::splat(self.radius),
        }
    }
}
