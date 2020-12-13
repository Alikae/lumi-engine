use luminance_derive::{Semantics, Vertex};

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "texpos", repr = "[f32; 2]", wrapper = "VertexTexpos")]
    Texpos,
}

#[derive(Vertex, Copy, Clone)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    #[allow(dead_code)]
    pub position: VertexPosition,

    #[allow(dead_code)]
    pub texpos: VertexTexpos,
//    #[vertex(normalized = "true")]
}

pub const VERTICES: [Vertex; 4] = [
    Vertex::new(
        VertexPosition::new([-0.5, -0.5]),
        VertexTexpos::new([0., 0.]),
    ),
    Vertex::new(
        VertexPosition::new([0.5, -0.5]),
        VertexTexpos::new([1., 0.]),
    ),
    Vertex::new(
        VertexPosition::new([0.5, 0.5]),
        VertexTexpos::new([1., 1.]),
    ),
    Vertex::new(
        VertexPosition::new([-0.5, 0.5]),
        VertexTexpos::new([0., 1.]),
    ),
];
