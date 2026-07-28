pub mod directional;
pub mod point;

use {directional::DirectionalLight, point::PointLight};

pub enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
}

impl From<DirectionalLight> for Light {
    fn from(light: DirectionalLight) -> Self {
        Light::Directional(light)
    }
}

impl From<PointLight> for Light {
    fn from(light: PointLight) -> Self {
        Light::Point(light)
    }
}
