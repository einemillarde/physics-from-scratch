use glam::Vec3;
use crate::physics_calc::math::{
    Aabb,
    Transform,
};

#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    pub half_extents: Vec3,
}

impl Cuboid {
    pub fn compute_aabb(self, transform: Transform) -> Aabb {
        let corners = [
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new( 1.0, -1.0, -1.0),
            Vec3::new(-1.0,  1.0, -1.0),
            Vec3::new( 1.0,  1.0, -1.0),
            Vec3::new(-1.0, -1.0,  1.0),
            Vec3::new( 1.0, -1.0,  1.0),
            Vec3::new(-1.0,  1.0,  1.0),
            Vec3::new( 1.0,  1.0,  1.0),
        ];

        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for corner in corners {
            let local = corner * self.half_extents;
            let world = transform.transform_point(local);

            min = min.min(world);
            max = max.max(world);
        }

        Aabb { min, max }
    }
}
