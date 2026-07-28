use crate::asset::{MaterialHandle, vertex::Vertex};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub material: MaterialHandle,
}
