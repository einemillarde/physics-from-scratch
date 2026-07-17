use crate::asset::{MaterialHandle, vertex::Vertex};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub material: MaterialHandle,
}
