use {
    glam::Vec3,
    crate::physics_calc::{
        math::{
            Transform,
            Aabb
        },
        collider::{
            Primitive,
            ConvexHull
        }
    }
};

#[derive(Debug, Clone)]
pub enum CompoundPartShape {
    Primitive(Primitive),
    ConvexHull(ConvexHull),
}

#[derive(Debug, Clone)]
pub struct CompoundPart {
    shape: CompoundPartShape,
    local_transform: Transform,
}

#[derive(Debug, Clone)]
pub struct Compound {
    parts: Vec<CompoundPart>,
}

impl Compound {
    pub fn compute_aabb(&self, transform: Transform) -> Aabb {
        if self.parts.is_empty() {
            return Aabb {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            };
        }

        let mut total_min = Vec3::splat(f32::INFINITY);
        let mut total_max = Vec3::splat(f32::NEG_INFINITY);

        for part in &self.parts {
            let world_transform = transform.transform_by(part.local_transform);

            let aabb = match &part.shape {
                CompoundPartShape::Primitive(p) => p.compute_aabb(world_transform),
                CompoundPartShape::ConvexHull(h) => h.compute_aabb(world_transform),
            };

            total_min = total_min.min(aabb.min);
            total_max = total_max.max(aabb.max);
        }

        Aabb {
            min: total_min,
            max: total_max,
        }
    }
}
