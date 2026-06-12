use glam::Vec3;
use crate::physics_calc::math::{
    Aabb,
    Transform,
};

#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pub half_height: f32,
    pub radius: f32
}

impl Cylinder {
    pub fn compute_aabb(self, transform: Transform) -> Aabb {
        let center = transform.position;
        let axis = transform.rotation * Vec3::Y;
        let abs_axis = axis.abs();
        let radial = Vec3::ONE - abs_axis * abs_axis;
        let radial_extent = Vec3::new(
            radial.x.sqrt(),
            radial.y.sqrt(),
            radial.z.sqrt(),
        ) * self.radius;
        let axial_extent = abs_axis * self.half_height;
        let extent = radial_extent + axial_extent;
        Aabb {
            min: center - extent,
            max: center + extent,
        }
    }
}
