pub mod sphere;
pub mod cuboid;
pub mod capsule;
pub mod cylinder;

pub use sphere::Sphere;
pub use cuboid::Cuboid;
pub use capsule::Capsule;
pub use cylinder::Cylinder;

use crate::physics_calc::math::{
    Aabb,
    Transform
};

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Sphere(Sphere),
    Cuboid(Cuboid),
    Cylinder(Cylinder),
    Capsule(Capsule)
}

impl Primitive {
    pub fn compute_aabb(self, transform: Transform) -> Aabb {
        match self {
            Primitive::Sphere(sphere) => sphere.compute_aabb(transform),
            Primitive::Cuboid(cuboid) => cuboid.compute_aabb(transform),
            Primitive::Cylinder(cylinder) => cylinder.compute_aabb(transform),
            Primitive::Capsule(capsule) => capsule.compute_aabb(transform),
        }
    }
}
