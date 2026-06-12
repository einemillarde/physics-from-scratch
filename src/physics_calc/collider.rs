pub mod primitive;
pub mod compound;
pub mod convex_hull;
pub mod static_mesh;

pub use primitive::Primitive;
pub use convex_hull::ConvexHull;
pub use static_mesh::StaticMesh;
pub use compound::Compound;

#[derive(Debug, Clone)]
pub enum Collider {
    Primitive(Primitive),
    ConvexHull(ConvexHull),
    StaticMesh(StaticMesh),
    Compound(Compound)
}

use crate::physics_calc::math::{
    Aabb,
    Transform
};

impl Collider {
    pub fn compute_aabb(&self, transform: Transform) -> Aabb {
        match self {
            Collider::Primitive(primitive) => primitive.compute_aabb(transform),
            Collider::ConvexHull(convex_hull) => convex_hull.compute_aabb(transform),
            Collider::StaticMesh(static_mesh) => static_mesh.compute_aabb(transform),
            Collider::Compound(compound) => compound.compute_aabb(transform)
        }
    }
}

