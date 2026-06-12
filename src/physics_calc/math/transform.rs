use glam::{Quat, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn transform_point(self, point: Vec3) -> Vec3 {
        self.position + self.rotation * (self.scale * point)
    }

    pub fn transform_by(self, transform: Transform) -> Transform {
        let mut t2 = self;
        t2.position *= transform.position;
        t2.scale *= transform.scale;
        t2.rotation = t2.rotation.mul_quat(transform.rotation);
        t2
    }

    pub fn from_pos_rot_scale(self, position: Vec3, rotation: Vec3, scale: Vec3) -> Transform {
        Transform { position, rotation, scale }
    }
}
