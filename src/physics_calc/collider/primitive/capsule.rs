use glam::Vec3;
use crate::physics_calc::math::{
    Aabb,
    Transform,
};

#[derive(Debug, Clone, Copy)]
pub struct Capsule {
    pub half_height: f32,
    pub radius: f32
}

impl Capsule {
    pub fn compute_aabb(self, transform: Transform) -> Aabb {
        let a = transform.transform_point(Vec3::new(0.0, -self.half_height, 0.0));
        let b = transform.transform_point(Vec3::new(0.0,  self.half_height, 0.0));

        let radius = Vec3::splat(self.radius);

        let min = a.min(b) - radius;
        let max = a.max(b) + radius;

        Aabb { min, max }
    }
}

