use {
    glam::Vec3,
    crate::physics_calc::math::{
        Transform,
        Aabb
    }
};

#[derive(Debug, Clone)]
pub struct ConvexHull {
    pub vertices: Vec<Vec3>
}

impl ConvexHull {
    pub fn compute_aabb(&self, transform: Transform) -> Aabb {
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        for v in self.vertices.iter() {
            let world = transform.transform_point(*v);
            min = min.min(world);
            max = max.max(world);
        }
        Aabb { min, max }
    }
}
